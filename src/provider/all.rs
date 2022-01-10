use std::any::Any;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

trait DestinationProvider {
    type OutputType;
}

trait SourceProvider {
    type InputType;
}

pub struct ProviderRegistry {
    pub(crate) source_provider: HashMap<String, Arc<Mutex<dyn SourceProvider + Send + Sync>>>
    pub(crate) destination_provider: HashMap<String, Arc<Mutex<dyn SourceProvider + Send + Sync>>>
}

impl ProviderRegistry {
    pub fn new() -> ProviderRegistry {
        ProviderRegistry {
            source_provider: HashMap::new(),
            destination_provider: HashMap::new()
        }
    }

    pub(crate) fn register_destination_provider<P: 'static + DestinationProvider + Send + Sync>(&mut self, name: &str, provider: P) {
        self.source_provider.insert(name.to_string(), Arc::new(Mutex::new(provider)));
    }

    pub(crate) fn register_source_provider<P: 'static + SourceProvider + Send + Sync>(&mut self, name: &str, provider: P) {
        self.source_provider.insert(name.to_string(), Arc::new(Mutex::new(provider)));
    }

    pub(crate) fn get_by_name(&self, name: &str) -> Option<&Arc<Mutex<dyn Provider + Sync + Send>>> {
        self.source_provider.get(name)
    }

    pub(crate) fn registered_provider_entries(&self) -> Vec<(&String, &Arc<Mutex<dyn Provider + Sync + Send>>)> {
        self.source_provider.iter().collect()
    }

    pub(crate) fn registered_provider_names(&self) -> Vec<String> {
        self.source_provider.keys().map(String::from).collect()
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
    type OutputType = ();
}

impl Provider for Todoist {}

impl SourceProvider for Todoist {
    type InputType = ();
}
