use serde_json::json;
use crate::model::*;
use crate::InternalApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateManyDocumentsRequest<'a> {
    pub(crate) http_client: &'a InternalApiClient,
    pub coll: String,
    pub filter: DataQuery,
    pub update: DataUpdate,
}
impl<'a> UpdateManyDocumentsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!("/app-api/v1/data/colls/{coll}/updateMany", coll = self.coll),
            );
        r = r.json(json!({ "filter" : self.filter }));
        r = r.json(json!({ "update" : self.update }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for UpdateManyDocumentsRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}