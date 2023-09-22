use serde_json::json;
use crate::model::*;
use crate::InternalApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateDocumentByIdRequest<'a> {
    pub(crate) http_client: &'a InternalApiClient,
    pub id: String,
    pub coll: String,
    pub id: String,
}
impl<'a> UpdateDocumentByIdRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<DataDocument> {
        let mut r = self
            .http_client
            .client
            .put(
                &format!(
                    "/app-api/v1/data/colls/{coll}/docs/{id}", coll = self.coll, id =
                    self.id
                ),
            );
        r = r.json(json!({ "_id" : self.id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for UpdateDocumentByIdRequest<'a> {
    type Output = httpclient::InMemoryResult<DataDocument>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}