use serde::{Deserialize, Serialize};

use crate::queries::Query;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentHomeProductListReq {
    pub series_id: i32,
    pub first: Option<i32>,
    pub last: Option<i32>,
    pub sort_type: Option<SortType>,
}

#[derive(Serialize)]
pub enum SortType {
    #[serde(rename = "desc")]
    Descending,
    #[serde(rename = "asc")]
    Ascending,
}

impl Query for ContentHomeProductListReq {
    type Response = ContentHomeProductListRes;

    fn query() -> &'static str {
        include_str!("content_home_product_list.gql")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentHomeProductListRes {
    pub content_home_product_list: ContentHomeProductList,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentHomeProductList {
    pub edges: Vec<Edge>,
    pub total_count: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub node: Node,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub row1: Row1,
    pub row2: Vec<String>,
    pub row3: Option<String>,
    pub scheme: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row1 {
    pub title: String,
}
