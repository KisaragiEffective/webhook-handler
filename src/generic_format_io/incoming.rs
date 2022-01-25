use std::marker::PhantomData;
use serde::Deserialize;
use crate::PhantomLifetime;

pub struct GenericIncomingDeserializer<'de, D: Deserialize<'de>, F: FnOnce(&'static str) -> D> {
    f: F,
    __phantom: PhantomLifetime<'de>
}

impl <'de, D: Deserialize<'de>, F: FnOnce(&'static str) -> D> GenericIncomingDeserializer<'de, D, F> {
    pub(crate) fn new(f: F) -> Self {
        GenericIncomingDeserializer {
            f,
            __phantom: PhantomData
        }
    }
}