#![allow(unused_imports)]
use internal_api::InternalApiClient;
use internal_api::model::*;
#[tokio::main]
async fn main() {
    let client = InternalApiClient::from_env();
    let coll = "your coll";
    let filter = DataQuery {};
    let update = DataUpdate {};
    let response = client.update_many_documents(coll, filter, update).await.unwrap();
    println!("{:#?}", response);
}