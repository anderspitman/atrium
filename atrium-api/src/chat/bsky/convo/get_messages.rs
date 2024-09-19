// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `chat.bsky.convo.getMessages` namespace.
pub const NSID: &str = "chat.bsky.convo.getMessages";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    pub convo_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::LimitedNonZeroU8<100u8>>,
}
pub type Parameters = crate::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub messages: Vec<crate::types::Union<OutputMessagesItem>>,
}
pub type Output = crate::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum OutputMessagesItem {
    #[serde(rename = "chat.bsky.convo.defs#messageView")]
    ChatBskyConvoDefsMessageView(Box<crate::chat::bsky::convo::defs::MessageView>),
    #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
    ChatBskyConvoDefsDeletedMessageView(
        Box<crate::chat::bsky::convo::defs::DeletedMessageView>,
    ),
}
