// use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::{
    api::{Api, ApiParam},
    from_opt_value, HandleParams, Handler, NamedRequest, RequestHandler, Result,
};

/** Lenra listener request */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub(crate) struct ListenerRequest {
    pub action: String,
    pub props: Option<Value>,
    pub event: Option<Value>,
    pub api: ApiParam,
}

impl NamedRequest for ListenerRequest {
    fn name(&self) -> String {
        self.action.clone()
    }
}

pub struct ListenerParams<P, E>
where
    P: DeserializeOwned + 'static,
    E: DeserializeOwned + 'static,
{
    pub props: Option<P>,
    pub event: Option<E>,
    pub api: Api,
}

impl<P, E> HandleParams<ListenerRequest> for ListenerParams<P, E>
where
    P: DeserializeOwned + 'static,
    E: DeserializeOwned + 'static,
{
    fn from_request(request: ListenerRequest) -> Self {
        ListenerParams {
            props: from_opt_value(request.props).unwrap(),
            event: from_opt_value(request.event).unwrap(),
            api: Api::new(request.api),
        }
    }
}

pub struct Listener {
    name: String,
    build_fn: Box<dyn Fn(ListenerRequest) -> Result<()>>,
}

impl RequestHandler<ListenerRequest, ()> for Listener {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle(&self, request: ListenerRequest) -> Result<()> {
        (self.build_fn)(request)
    }

    fn create(name: &str, build_fn: Box<dyn Fn(ListenerRequest) -> Result<()>>) -> Self {
        Listener {
            name: name.to_string(),
            build_fn,
        }
    }
}

impl Handler<ListenerRequest, ()> for Listener {}
