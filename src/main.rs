use std::str::FromStr;
use actix_web::{web, guard, App, HttpServer, HttpResponse, HttpRequest};
use qstring::QString;

trait DestinationProvider {
    type OutputType;
}

trait SourceProvider {
    type InputType;
}

enum Provider {
    Discord,
    Todoist,
}

impl FromStr for Provider {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "discord" => Ok(Provider::Discord),
            "todoist" => Ok(Provider::Todoist),
            _ => Err(()),
        }
    }
}

impl DestinationProvider for Provider::Discord { type OutputType = String; }

impl SourceProvider for Provider::Todoist { type InputType = String; }

// https://stackoverflow.com/questions/54406029/how-can-i-parse-query-strings-in-actix-web
async fn api_post(request: HttpRequest) -> HttpResponse {
    let qs = QString::from(request.query_string());
    let from_provider = qs.get("from");
    let to_provider = qs.get("to");
    if let Some(from_provider) = from_provider {

    } else {

    }
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route(
                "/api/post",
                web::get()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(api_post))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
