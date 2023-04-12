use serde::{Deserialize, Serialize};

use crate::queries::Query;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveTicketFreeReq {
    pub input: ReceiveTicketFreeInput,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveTicketFreeInput {
    pub ticket_type: TicketType,
    pub ticket_uid: i32,
}

impl Query for ReceiveTicketFreeReq {
    type Response = ReceiveTicketFreeRes;

    fn query() -> &'static str {
        include_str!("receive_ticket_free.gql")
    }
}

#[derive(Default, Serialize)]
pub enum TicketType {
    #[default]
    TodayGift,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveTicketFreeRes {
    pub receive_ticket_free: ReceiveTicketFree,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveTicketFree {
    pub ticket_count: i32,
}
