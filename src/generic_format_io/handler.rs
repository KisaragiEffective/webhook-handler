use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::marker::PhantomData;
use actix_web::{error, HttpRequest, HttpResponse, Responder};
use actix_web::web::{Json, Query};
use anyhow::anyhow;
use log::{error, trace};
use crate::{ApiKey, GenericOutgoingSerializer, PhantomLifetime};
use crate::generic_format_io::incoming::GenericIncomingDeserializer;

pub struct GenericHandler<'de, D: Deserialize<'de>, S: Serialize, F: 'static + FnOnce(D) -> S, TD: FnOnce(&'static str) -> D, TS: FnOnce(S) -> &'static str> {
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

pub struct JsonHandler<'de, D: Deserialize<'de>, S: Serialize, F: 'static + FnOnce(D) -> S + ?Sized> {
    to: String,
    f: Arc<F>,
    __phantom_de: PhantomLifetime<'de>,
    __phantom_s: PhantomData<S>,
    __phantom_d: PhantomData<D>
}

impl <'de, D: Deserialize<'de>, S: Serialize, F: 'static + FnOnce(D) -> S> JsonHandler<'de, D, S, F> {
    pub(crate) fn new(to: String, f: F) -> Self {
        JsonHandler::<'de, D, S, F> {
            to, f: Arc::new(f),
            __phantom_d: PhantomData,
            __phantom_s: PhantomData,
            __phantom_de: PhantomData
        }
    }
}

// TODO: input type can be inferred by Content-Type
pub async fn handle<'de, D: Deserialize<'de>, S: Serialize, F: 'static + Copy + FnOnce(D) -> S>(
    handler: Arc<JsonHandler<'de, D, S, F>>,
    Json(incoming_data): actix_web::web::Json<D>,
    Query(api_key): actix_web::web::Query<ApiKey>,
) -> impl Responder {
    // TODO: api_key=something in query string
    trace!("enter");
    let client = reqwest::Client::new();
    let outgoing_data: &S = &(handler.f)(incoming_data);
    let result = client
        .post(&handler.to)
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
            error!("ERROR!!!: {:?}", e);
            HttpResponse::NotModified()
        }
    }
}

pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;
    use actix_web::HttpResponse;

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
