use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod search {
    pub mod search_keyword;
}
pub mod series {
    pub mod content_home_overview;
    pub mod content_home_overview_and_product_list;
    pub mod content_home_product_list;
}

pub trait Query: Serialize {
    type Response: DeserializeOwned;

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
