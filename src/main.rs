#![warn(clippy::pedantic, clippy::nursery)]
#![deny(type_alias_bounds, legacy_derive_helpers)]
mod provider;
mod payload;

use std::any::Any;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::marker::PhantomData;
use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock};
use actix_web::{App, guard, HttpRequest, HttpResponse, HttpServer, web};
use qstring::QString;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use provider::all::{Discord, Todoist};
use crate::payload::todoist::TodoistPayload;
use crate::provider::all::{SourceProvider, GenRegistry, DestinationProvider, Sender, RecvSend};

// https://stackoverflow.com/questions/54406029/how-can-i-parse-query-strings-in-actix-web
async fn api_post(request: HttpRequest) -> HttpResponse {
    let qs = QString::from(request.query_string());
    let from_provider = qs.get("from");
    let to_provider = qs.get("to");
    from_provider.map_or_else(
        || HttpResponse::BadRequest().json(BadRequestErrors::required_query_parameter("from")),
        |from_provider_name| {
            to_provider.map_or_else(
                || HttpResponse::BadRequest().json(BadRequestErrors::required_query_parameter("to")),
                |to_provider_name| {
                    let source_registry = SOURCE_PROVIDER_REGISTRY.borrow().get().unwrap().read().unwrap();
                    source_registry.get_by_key(from_provider_name).map_or_else(
                        || HttpResponse::BadRequest().json(BadRequestErrors::indicated_unsupported_platform(
                            "from",
                            source_registry.registered_keys())),
                        |from_provider| {
                            let dest_registry = DESTINATION_PROVIDER_REGISTRY.borrow().get().unwrap().read().unwrap();
                            dest_registry.get_by_key(to_provider_name).map_or_else(
                                || HttpResponse::BadRequest().json(BadRequestErrors::indicated_unsupported_platform(
                                    "to",
                                    dest_registry.registered_keys())),
                                |to_provider| {

                                    HttpResponse::NoContent().finish()
                                }
                            )
                        }
                    )
                }
            )
        }
    )
}

struct BadRequestErrors {}

impl BadRequestErrors {
    fn required_query_parameter(name: &str) -> Value {
        json!({
            "code": 400,
            "reason": format!("query parameter '{}' is required", name)
        })
    }

    fn indicated_unsupported_platform(name: &str, supported_platforms: Vec<&&str>) -> Value {
        json!({
            "code": 400,
            "reason": "query parameter 'from' indicates unsupported platform",
            "name": name,
            "supported_platforms": supported_platforms
        })
    }
}

// bypass E0225
trait SizedDeserialize<'a> : Send + Sync + Deserialize<'a> {}
trait SizedSerialize : Send + Sync + Serialize {}

trait TypeTag {
    type Tag;
}

struct DiscordTypeTag;
impl TypeTag for DiscordTypeTag { type Tag = Discord; }

struct TodoistTypeTag;
impl TypeTag for TodoistTypeTag { type Tag = TodoistPayload; }

struct X<'a, D: SizedDeserialize<'a>, S: SizedSerialize> {
    de: D,
    ser: S,
    __phantom_data: PhantomData<&'a ()>
}

impl <'a, D: TypeTag + SizedDeserialize<'a>, S: TypeTag + SizedSerialize> Into<RecvSend<'a, Box<D::Tag>, Box<S::Tag>>> for X<'a, D, S> {
    fn into(self) -> RecvSend<'a, Box<D>, Box<S>> {
        RecvSend::new()
    }
}

type ConcurrentGenRegistry<K, V> = Arc<RwLock<GenRegistry<K, V>>>;
static RECV_SEND_REGISTRY: OnceCell<ConcurrentGenRegistry<(&'static str, &'static str), RecvSend<'static, Box<dyn SizedDeserialize>, Box<dyn SizedSerialize>>>> = OnceCell::new();
static SOURCE_PROVIDER_REGISTRY: OnceCell<ConcurrentGenRegistry<&'static str, &Box<&'static dyn SourceProvider<InputType = dyn Any>>>> = OnceCell::new();
static DESTINATION_PROVIDER_REGISTRY: OnceCell<ConcurrentGenRegistry<&'static str, &Box<&'static dyn DestinationProvider<OutputType = dyn Any>>>> = OnceCell::new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    RECV_SEND_REGISTRY.set(Arc::new(RwLock::new(GenRegistry::new()))).ok().unwrap();
    if let Some(recv_send) = RECV_SEND_REGISTRY.get() {
        let mut recv_send = recv_send.write().unwrap();
        recv_send.register(("todoist", "discord"), X {
            de: TodoistTypeTag,
            ser: DiscordTypeTag
        }.into())
    }

    if let Some(ad) = DESTINATION_PROVIDER_REGISTRY.get() {
        let mut register = ad.write().unwrap();
        let y: Box<&dyn Any> = Box::new(&Todoist);
        let z = y.downcast_ref::<Box<&'static dyn DestinationProvider<OutputType = dyn Any>>>();
        register.register("discord", z.unwrap());
    }
    HttpServer::new(|| {
        App::new()
            .route(
                "/api/post",
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(api_post)
            )
            .route(
                "/api/post",
                web::route()
                    .guard(guard::Not(guard::Post()))
                    .to(|| { HttpResponse::MethodNotAllowed().json(json!({
                        "reason": "You must use POST request"
                    })) })
            )
            .route(
                "/api/post",
                web::route()
                    .guard(guard::Not(guard::Header("content-type", "application/json")))
                    .to(|| { HttpResponse::BadRequest().json(json!({
                        "reason": "header Content-Type must be included"
                    })) })
            )

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

struct TypeBox<T>(T);