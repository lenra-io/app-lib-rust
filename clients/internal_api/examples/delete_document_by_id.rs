#![allow(unused_imports)]
use internal_api::InternalApiClient;
use internal_api::model::*;
#[tokio::main]
async fn main() {
    let client = InternalApiClient::from_env();
    let coll = "your coll";
    let id = "your id";
    let response = client.delete_document_by_id(coll, id).await.unwrap();
    println!("{:#?}", response);
}