use serde::{Deserialize, Serialize};

use crate::queries::Query;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TodayGiftListReq;

impl Query for TodayGiftListReq {
    type Response = TodayGiftListRes;

    fn query() -> &'static str {
        include_str!("today_gift_list.gql")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodayGiftListRes {
    pub today_gift_list: TodayGiftList,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodayGiftList {
    pub list: Vec<Item>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub ticket_uid: i32,
    pub scheme: String,
    pub is_received: bool,
}
