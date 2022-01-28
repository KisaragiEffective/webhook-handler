#![warn(clippy::pedantic, clippy::nursery)]
#![deny(type_alias_bounds, legacy_derive_helpers, late_bound_lifetime_arguments)]
mod payload;
mod call;
mod config;
mod serde_integration;
mod generic_format_io;

use std::any::Any;
use std::borrow::Borrow;
use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use actix_web::{App, guard, HttpResponse, HttpServer, Responder, web};
use actix_web::web::JsonConfig;
use serde::{Deserialize, Deserializer, Serialize};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use generic_format_io::handler;
use generic_format_io::handler::JsonHandler;
use crate::generic_format_io::outgoing::GenericOutgoingSerializer;
use crate::payload::todoist::{TodoistEvent, TodoistPayload};
use crate::payload::discord::{DiscordWebhookPayload, Embed, EmbedCollection, EmbedField, EmbedFields};
use crate::call::api_key::ApiKey;
use crate::config::config::Config;

type PhantomLifetime<'a> = PhantomData<&'a ()>;

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
                        title: Some("Note added".to_string()),
                        description: None,
                        url: Some("test".to_string()),
                        color: Some(0xFF34eb5e),
                        footer: None,
                        image: None,
                        thumbnail: None,
                        video: None,
                        provider: None,
                        author: None,
                        fields: (EmbedFields(vec![EmbedField {
                            name: "description".to_string(),
                            value: note.content
                        }]))
                    }
                ]),
                components: Default::default()
            }
        }
        _ => unreachable!("oops")
    }
}

static RUNNING_CONFIG: OnceCell<Config> = OnceCell::new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // This function contains code snippet which is licensed with Apache License 2.0
    // from https://github.com/actix/examples.
    // See https://www.apache.org/licenses/LICENSE-2.0.txt for full text.
    println!("starting");
    // load SSL keys
    let mut config = {
        println!("loading cert.pem");
        let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
        let cert_chain = certs(cert_file).unwrap().iter().map(|a| Certificate(a.clone())).collect();
        println!("loading key.pem");
        let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
        let mut keys = pkcs8_private_keys(key_file).unwrap().iter().map(|x| PrivateKey(x.clone())).collect::<Vec<_>>();
        if keys.is_empty() {
            eprintln!("Could not locate PKCS 8 private keys.");
            std::process::exit(1);
        }
        ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, keys.remove(0)).unwrap()
    };

    println!("Reading config...");
    let running_config = File::open("data/config.json").unwrap();
    RUNNING_CONFIG.set(serde_json::from_reader(BufReader::new(running_config)).unwrap());
    println!("building HttpServer");
    let mut http_server = HttpServer::new(|| {
        App::new()
            .app_data(
                JsonConfig::default().error_handler(handler::json_error_handler)
            )
            .service(
                web::resource("/api/from/todoist/to/discord")
                    .route(
                        web::post()
                            .guard(guard::Header("content-type", "application/json"))
                            .to(|a, b| handler::handle(Arc::new(JsonHandler::new(
                                RUNNING_CONFIG.get().unwrap().discord_webhook.clone().unwrap(),
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
    });
    println!("binding ports");
    http_server
        .bind_rustls(format!("127.0.0.1:{}", RUNNING_CONFIG.get().unwrap().https_port), config)?
        .bind(format!("127.0.0.1:{}", RUNNING_CONFIG.get().unwrap().http_port))?
        .run()
        .await;

    println!("stopped");
    Ok(())
}
