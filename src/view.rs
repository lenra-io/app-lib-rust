use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::Result;

/** Unknown view request */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ViewRequest {
    pub view: String,
    pub data: Option<Value>,
    pub props: Option<Value>,
    pub context: Option<Context>,
}

impl ViewRequest {
    pub fn name(&self) -> String {
        self.view.clone()
    }
}

#[derive(Serialize)]
pub struct ViewParams<D, P>
where
    D: DeserializeOwned + 'static,
    P: DeserializeOwned + 'static,
{
    pub data: Option<D>,
    pub props: Option<P>,
    pub context: Option<Context>,
}

pub struct View {
    name: String,
    build_fn: Box<dyn Fn(ViewRequest) -> Result<Value>>,
}

impl View {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn new<D, P, R, F>(name: String, build_fn: F) -> Self
    where
        D: DeserializeOwned + 'static,
        P: DeserializeOwned + 'static,
        R: DeserializeOwned + Serialize + 'static,
        F: Fn(ViewParams<D, P>) -> Result<R> + 'static,
    {
        let boxed_fn: Box<dyn Fn(ViewRequest) -> Result<Value>> =
            Box::new(move |request: ViewRequest| {
                let result = build_fn(ViewParams {
                    data: from_opt_value(request.data)?,
                    props: from_opt_value(request.props)?,
                    context: request.context,
                });
                match result {
                    Ok(res) => serde_json::to_value(res)
                        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>),
                    Err(e) => Err(e),
                }
            });
        View {
            name,
            build_fn: boxed_fn,
        }
    }

    pub(crate) fn build(&self, request: ViewRequest) -> Result<Value> {
        (self.build_fn)(request)
    }
}

fn from_opt_value<T>(opt: Option<Value>) -> Result<Option<T>>
where
    T: DeserializeOwned + 'static,
{
    Ok(match opt {
        Some(value) => Some(
            serde_json::from_value(value)
                .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?,
        ),
        None => None,
    })
}

pub trait ViewBuilder {
    fn build<D: DeserializeOwned, P: DeserializeOwned, R: DeserializeOwned>(
        data: Option<D>,
        properties: Option<P>,
    ) -> Result<R>;
}

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
    dx: u16,
    dy: u16,
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
