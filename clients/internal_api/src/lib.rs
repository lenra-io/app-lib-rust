//! [`InternalApiClient`](struct.InternalApiClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;
pub struct InternalApiClient {
    pub client: httpclient::Client,
    authentication: InternalApiAuthentication,
}
impl InternalApiClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new().base_url("http://localhost:4001"),
            authentication: InternalApiAuthentication::from_env(),
        }
    }
}
impl InternalApiClient {
    pub fn new(url: &str, authentication: InternalApiAuthentication) -> Self {
        let client = httpclient::Client::new().base_url(url);
        Self { client, authentication }
    }
    pub fn with_authentication(
        mut self,
        authentication: InternalApiAuthentication,
    ) -> Self {
        self.authentication = authentication;
        self
    }
    pub(crate) fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            InternalApiAuthentication::BearerAuth { bearer_auth } => {
                r = r.bearer_auth(bearer_auth);
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    ///Deletes a collection from database
    pub fn delete_collection(&self, coll: &str) -> request::DeleteCollectionRequest {
        request::DeleteCollectionRequest {
            http_client: &self,
            coll: coll.to_owned(),
        }
    }
    ///Gets documents from database
    pub fn get_documents(&self, coll: &str) -> request::GetDocumentsRequest {
        request::GetDocumentsRequest {
            http_client: &self,
            coll: coll.to_owned(),
        }
    }
    ///Creates a document in database
    pub fn create_document(&self, coll: &str) -> request::CreateDocumentRequest {
        request::CreateDocumentRequest {
            http_client: &self,
            coll: coll.to_owned(),
        }
    }
    ///Gets a document from database
    pub fn get_document_by_id(
        &self,
        coll: &str,
        id: &str,
    ) -> request::GetDocumentByIdRequest {
        request::GetDocumentByIdRequest {
            http_client: &self,
            coll: coll.to_owned(),
            id: id.to_owned(),
        }
    }
    ///Updates a document in database
    pub fn update_document_by_id(
        &self,
        id: &str,
        coll: &str,
        id: &str,
    ) -> request::UpdateDocumentByIdRequest {
        request::UpdateDocumentByIdRequest {
            http_client: &self,
            id: id.to_owned(),
            coll: coll.to_owned(),
            id: id.to_owned(),
        }
    }
    ///Deletes a document from database
    pub fn delete_document_by_id(
        &self,
        coll: &str,
        id: &str,
    ) -> request::DeleteDocumentByIdRequest {
        request::DeleteDocumentByIdRequest {
            http_client: &self,
            coll: coll.to_owned(),
            id: id.to_owned(),
        }
    }
    ///Finds documents in database
    pub fn find_documents(
        &self,
        coll: &str,
        query: DataQuery,
    ) -> request::FindDocumentsRequest {
        request::FindDocumentsRequest {
            http_client: &self,
            coll: coll.to_owned(),
            projection: None,
            query,
        }
    }
    ///Updates many documents in database
    pub fn update_many_documents(
        &self,
        coll: &str,
        filter: DataQuery,
        update: DataUpdate,
    ) -> request::UpdateManyDocumentsRequest {
        request::UpdateManyDocumentsRequest {
            http_client: &self,
            coll: coll.to_owned(),
            filter,
            update,
        }
    }
    ///Creates a transaction
    pub fn create_transaction(&self) -> request::CreateTransactionRequest {
        request::CreateTransactionRequest {
            http_client: &self,
        }
    }
    ///Commits a transaction
    pub fn commit_transaction(&self) -> request::CommitTransactionRequest {
        request::CommitTransactionRequest {
            http_client: &self,
        }
    }
    ///Aborts a transaction
    pub fn abort_transaction(&self) -> request::AbortTransactionRequest {
        request::AbortTransactionRequest {
            http_client: &self,
        }
    }
}
pub enum InternalApiAuthentication {
    BearerAuth { bearer_auth: String },
}
impl InternalApiAuthentication {
    pub fn from_env() -> Self {
        Self::BearerAuth {
            bearer_auth: std::env::var("INTERNAL_API_BEARER_AUTH")
                .expect("Environment variable INTERNAL_API_BEARER_AUTH is not set."),
        }
    }
}