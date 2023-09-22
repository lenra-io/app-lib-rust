#![allow(unused_imports)]
use internal_api::InternalApiClient;
use internal_api::model::*;
#[tokio::main]
async fn main() {
    let client = InternalApiClient::from_env();
    let coll = "your coll";
    let response = client.delete_collection(coll).await.unwrap();
    println!("{:#?}", response);
}