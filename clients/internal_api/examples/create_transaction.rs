#![allow(unused_imports)]
use internal_api::InternalApiClient;
use internal_api::model::*;
#[tokio::main]
async fn main() {
    let client = InternalApiClient::from_env();
    let response = client.create_transaction().await.unwrap();
    println!("{:#?}", response);
}