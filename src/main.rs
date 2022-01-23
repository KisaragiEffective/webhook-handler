#![warn(clippy::pedantic, clippy::nursery)]
#![deny(type_alias_bounds, legacy_derive_helpers, late_bound_lifetime_arguments)]
mod payload;

use std::any::Any;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::marker::PhantomData;
use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock};
use actix_web::{App, guard, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Factory;
use actix_web::http::{HeaderName, HeaderValue};
use actix_web::web::Json;
use qstring::QString;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use anyhow::{Result, bail, anyhow, Error};
use crate::payload::todoist::TodoistPayload;
use crate::payload::discord::DiscordWebhookPayload;

type PhantomRef<'a> = PhantomData<&'a ()>;

struct JsonHandler<'de, D: Deserialize<'de>, S: Serialize, F: 'static + FnOnce(D) -> S + ?Sized> {
    to: &'static str,
    f: Arc<F>,
    __phantom_de: PhantomRef<'de>,
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

async fn h<'de, D: Deserialize<'de>, S: Serialize, F: 'static + Copy + FnOnce(D) -> S>(this: Arc<JsonHandler<'de, D, S, F>>, Json(request): actix_web::web::Json<D>) -> impl Responder {
    dbg!("enter");
    let client = reqwest::Client::new();
    let x = &(this.f)(request);
    let result = client.post(this.to).json(x).send().await.map(|_| ()).map_err(|x| anyhow!(x));
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let g =1;
        App::new()
            .service(
                web::resource("/api/from/todoist/to/discord")
                    .route(
                        web::post()
                            .guard(guard::Header("content-type", "application/json"))
                            .to(|a| h::<'static, TodoistPayload, DiscordWebhookPayload, _>( Arc::new(JsonHandler::new(
                                "https://discord.com/api/webhooks/934771607552016465/QIbXdt6S3YE6tNJXeaNPYzJmjb4tGfZALX1z245XPBcFdiIm9TdoSRiye_pvtnDNqfgr",
                                |a: TodoistPayload| {
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
        .await
}

struct TypeBox<T>(T);