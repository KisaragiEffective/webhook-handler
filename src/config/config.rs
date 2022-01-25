use serde::Deserialize;
use iso8601::DateTime;

#[derive(Deserialize)]
pub(crate) struct Config {
    created_at: DateTime,
    pub(crate) discord_webhook: Option<String>,
}
