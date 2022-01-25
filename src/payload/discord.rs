use serde::Serialize;
use serde_json::Value;
use iso8601::DateTime as ISO8601DateTime;
/// for more information, see https://discord.com/developers/docs/resources/webhook#execute-webhook
#[derive(Serialize)]
struct DiscordWebhookQueryPayload {
    #[serde(default = "_false")]
    wait: bool,
    thread_id: Option<ThreadID>,
}

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
struct ThreadID(u64);

/// for more information, see https://discord.com/developers/docs/resources/webhook
#[derive(Serialize)]
pub(crate) struct DiscordWebhookPayload {
    pub(crate) content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) avatar_url: Option<String>,
    #[serde(default = "_false")]
    pub(crate) tts: bool,
    #[serde(default)]
    pub(crate) embeds: EmbedCollection,
    // #[serde(default)]
    // allowed_mentions: MentionAssertion,
    #[serde(default)]
    pub(crate) components: Components,
    // files: Vec<File>,
    // payload_json: Value,
    // attatchments: PartialAttachment,
}

#[derive(Serialize, Default)]
pub struct Components(Vec<Component>);

#[derive(Serialize)]
pub struct Component {

}

#[derive(Serialize, Default)]
pub struct EmbedCollection(pub Vec<Embed>);

/// https://discord.com/developers/docs/resources/channel#embed-object
#[derive(Serialize, Default)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none", serialize_with = "f")]
    // timestamp: Option<ISO8601DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedThumbnail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedVideo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<EmbedFields>
}

/// https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure
#[derive(Serialize)]
pub struct EmbedFooter {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_icon_url: Option<String>
}

//noinspection DuplicatedCode
/// https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure
#[derive(Serialize)]
pub struct EmbedImage {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<usize>,
}

//noinspection DuplicatedCode
/// https://discord.com/developers/docs/resources/channel#embed-object-embed-thumbnail-structure
#[derive(Serialize)]
pub struct EmbedThumbnail {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<usize>,
}

#[derive(Serialize)]
pub struct EmbedVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<usize>,
}

/// https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure
#[derive(Serialize)]
pub struct EmbedProvider {
    name: Option<String>,
    url: Option<String>,
}

/// https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure
#[derive(Serialize)]
pub struct EmbedAuthor {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_icon_url: Option<String>,
}

#[derive(Serialize, Eq, PartialEq, Clone, Hash)]
pub struct EmbedFields(Vec<EmbedField>);

/// https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure
#[derive(Serialize, Eq, PartialEq, Clone, Hash)]
pub struct EmbedField {
    name: String,
    value: String,
    // #[serde(skip_serializing_if = "_false()")]
    // inline: bool,
}

#[inline]
const fn _false() -> bool {
    false
}