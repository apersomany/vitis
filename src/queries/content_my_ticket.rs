use serde::{Deserialize, Serialize};

use super::Query;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentMyTicketReq {
    pub series_id: i32,
}

impl Query for ContentMyTicketReq {
    type Response = ContentMyTicketRes;

    fn query() -> &'static str {
        include_str!("content_my_ticket.gql")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentMyTicketRes {
    pub content_my_ticket: ContentMyTicket,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentMyTicket {
    pub ticket_own_count: i32,
    pub ticket_rental_count: i32,
    pub waitfree: Option<Waitfree>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Waitfree {
    pub charged_complete: bool,
}
