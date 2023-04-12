use serde::{Deserialize, Serialize};

/// Although is says "check" it is actually like "get"
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentCheckFreeTicketReq {
    pub series_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentCheckFreeTicketRes {
    pub content_check_free_ticket: ContentCheckFreeTicket,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentCheckFreeTicket {
    pub list: Vec<Item>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub count: i32,
}
