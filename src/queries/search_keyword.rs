use serde::{Deserialize, Serialize};
use url::Url;

use crate::queries::Query;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchKeywordReq {
    pub input: SearchKeywordInput,
}

impl Query for SearchKeywordReq {
    type Response = SearchKeywordRes;

    fn query() -> &'static str {
        include_str!("search_keyword.gql")
    }
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchKeywordInput {
    pub keyword: String,
    pub page: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchKeywordRes {
    pub search_keyword: SearchKeyword,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchKeyword {
    pub list: Vec<Item>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub thumbnail: String,
    pub row1: String,
    pub row2: Vec<String>,
    pub row3: Row3,
    pub scheme: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row3 {
    pub meta_list: Vec<String>,
}
