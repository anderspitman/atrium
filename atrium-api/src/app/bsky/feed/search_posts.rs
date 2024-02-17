// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.feed.searchPosts` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    ///Optional pagination mechanism; may not necessarily allow scrolling through entire result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::LimitedNonZeroU8<100u8>>,
    ///Search query string; syntax, phrase, boolean, and faceting is unspecified, but Lucene query syntax is recommended.
    pub q: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    ///Count of search hits. Optional, may be rounded/truncated, and may not be possible to paginate through all hits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hits_total: Option<i64>,
    pub posts: Vec<crate::app::bsky::feed::defs::PostView>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    BadQueryString(Option<String>),
}
