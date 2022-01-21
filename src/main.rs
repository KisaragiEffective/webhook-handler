#![warn(clippy::pedantic, clippy::nursery)]
#![deny(type_alias_bounds)]
mod provider;

use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock};
use actix_web::{App, guard, HttpRequest, HttpResponse, HttpServer, web};
use qstring::QString;
use serde_json::{json, Value};
use provider::all::{Discord, Todoist};
use crate::provider::all::{DestinationProviderRegistry, SourceProviderRegistry, Registry};

// https://stackoverflow.com/questions/54406029/how-can-i-parse-query-strings-in-actix-web
async fn api_post(request: HttpRequest) -> HttpResponse {
    let qs = QString::from(request.query_string());
    let from_provider = qs.get("from");
    let to_provider = qs.get("to");
    from_provider.map_or_else(
        || HttpResponse::BadRequest().json(BadRequestErrors::required_query_parameter("from")),
        |from_provider| {
            to_provider.map_or_else(
                || HttpResponse::BadRequest().json(BadRequestErrors::required_query_parameter("to")),
                |to_provider| {
                    let source_registry = &SOURCE_PROVIDER_REGISTRY.get().unwrap().read().unwrap();
                    source_registry.get_by_name(from_provider).map_or_else(
                        || HttpResponse::BadRequest().json(BadRequestErrors::indicated_unsupported_platform(
                            "from",
                            source_registry.registered_provider_names())),
                        |from_provider| {
                            let dest_registry = &DESTINATION_PROVIDER_REGISTRY.get().unwrap().read().unwrap();
                            dest_registry.get_by_name(to_provider).map_or_else(
                                || HttpResponse::BadRequest().json(BadRequestErrors::indicated_unsupported_platform(
                                    "to",
                                    dest_registry.registered_provider_names())),
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

    fn indicated_unsupported_platform(name: &str, supported_platforms: Vec<&String>) -> Value {
        json!({
            "code": 400,
            "reason": "query parameter 'from' indicates unsupported platform",
            "name": name,
            "supported_platforms": supported_platforms
        })
    }
}

static SOURCE_PROVIDER_REGISTRY: OnceCell<Arc<RwLock<SourceProviderRegistry>>> = OnceCell::new();
static DESTINATION_PROVIDER_REGISTRY: OnceCell<Arc<RwLock<DestinationProviderRegistry>>> = OnceCell::new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SOURCE_PROVIDER_REGISTRY.set(Arc::new(RwLock::new(SourceProviderRegistry::new()))).ok().unwrap();
    DESTINATION_PROVIDER_REGISTRY.set(Arc::new(RwLock::new(DestinationProviderRegistry::new()))).ok().unwrap();
    if let Some(mut ad) = SOURCE_PROVIDER_REGISTRY.get() {
        let mut register = ad.write().unwrap();
        // register.register("discord", &(Todoist));
        // register.register_source_provider("github", GitHub);
    }

    if let Some(mut ad) = DESTINATION_PROVIDER_REGISTRY.get() {
        let mut register = ad.write().unwrap();
        // register.register("todoist", &(Discord));
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
