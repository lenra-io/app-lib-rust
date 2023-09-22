use serde_json::json;
use crate::model::*;
use crate::InternalApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct FindDocumentsRequest<'a> {
    pub(crate) http_client: &'a InternalApiClient,
    pub coll: String,
    pub projection: Option<serde_json::Value>,
    pub query: DataQuery,
}
impl<'a> FindDocumentsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Vec<DataDocument>> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/app-api/v1/data/colls/{coll}/find", coll = self.coll));
        if let Some(ref unwrapped) = self.projection {
            r = r.json(json!({ "projection" : unwrapped }));
        }
        r = r.json(json!({ "query" : self.query }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn projection(mut self, projection: serde_json::Value) -> Self {
        self.projection = Some(projection);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FindDocumentsRequest<'a> {
    type Output = httpclient::InMemoryResult<Vec<DataDocument>>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}