#![warn(clippy::pedantic, clippy::nursery)]
#![deny(type_alias_bounds, legacy_derive_helpers, late_bound_lifetime_arguments)]
mod payload;

use std::any::Any;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::marker::PhantomData;
use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock};
use actix_web::{App, error, guard, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::web::{Json, JsonConfig};
use qstring::QString;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use anyhow::{Result, bail, anyhow, Error};
use crate::payload::todoist::{TodoistEvent, TodoistPayload};
use crate::payload::discord::{DiscordWebhookPayload, Embed, EmbedCollection};

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

async fn handle<'de, D: Deserialize<'de>, S: Serialize, F: 'static + Copy + FnOnce(D) -> S>(this: Arc<JsonHandler<'de, D, S, F>>, Json(incoming_data): actix_web::web::Json<D>) -> impl Responder {
    dbg!("enter");
    let client = reqwest::Client::new();
    let outgoing_data: &S = &(this.f)(incoming_data);
    let result = client
        .post(this.to)
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
    dbg!("start");
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
                            .to(|a| handle::<'static, TodoistPayload, DiscordWebhookPayload, _>(Arc::new(JsonHandler::new(
                                "https://discord.com/api/webhooks/934771607552016465/QIbXdt6S3YE6tNJXeaNPYzJmjb4tGfZALX1z245XPBcFdiIm9TdoSRiye_pvtnDNqfgr",
                                todoist_to_webhook
                            )), a))
                    )
                    .route(
                        web::post()
                            .to(|| {
                                HttpResponse::BadRequest().body("Content-Type header must be included")
                            })
                    )
            )
            .service(
                web::resource("/api/test")
                    .route(
                        web::post()
                            .guard(guard::Header("content-type", "application/json"))
                            .to(|a| handle::<'static, i32, DiscordWebhookPayload, _>(Arc::new(JsonHandler::new(
                                "https://discord.com/api/webhooks/934771607552016465/QIbXdt6S3YE6tNJXeaNPYzJmjb4tGfZALX1z245XPBcFdiIm9TdoSRiye_pvtnDNqfgr",
                                |a: i32| {
                                    DiscordWebhookPayload {
                                        content: "Shit!".to_string(),
                                        username: None,
                                        avatar_url: None,
                                        tts: false,
                                        embeds: Default::default(),
                                        components: Default::default()
                                    }
                                }
                            )), a))
                    )
                    .route(
                        web::post()
                            .to(|| {
                                HttpResponse::BadRequest().body("Content-Type header must be included")
                            })
                    )
            )

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await;

    dbg!("stop");
    Ok(())
}

struct TypeBox<T>(T);