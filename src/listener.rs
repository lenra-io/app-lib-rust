// use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::{Api, ApiParam};

/** Lenra listener request */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub(crate) struct ListenerRequest {
    pub action: String,
    pub props: Option<Value>,
    pub event: Option<Value>,
    pub api: ApiParam,
}

pub type ListenerHandler<P, E> = dyn Fn(Option<P>, Option<E>, Api);
