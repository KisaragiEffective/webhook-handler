use std::any::Any;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;
use once_cell::sync::OnceCell;
use std::str::FromStr;
use std::sync::Arc;
use actix_web::{web, guard, App, HttpServer, HttpResponse, HttpRequest};
use actix_web::dev::RequestHead;
use actix_web::guard::Guard;
use qstring::QString;
use serde_json::{json, Value};

trait DestinationProvider {
    type OutputType;
}

trait SourceProvider {
    type InputType;
}
struct ProviderRegistry {
    hash_map: Box<HashMap<String, ProviderProxy>>
}

impl ProviderRegistry {
    fn register<P: 'static + Provider + Sync + Send>(&mut self, name: &str, provider: P) {
        self.hash_map.insert(name.to_string(), ProviderProxy::new(provider));
    }
}

static PROVIDER_REGISTRY: OnceCell<Arc<&mut ProviderRegistry>> = OnceCell::new();
struct ProviderProxy {
    back: Box<dyn Provider>
}

unsafe impl Sync for ProviderRegistry {}
unsafe impl Send for ProviderRegistry {}

impl ProviderProxy {
    fn new<P: 'static + Provider>(back: P) -> ProviderProxy {
        ProviderProxy { back: Box::new(back) }
    }
}
trait Provider {}
impl Provider for dyn SourceProvider<InputType = dyn Any> {}
impl Provider for dyn DestinationProvider<OutputType = dyn Any> {}

trait ProviderPipeline {
    type From: SourceProvider;
    type To: DestinationProvider;
    fn convert(from: <Self::From as SourceProvider>::InputType) -> <<Self as ProviderPipeline>::To as DestinationProvider>::OutputType;
}

struct Discord;
struct Todoist;

impl Provider for Discord {}
impl DestinationProvider for Discord {
    type OutputType = ();
}

impl Provider for Todoist {}
impl SourceProvider for Todoist {
    type InputType = ();
}

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
                    let registry = &PROVIDER_REGISTRY.get().unwrap().hash_map;
                    registry.get(from_provider).map_or_else(
                        || HttpResponse::BadRequest().json(BadRequestErrors::indicated_unsupported_platform(
                            "from",
                            registry.keys().collect::<Vec<_>>())),
                        |from_provider| {
                            registry.get(to_provider).map_or_else(
                                || HttpResponse::BadRequest().json(BadRequestErrors::indicated_unsupported_platform(
                                    "to",
                                    registry.keys().collect::<Vec<_>>())),
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let x: &'static mut ProviderRegistry = &mut ProviderRegistry { hash_map: Box::new(HashMap::new()) };
    PROVIDER_REGISTRY.set(Arc::new(x)).ok().unwrap();

    if let Some(mut ad) = PROVIDER_REGISTRY.get() {
        let mut xy = ad.borrow_mut();
        &xy.borrow_mut().register("discord", Discord);
        xy.borrow_mut().register("todoist", Todoist);
    }
    HttpServer::new(|| {
        App::new()
            .route(
                "/api/post",
                web::get()
                    .guard(guard::Not(guard::Post()))
                    .to(|| { HttpResponse::MethodNotAllowed().json(json!({
                        "reason": "You must use POST request"
                    })) })
            )
            .route(
                "/api/post",
                web::get()
                    .guard(guard::Not(guard::Header("content-type", "application/json")))
                    .to(|| { HttpResponse::BadRequest().json(json!({
                        "reason": "header Content-Type must be included"
                    })) })
            )
            .route(
                "/api/post",
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(api_post)
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
