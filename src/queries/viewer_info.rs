use serde::{Deserialize, Serialize};

use super::Query;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewerInfoReq {
    pub series_id: i32,
    pub product_id: i32,
}

impl Query for ViewerInfoReq {
    type Response = ViewerInfoRes;
    fn query() -> &'static str {
        include_str!("viewer_info.gql")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewerInfoRes {
    pub prev_item: Option<NearItem>,
    pub next_item: Option<NearItem>,
    pub viewer_data: ViewerData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NearItem {
    pub product_id: i32,
    pub title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ViewerData {
    TextViewerData {
        content_list: Vec<Content>,
    },
    ImageViewerData {
        image_download_data: ImageDownloadData,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub secure_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageDownloadData {
    pub files: Vec<File>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub size: i32,
    pub secure_url: String,
    pub width: i32,
    pub height: i32,
}

#[tokio::test]
async fn test() {
    println!(
        "{:#?}",
        ViewerInfoReq {
            series_id: 48787424,
            product_id: 50866481,
        }
        .test()
        .await
        .unwrap()
    );
}
