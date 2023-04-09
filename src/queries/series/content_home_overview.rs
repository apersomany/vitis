use serde::{Deserialize, Serialize};

use crate::queries::Query;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentHomeOverviewReq {
    pub series_id: i32,
}

impl Query for ContentHomeOverviewReq {
    type Response = ContentHomeOverviewRes;

    fn query() -> &'static str {
        include_str!("content_home_overview.gql")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentHomeOverviewRes {
    pub content_home_overview: ContentHomeOverview,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentHomeOverview {
    pub content: Content,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub title: String,
    pub thumbnail: String,
    pub authors: String,
    pub waitfree_block_count: i32,
    pub free_slide_count: i32,
    pub service_property: ServiceProperty,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceProperty {
    pub view_count: i32,
    pub rating_count: i32,
    pub rating_sum: i32,
}
