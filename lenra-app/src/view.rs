use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::{
    components::lenra::LenraComponent, from_opt_value, HandleParams, Handler, NamedRequest,
    RequestHandler, Result,
};

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

pub trait ViewResponseGenerator: Serialize {
    fn gen(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl ViewResponseGenerator for Value {}

impl ViewResponseGenerator for LenraComponent {}

pub type ViewResponse = String;

pub type ViewBuilder = Box<dyn Fn(ViewRequest) -> Result<String>>;

pub struct View {
    name: String,
    build_fn: ViewBuilder,
}

impl RequestHandler<ViewRequest, ViewResponse> for View {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle(&self, request: ViewRequest) -> Result<ViewResponse> {
        (self.build_fn)(request)
    }

    fn create(name: &str, build_fn: ViewBuilder) -> Self {
        View {
            name: name.to_string(),
            build_fn,
        }
    }
}

impl Handler<ViewRequest, ViewResponse> for View {}

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

#[cfg(test)]
mod test {

    use serde_json::json;

    use crate::{components::{json::listener, lenra::*}, ComponentBuilder};

    use super::*;

    #[test]
    fn simple_lenra_view_from_component() {
        let view = super::View::create(
            "test",
            Box::new(|_| {
                let component: LenraComponent = text("test".into()).into();
                Ok(component.gen())
            }),
        );
        let request = ViewRequest {
            view: "test".into(),
            data: None,
            props: None,
            context: None,
        };
        assert_eq!(
            view.handle(request).unwrap(),
            r#"{"_type":"text","value":"test"}"#
        );
    }

    #[test]
    fn simple_json_view() {
        let view = super::View::create(
            "test",
            Box::new(|_| {
                let response = json!({
                    "myField": "test",
                    "onEvent": listener("name".try_into().unwrap()).build(),
                });
                Ok(response.gen())
            }),
        );
        let request = ViewRequest {
            view: "test".into(),
            data: None,
            props: None,
            context: None,
        };
        assert_eq!(
            view.handle(request).unwrap(),
            r#"{"myField":"test","onEvent":{"_type":"listener","name":"name"}}"#
        );
    }
}
