// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `chat.bsky.convo.defs` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConvoView {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message: Option<crate::types::Union<ConvoViewLastMessageRefs>>,
    pub members: Vec<crate::chat::bsky::actor::defs::ProfileViewBasic>,
    pub muted: bool,
    pub rev: String,
    pub unread_count: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeletedMessageView {
    pub id: String,
    pub rev: String,
    pub sender: MessageViewSender,
    pub sent_at: crate::types::string::Datetime,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LogBeginConvo {
    pub convo_id: String,
    pub rev: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LogCreateMessage {
    pub convo_id: String,
    pub message: crate::types::Union<LogCreateMessageMessageRefs>,
    pub rev: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LogDeleteMessage {
    pub convo_id: String,
    pub message: crate::types::Union<LogDeleteMessageMessageRefs>,
    pub rev: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LogLeaveConvo {
    pub convo_id: String,
    pub rev: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MessageInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<crate::types::Union<MessageInputEmbedRefs>>,
    ///Annotations of text (mentions, URLs, hashtags, etc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<crate::app::bsky::richtext::facet::Main>>,
    pub text: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MessageRef {
    pub convo_id: String,
    pub did: crate::types::string::Did,
    pub message_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MessageView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<crate::types::Union<MessageViewEmbedRefs>>,
    ///Annotations of text (mentions, URLs, hashtags, etc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<crate::app::bsky::richtext::facet::Main>>,
    pub id: String,
    pub rev: String,
    pub sender: MessageViewSender,
    pub sent_at: crate::types::string::Datetime,
    pub text: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MessageViewSender {
    pub did: crate::types::string::Did,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ConvoViewLastMessageRefs {
    #[serde(rename = "chat.bsky.convo.defs#messageView")]
    MessageView(Box<MessageView>),
    #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
    DeletedMessageView(Box<DeletedMessageView>),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum LogCreateMessageMessageRefs {
    #[serde(rename = "chat.bsky.convo.defs#messageView")]
    MessageView(Box<MessageView>),
    #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
    DeletedMessageView(Box<DeletedMessageView>),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum LogDeleteMessageMessageRefs {
    #[serde(rename = "chat.bsky.convo.defs#messageView")]
    MessageView(Box<MessageView>),
    #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
    DeletedMessageView(Box<DeletedMessageView>),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum MessageInputEmbedRefs {
    #[serde(rename = "app.bsky.embed.record")]
    AppBskyEmbedRecordMain(Box<crate::app::bsky::embed::record::Main>),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum MessageViewEmbedRefs {
    #[serde(rename = "app.bsky.embed.record#view")]
    AppBskyEmbedRecordView(Box<crate::app::bsky::embed::record::View>),
}
