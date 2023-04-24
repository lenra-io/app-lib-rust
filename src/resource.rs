use serde::{Deserialize, Serialize};

use crate::{HandleParams, Handler, NamedRequest, RequestHandler, Result};

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

pub fn map_resources<const N: usize>(resources: [(&str, &[u8]); N]) -> Vec<Resource> {
    let mut ret = vec![];
    resources.iter().for_each(|res| ret.push(map_resource(res)));
    ret
}

fn map_resource(&resource: &(&str, &[u8])) -> Resource {
    let (name, bytes) = resource;
    let vec = bytes.to_vec();
    Resource::create(
        name,
        Box::new(move |_request: ResourceRequest| Ok(vec.clone())),
    )
}
