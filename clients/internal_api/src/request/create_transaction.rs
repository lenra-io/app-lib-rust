use serde_json::json;
use crate::model::*;
use crate::InternalApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateTransactionRequest<'a> {
    pub(crate) http_client: &'a InternalApiClient,
}
impl<'a> CreateTransactionRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<String> {
        let mut r = self.http_client.client.post("/app-api/v1/data/transaction");
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for CreateTransactionRequest<'a> {
    type Output = httpclient::InMemoryResult<String>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}