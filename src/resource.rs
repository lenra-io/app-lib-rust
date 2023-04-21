use serde::{Deserialize, Serialize};

use crate::{HandleParams, Handler, NamedRequest, RequestHandler, Result};

#[macro_export]
macro_rules! resource {
    ($path: tt) => {
        ($path, include_bytes!($path))
    };
}

/** Lenra view request */
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResourceRequest {
    pub resource: String,
}

impl NamedRequest for ResourceRequest {
    fn name(&self) -> String {
        self.resource.clone()
    }
}

pub struct ResourceParams;

impl HandleParams<ResourceRequest> for ResourceParams {
    fn from_request(_request: ResourceRequest) -> Self {
        ResourceParams
    }
}

pub struct Resource {
    name: String,
    build_fn: Box<dyn Fn(ResourceRequest) -> Result<Vec<u8>>>,
}

impl RequestHandler<ResourceRequest, Vec<u8>> for Resource {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle(&self, request: ResourceRequest) -> Result<Vec<u8>> {
        (self.build_fn)(request)
    }

    fn create(name: &str, build_fn: Box<dyn Fn(ResourceRequest) -> Result<Vec<u8>>>) -> Self {
        Resource {
            name: name.to_string(),
            build_fn,
        }
    }
}

impl Handler<ResourceRequest, Vec<u8>> for Resource {}
