use serde_json::json;
use crate::model::*;
use crate::InternalApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetDocumentsRequest<'a> {
    pub(crate) http_client: &'a InternalApiClient,
    pub coll: String,
}
impl<'a> GetDocumentsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Vec<DataDocument>> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/app-api/v1/data/colls/{coll}/docs", coll = self.coll));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for GetDocumentsRequest<'a> {
    type Output = httpclient::InMemoryResult<Vec<DataDocument>>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}