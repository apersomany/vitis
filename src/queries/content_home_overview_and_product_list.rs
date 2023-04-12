use serde::{Deserialize, Serialize};

use crate::queries::Query;

use super::{
    content_home_overview::ContentHomeOverview,
    content_home_product_list::{ContentHomeProductList, SortType},
};

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentHomeOverviewAndProductListReq {
    pub series_id: i32,
    pub first: Option<i32>,
    pub last: Option<i32>,
    pub sort_type: Option<SortType>,
}

impl Query for ContentHomeOverviewAndProductListReq {
    type Response = ContentHomeOverviewAndProductListRes;

    fn query() -> &'static str {
        include_str!("content_home_overview_and_product_list.gql")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentHomeOverviewAndProductListRes {
    pub content_home_overview: ContentHomeOverview,
    pub content_home_product_list: ContentHomeProductList,
}
