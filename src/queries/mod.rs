use std::fmt::Debug;

use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod content_check_free_ticket;
pub mod content_home_overview;
pub mod content_home_overview_and_product_list;
pub mod content_home_product_list;
pub mod content_my_ticket;
pub mod receive_ticket_free;
pub mod search_keyword;
pub mod static_landing_ranking_section;
pub mod today_gift_list;
pub mod viewer_info;

pub trait Query: Serialize + Default {
    type Response: DeserializeOwned + Debug;

    fn query() -> &'static str;

    async fn send(&self, client: &Client) -> Result<Self::Response> {
        let res = client
            .post("https://page.kakao.com/graphql")
            .json(&BaseReq {
                query: Self::query(),
                variables: &self,
            })
            .send()
            .await?;
        let res = res.json::<BaseRes<Self::Response>>().await?;
        if let Some(errors) = res.errors {
            let errors = errors
                .into_iter()
                .map(|e| e.message)
                .collect::<Vec<String>>()
                .join("\n");
            Err(anyhow!("Server replied with error(s): \n{}", errors))
        } else {
            Ok(res.data.context("Server replied with no data")?)
        }
    }

    async fn test() -> Result<Self::Response> {
        let client = reqwest::Client::builder()
            .user_agent("test")
            .build()
            .unwrap();
        Self::default().send(&client).await
    }
}

#[derive(Serialize)]
struct BaseReq<'a, T> {
    query: &'static str,
    variables: &'a T,
}

#[derive(Deserialize)]
struct BaseRes<T> {
    data: Option<T>,
    errors: Option<Vec<Error>>,
}

#[derive(Deserialize)]
struct Error {
    message: String,
}
