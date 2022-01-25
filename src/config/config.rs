use serde::Deserialize;
use iso8601::DateTime;
use crate::serde_integration::deserializers::*;

#[derive(Deserialize)]
pub(crate) struct Config {
    #[serde(deserialize_with = "deserialize_iso8601")]
    created_at: DateTime,
    pub(crate) discord_webhook: Option<String>,
}
