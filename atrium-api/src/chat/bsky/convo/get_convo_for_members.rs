// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `chat.bsky.convo.getConvoForMembers` namespace.
pub const NSID: &str = "chat.bsky.convo.getConvoForMembers";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub members: Vec<crate::types::string::Did>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub convo: crate::chat::bsky::convo::defs::ConvoView,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}