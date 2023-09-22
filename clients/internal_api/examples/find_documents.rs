#![allow(unused_imports)]
use internal_api::InternalApiClient;
use internal_api::model::*;
#[tokio::main]
async fn main() {
    let client = InternalApiClient::from_env();
    let coll = "your coll";
    let query = DataQuery {};
    let response = client
        .find_documents(coll, query)
        .projection(serde_json::json!({}))
        .await
        .unwrap();
    println!("{:#?}", response);
}