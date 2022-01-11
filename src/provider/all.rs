use std::any::Any;
use std::collections::HashMap;
use std::ops::Deref;
use std::process::Output;
use std::sync::{Arc, Mutex};
use serde_json::Value;

trait DestinationProvider {
    type OutputType;
}

trait SourceProvider {
    type InputType;
}

pub(crate) trait Registry {
    type Value;
    fn register(&mut self, name: &str, registering_value: Self::Value);
    fn get_by_name(&self, name: &str) -> Option<Arc<Mutex<<Self::Value as Deref>::Target>>> where Self::Value: Deref;
    fn entries(&self) -> Vec<(&String, Arc<Mutex<<Self::Value as Deref>::Target>>)> where Self::Value: Deref;
    fn registered_provider_names(&self) -> Vec<&String>;
}

#[derive(Default)]
pub struct SourceProviderRegistry {
    pub(crate) provider: HashMap<String, Arc<Mutex<(dyn SourceProvider<InputType = dyn Any> + Send + Sync + 'static)>>>
}

impl SourceProviderRegistry {
    pub(crate) fn new() -> SourceProviderRegistry {
        SourceProviderRegistry {
            provider: HashMap::new()
        }
    }
}
impl Registry for SourceProviderRegistry {
    type Value = Box<dyn SourceProvider<InputType = dyn Any + Send + Sync + 'static> + Send + Sync + 'static>;

    fn register(&mut self, name: &str, registering_value: Self::Value) {
        self.provider.insert(name.to_string(), Arc::new(Mutex::new(registering_value.deref())));
    }

    fn get_by_name(&self, name: &str) -> Option<Arc<Mutex<<Self::Value as Deref>::Target>>> {
        self.provider.get(name).map(|x| *x)
    }

    fn entries(&self) -> Vec<(&String, Arc<Mutex<<Self::Value as Deref>::Target>>)> {
        self.provider.iter().map(|(k, v)| (k, *v)).collect()
    }

    fn registered_provider_names(&self) -> Vec<&String> {
        self.provider.keys().collect()
    }
}

#[derive(Default)]
pub struct DestinationProviderRegistry {
    pub(crate) provider: HashMap<String, Arc<Mutex<dyn DestinationProvider<OutputType = dyn Any> + Send + Sync>>>
}

impl DestinationProviderRegistry {
    pub(crate) fn new() -> DestinationProviderRegistry {
        DestinationProviderRegistry {
            provider: HashMap::new()
        }
    }
}

impl Registry for DestinationProviderRegistry {
    type Value = Box<dyn DestinationProvider<OutputType = dyn Any> + Send + Sync + 'static>;

    fn register(&mut self, name: &str, registering_value: Self::Value) {
        self.provider.insert(name.to_string(), Arc::new(Mutex::new(*registering_value)));
    }

    fn get_by_name(&self, name: &str) -> Option<Arc<Mutex<<Self::Value as Deref>::Target>>> {
        self.provider.get(name).map(|p| *p)
    }

    fn entries(&self) -> Vec<(&String, Arc<Mutex<<Self::Value as Deref>::Target>>)> {
        self.provider.iter().map(|(k, v)| (k, *v)).collect()
    }

    fn registered_provider_names(&self) -> Vec<&String> {
        self.provider.keys().collect()
    }
}

pub(crate) trait Provider {}

impl Provider for dyn SourceProvider<InputType = dyn Any> {}

impl Provider for dyn DestinationProvider<OutputType = dyn Any> {}

trait ProviderPipeline {
    type From: SourceProvider;
    type To: DestinationProvider;
    fn convert(from: <Self::From as SourceProvider>::InputType) -> <<Self as ProviderPipeline>::To as DestinationProvider>::OutputType;
}

pub struct Discord;

pub struct Todoist;

impl Provider for Discord {}

impl DestinationProvider for Discord {
    type OutputType = Vec<i32>;
}

impl Provider for Todoist {}

impl SourceProvider for Todoist {
    type InputType = Vec<i32>;
}
