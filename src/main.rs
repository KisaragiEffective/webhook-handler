#![warn(clippy::pedantic, clippy::nursery)]
#![deny(type_alias_bounds, legacy_derive_helpers, late_bound_lifetime_arguments)]
mod payload;
mod call;
mod config;

use std::any::Any;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;
use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock};
use actix_web::{App, error, guard, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::web::{Json, JsonConfig, Query};
use qstring::QString;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use anyhow::{Result, bail, anyhow, Error};
use rustls::server::NoClientAuth;
use rustls::{Certificate, PrivateKey, ServerConfig, SupportedCipherSuite};
use rustls_pemfile::{certs, pkcs8_private_keys};
use crate::payload::todoist::{TodoistEvent, TodoistPayload};
use crate::payload::discord::{DiscordWebhookPayload, Embed, EmbedCollection};
use crate::call::api_key::ApiKey;

type PhantomLifetime<'a> = PhantomData<&'a ()>;

struct GenericIncomingDeserializer<'de, D: Deserialize<'de>, F: FnOnce(&'static str) -> D> {
    f: F,
    __phantom: PhantomLifetime<'de>
}

impl <'de, D: Deserialize<'de>, F: FnOnce(&'static str) -> D> GenericIncomingDeserializer<'de, D, F> {
    fn new(f: F) -> Self {
        GenericIncomingDeserializer {
            f,
            __phantom: PhantomData
        }
    }
}

struct GenericOutgoingSerializer<S: Serialize, F: FnOnce(S) -> &'static str> {
    f: F,
    __phantom: PhantomData<S>
}

impl <S: Serialize, F: FnOnce(S) -> &'static str> GenericOutgoingSerializer<S, F> {
    fn new(f: F) -> Self {
        GenericOutgoingSerializer {
            f,
            __phantom: PhantomData
        }
    }
}

struct GenericHandler<'de, D: Deserialize<'de>, S: Serialize, F: 'static + FnOnce(D) -> S, TD: FnOnce(&'static str) -> D, TS: FnOnce(S) -> &'static str> {
    incoming_deserializer: GenericIncomingDeserializer<'de, D, TD>,
    outgoing_serializer: GenericOutgoingSerializer<S, TS>,
    post_url: &'static str,
    mapper: Arc<F>,
    __phantom: PhantomLifetime<'de>
}

impl <'de, D: Deserialize<'de>, S: Serialize, F: 'static + FnOnce(D) -> S, TD: FnOnce(&'static str) -> D, TS: FnOnce(S) -> &'static str> GenericHandler<'de, D, S, F, TD, TS> {
    fn new(post_url: &'static str, incoming_deserializer: TD, mapper: F, outgoing_serializer: TS) -> Self {
        GenericHandler {
            incoming_deserializer: GenericIncomingDeserializer::new(incoming_deserializer),
            outgoing_serializer: GenericOutgoingSerializer::new(outgoing_serializer),
            post_url,
            mapper: Arc::new(mapper),
            __phantom: PhantomData
        }
    }
}

struct JsonHandler<'de, D: Deserialize<'de>, S: Serialize, F: 'static + FnOnce(D) -> S + ?Sized> {
    to: &'static str,
    f: Arc<F>,
    __phantom_de: PhantomLifetime<'de>,
    __phantom_s: PhantomData<S>,
    __phantom_d: PhantomData<D>
}

impl <'de, D: Deserialize<'de>, S: Serialize, F: 'static + FnOnce(D) -> S> JsonHandler<'de, D, S, F> {
    fn new(to: &'static str, f: F) -> Self {
        JsonHandler::<'de, D, S, F> {
            to, f: Arc::new(f),
            __phantom_d: PhantomData,
            __phantom_s: PhantomData,
            __phantom_de: PhantomData
        }
    }
}

async fn handle<'de, D: Deserialize<'de>, S: Serialize, F: 'static + Copy + FnOnce(D) -> S>(
    handler: Arc<JsonHandler<'de, D, S, F>>,
    Json(incoming_data): actix_web::web::Json<D>,
    Query(api_key): actix_web::web::Query<ApiKey>,
) -> impl Responder {
    // TODO: api_key=something in query string
    dbg!("enter");
    let client = reqwest::Client::new();
    let outgoing_data: &S = &(handler.f)(incoming_data);
    let result = client
        .post(handler.to)
        .json(outgoing_data)
        .send()
        .await
        .map(|_| ())
        .map_err(|x| anyhow!(x));
    match result {
        Ok(_) => {
            HttpResponse::NoContent()
        }
        Err(e) => {
            eprintln!("ERROR!!!: {:?}", e);
            HttpResponse::NotModified()
        }
    }
}

#[derive(Serialize)]
struct ErrorDescription {
    reason: String
}

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => {
            HttpResponse::UnsupportedMediaType().body(detail)
        }
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}

fn todoist_to_webhook(incoming_data: TodoistPayload) -> DiscordWebhookPayload {
    let username = Some("Todoist".to_string());
    let avatar_url = Some("https://upload.wikimedia.org/wikipedia/commons/thumb/e/e1/Cib-todoist_%28CoreUI_Icons_v1.0.0%29.svg/240px-Cib-todoist_%28CoreUI_Icons_v1.0.0%29.svg.png".to_string());
    let content = "abx".to_string();
    let tts = false;
    match incoming_data.event {
        TodoistEvent::NoteAdded(note) => {
            DiscordWebhookPayload {
                content,
                username,
                avatar_url,
                tts,
                embeds: EmbedCollection(vec![
                    Embed {
                        title: None,
                        description: None,
                        url: None,
                        color: None,
                        footer: None,
                        image: None,
                        thumbnail: None,
                        video: None,
                        provider: None,
                        author: None,
                        fields: None
                    }
                ]),
                components: Default::default()
            }
        }
        _ => unreachable!("oops")
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // This function contains code snippet which is licensed with Apache License 2.0
    // from https://github.com/actix/examples.
    // See https://www.apache.org/licenses/LICENSE-2.0.txt for full text.
    println!("starting");
    // load SSL keys
    let mut config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    println!("loading cert.pem");
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    println!("loading key.pem");
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let cert_chain = cert_chain.iter().map(|a| Certificate(a.clone())).collect();
    let mut keys = pkcs8_private_keys(key_file).unwrap().iter().map(|x| PrivateKey(x.clone())).collect::<Vec<_>>();
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }
    let config = config.with_single_cert(cert_chain, keys.remove(0)).unwrap();
    println!("Reading config...");
    let running_config = File::open("data/config.yml").unwrap();
    let running_config = serde_json::from_reader::<_, crate::config::config::Config>(BufReader::new(running_config)).unwrap();
    println!("building HttpServer, binding ports...");
    HttpServer::new(|| {
        App::new()
            .app_data(
                JsonConfig::default().error_handler(json_error_handler)
            )
            .service(
                web::resource("/api/from/todoist/to/discord")
                    .route(
                        web::post()
                            .guard(guard::Header("content-type", "application/json"))
                            .to(|a, b| handle::<'static, TodoistPayload, DiscordWebhookPayload, _>(Arc::new(JsonHandler::new(
                                running_config.discord_webhook.unwrap().as_str(),
                                todoist_to_webhook
                            )), a, b))
                    )
                    .route(
                        web::post()
                            .to(|| {
                                HttpResponse::BadRequest().body("Content-Type header must be included")
                            })
                    )
            )
    })
        .bind_rustls("127.0.0.1:443", config)?
        .bind("127.0.0.1:80")?
        .run()
        .await;

    println!("stopped");
    Ok(())
}

struct TypeBox<T>(T);