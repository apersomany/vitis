use serde::{Deserialize, Serialize};
use url::Url;

use super::Query;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticLandingRankingSectionReq {
    pub param: StaticLandingRankingSectionParam,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticLandingRankingSectionParam {
    pub category_uid: i32,
    pub page: i32,
}

impl Query for StaticLandingRankingSectionReq {
    type Response = StaticLandingRankingSectionRes;

    fn query() -> &'static str {
        include_str!("static_landing_ranking_section.gql")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticLandingRankingSectionRes {
    pub static_landing_ranking_section: StaticLandingRankingSection,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticLandingRankingSection {
    pub groups: Vec<Group>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub row1: String,
    pub scheme: Url,
}

#[tokio::test]
async fn test() {
    let res = StaticLandingRankingSectionReq::test().await.unwrap();
    println!("{:#?}", res);
}
