use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;
use std::process::Output;
use std::sync::{Arc, Mutex};
use iso8601::DateTime;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::payload::todoist::TodoistPayload;
use crate::SizedDeserialize;
use anyhow::{Context, Result};

type Entry<'a, K, V> = (&'a K, &'a V);
type ArcMut<A> = Arc<Mutex<A>>;
type DerefTarget<D> = <D as Deref>::Target;

#[derive(Default)]
pub struct RecvSend<'de, In, Out> {
    __phantom_de: PhantomData<&'de ()>,
    __phantom_in: PhantomData<In>,
    __phantom_out: PhantomData<Out>
}

impl <'de, In, Out> RecvSend<'de, In, Out> {
    async fn on_receive<F: FnOnce(In) -> Out>(&self, raw_json: &'de str, to: String, f: F) -> Result<()>
        where In: Deserialize<'de> + Sized + Send + Sync, Out: Serialize + Sized + Send + Sync {
        let x = reqwest::Client::new();
        let y = serde_json::from_str::<In>(raw_json)
            .map(f)
            .map(|a| x.post(to).json(&a))
            .map(|rb| rb.send());

        match y {
            Ok(a) => {
                a.await.map(|_| ()).context("oops!")
            }
            Err(b) => {
                Err(b).context("oops!")
            }
        }
    }

    pub(crate) fn new<'d, I, O>() -> RecvSend<'d, I, O> {
        RecvSend {
            __phantom_de: PhantomData,
            __phantom_in: PhantomData,
            __phantom_out: PhantomData,
        }
    }
}

#[derive(Default)]
pub struct Sender<Out, Env> {
    __out_phantom: PhantomData<Out>,
    __env_phantom: PhantomData<Env>,
}

impl <Out, Env> Sender<Out, Env> {
    async fn send(to: String, out: Out) {
        todo!("fuck")
    }
}

pub struct Receiver<In> {
    __in_phantom: PhantomData<In>
}

impl <In> Receiver<In> {
    fn parse() {

    }
}

pub trait DestinationProvider: Send + Sync {
    type OutputType;
}

pub trait SourceProvider: Send + Sync {
    type InputType;
}

pub struct GenRegistry<K: Hash + Eq + Sync, V> {
    register: HashMap<K, V>
}

impl <K: Hash + Eq + Sync, V> GenRegistry<K, V> {
    pub(crate) fn register(&mut self, k: K, v: V) {
        self.register.insert(k, v);
    }

    fn unregister(&mut self, k: K) {
        &mut self.register.remove(&k);
    }

    pub(crate) fn get_by_key(&self, k: K) -> Option<&V> {
        self.register.get(&k)
    }

    pub fn registered_keys(&self) -> Vec<&K> {
        self.register.keys().collect()
    }

    fn entries(&self) -> Vec<(&K, &V)> {
        self.register.iter().collect()
    }

    pub(crate) fn new() -> GenRegistry<K, V> {
        GenRegistry {
            register: HashMap::new()
        }
    }
}

impl <K: Hash + Eq + Sync, V> GenRegistry<K, V> {
    pub(crate) fn register_into<L>(&mut self, k: K, v: L) where L: Into<V> {
        &mut self.register.insert(k, v.into());
    }
}

trait E0207Bypass {
    type X;
}

impl <T> E0207Bypass for T { type X = T; }

pub struct MiddlewareRegistry {

}

pub struct Middleware<T, U> {
    input_provider_name: String,
    output_provider_name: String,
    phantom_t: PhantomData<T>,
    phantom_u: PhantomData<U>,
}

impl <T: SourceProvider, U: DestinationProvider> Middleware<T, U> {

}

pub struct Discord;

impl DestinationProvider for Discord {
    type OutputType = &'static dyn Any;
}

pub struct Todoist;

impl SourceProvider for Todoist {
    type InputType = &'static TodoistPayload;
}
