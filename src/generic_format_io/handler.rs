use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::marker::PhantomData;
use crate::{GenericOutgoingSerializer, PhantomLifetime};
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
