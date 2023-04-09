#![feature(impl_trait_projections)]
#![feature(async_fn_in_trait)]

use reqwest::Client;

use crate::queries::{
    series::content_home_overview_and_product_list::ContentHomeOverviewAndProductListReq, Query,
};

pub mod endpoints;
pub mod queries;
pub mod state;

#[tokio::main]
async fn main() {
    let res = ContentHomeOverviewAndProductListReq {
        series_id: 51185123,
        ..Default::default()
    }
    .send(&Client::builder().user_agent("asdf").build().unwrap())
    .await
    .unwrap();
    println!("{:#?}", res);
}
