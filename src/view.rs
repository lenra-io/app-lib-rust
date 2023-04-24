use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::{from_opt_value, HandleParams, Handler, NamedRequest, RequestHandler, Result};

/** Unknown view request */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ViewRequest {
    pub view: String,
    pub data: Option<Value>,
    pub props: Option<Value>,
    pub context: Option<Context>,
}

impl NamedRequest for ViewRequest {
    fn name(&self) -> String {
        self.view.clone()
    }
}

pub struct ViewParams<D = Value, P = Value>
where
    D: DeserializeOwned + 'static,
    P: DeserializeOwned + 'static,
{
    pub data: Option<D>,
    pub props: Option<P>,
    pub context: Option<Context>,
}

impl<D, P> HandleParams<ViewRequest> for ViewParams<D, P>
where
    D: DeserializeOwned + 'static,
    P: DeserializeOwned + 'static,
{
    fn from_request(request: ViewRequest) -> Self {
        ViewParams {
            data: from_opt_value(request.data).unwrap(),
            props: from_opt_value(request.props).unwrap(),
            context: request.context,
        }
    }
}

pub struct View {
    name: String,
    build_fn: Box<dyn Fn(ViewRequest) -> Result<Value>>,
}

impl RequestHandler<ViewRequest, Value> for View {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle(&self, request: ViewRequest) -> Result<Value> {
        (self.build_fn)(request)
    }

    fn create(name: &str, build_fn: Box<dyn Fn(ViewRequest) -> Result<Value>>) -> Self {
        View {
            name: name.to_string(),
            build_fn,
        }
    }
}

impl Handler<ViewRequest, Value> for View {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub screen_size: Option<ScreenSize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ScreenSize {
    pub width: Option<u16>,
    pub height: Option<u16>,
}

/** Lenra view padding */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Padding {
    pub top: u16,
    pub bottom: u16,
    pub left: u16,
    pub right: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Decoration {
    pub color: Option<u32>,
    pub box_shadow: Option<BoxShadow>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BoxShadow {
    pub blur_radius: Option<u16>,
    pub color: Option<u32>,
    pub offset: Option<Offset>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Offset {
    pub dx: u16,
    pub dy: u16,
}

impl Padding {
    pub fn symmetric(vertical: u16, horizontal: u16) -> Padding {
        Padding {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }
}
