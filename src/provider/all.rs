use std::any::Any;
use std::collections::HashMap;
use std::ops::Deref;
use std::process::Output;
use std::sync::{Arc, Mutex};
use serde_json::Value;

type Entry<'a, K, V> = (&'a K, &'a V);
type ArcMut<A> = Arc<Mutex<A>>;
type DerefTarget<D> = <D as Deref>::Target;

pub trait DestinationProvider: Send + Sync {
    type OutputType;
}

pub trait SourceProvider: Send + Sync {
    type InputType;
}

pub(crate) trait Registry : Send + Sync {
    type Value;
    fn register(&mut self, name: &str, registering_value: Self::Value);
    fn get_by_name(&self, name: &str) -> Option<&ArcMut<&'static DerefTarget<Self::Value>>> where Self::Value: Deref;
    fn entries(&self) -> Vec<Entry<'_, String, ArcMut<&'static DerefTarget<Self::Value>>>> where Self::Value: Deref;
    fn registered_provider_names(&self) -> Vec<&String>;
}

#[derive(Default)]
pub struct SourceProviderRegistry {
    pub(crate) provider: HashMap<String, ArcMut<&'static (dyn SourceProvider<InputType = &'static dyn Any> + Send + Sync)>>
}

impl SourceProviderRegistry {
    pub(crate) fn new() -> Self {
        SourceProviderRegistry {
            provider: HashMap::new()
        }
    }
}

impl Registry for SourceProviderRegistry {
    type Value = &'static (dyn SourceProvider<InputType = &'static dyn Any> + Send + Sync);

    fn register(&mut self, name: &str, registering_value: Self::Value) {
        self.provider.insert(name.to_string(), Arc::new(Mutex::new(registering_value)));
    }

    fn get_by_name(&self, name: &str) -> Option<&ArcMut<&'static DerefTarget<Self::Value>>> {
        self.provider.get(name).to_owned()
    }

    fn entries(&self) -> Vec<Entry<'_, String, ArcMut<&'static DerefTarget<Self::Value>>>> {
        self.provider.iter().collect()
    }

    fn registered_provider_names(&self) -> Vec<&String> {
        self.provider.keys().collect()
    }
}

#[derive(Default)]
pub struct DestinationProviderRegistry {
    pub(crate) provider: HashMap<String, ArcMut<&'static (dyn DestinationProvider<OutputType = &'static dyn Any> + Send + Sync)>>
}

impl DestinationProviderRegistry {
    pub(crate) fn new() -> Self {
        DestinationProviderRegistry {
            provider: HashMap::new()
        }
    }
}

impl Registry for DestinationProviderRegistry {
    type Value = &'static (dyn DestinationProvider<OutputType = &'static dyn Any> + Send + Sync);

    fn register(&mut self, name: &str, registering_value: Self::Value) {
        self.provider.insert(name.to_string(), Arc::new(Mutex::new(registering_value)));
    }

    fn get_by_name(&self, name: &str) -> Option<&ArcMut<&'static DerefTarget<Self::Value>>> {
        self.provider.get(name)
    }

    fn entries(&self) -> Vec<Entry<'_, String, ArcMut<&'static <Self::Value as Deref>::Target>>> {
        self.provider.iter().collect()
    }

    fn registered_provider_names(&self) -> Vec<&String> {
        self.provider.keys().collect()
    }
}

pub struct Discord;

pub struct Todoist;

impl DestinationProvider for Discord {
    type OutputType = &'static dyn Any;
}

impl SourceProvider for Todoist {
    type InputType = &'static dyn Any;
}
