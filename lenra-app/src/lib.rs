use std::{
    error::Error,
    fmt::{self, Debug},
    io::{self, Write},
    str,
};

use listener::{Listener, ListenerRequest};
use log::{error, warn};
use manifest::Manifest;
use resource::{Resource, ResourceRequest};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use view::{View, ViewRequest};

pub mod api;
pub mod components;
pub mod listener;
pub mod manifest;
pub mod resource;
pub mod view;

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

// Macros

#[macro_export]
macro_rules! from_value {
    ($name:ident) => {
        impl Into<$name> for serde_json::Value {
            fn into(self) -> $name {
                serde_json::from_value(self).unwrap()
            }
        }
    };
}

#[macro_export]
macro_rules! props {
    ($name:ident) => {
        impl From<$name> for lenra_app::components::lenra::DefsProps {
            fn from(value: $name) -> lenra_app::components::lenra::DefsProps {
                serde_json::to_value(value).unwrap().into()
            }
        }
        impl From<$name> for lenra_app::manifest::DefsProps {
            fn from(value: $name) -> lenra_app::manifest::DefsProps {
                serde_json::to_value(value).unwrap().into()
            }
        }
    };
}

// Components

#[derive(Default)]
pub struct LenraApp {
    pub manifest: Manifest,
    pub views: Vec<View>,
    pub listeners: Vec<Listener>,
    pub resources: Vec<Resource>,
}

impl LenraApp {
    pub fn run(self) -> Result<()> {
        env_logger::init();

        let request = serde_json::from_reader(std::io::stdin());
        self.handle(
            request.unwrap_or(Request::Other(Value::Null)),
            &mut io::stdout(),
        )
    }

    pub(crate) fn handle<W: Write>(self, request: Request, writer: &mut W) -> Result<()> {
        match request {
            Request::View(view) => self.handle_view(view, writer)?,
            Request::Listener(listener) => self.handle_listener(listener, writer)?,
            Request::Resource(resource) => self.handle_resource(resource, writer)?,
            Request::Other(req) => {
                if req != Value::Null {
                    warn!("Not managed request: {}", req);
                }
                write!(writer, "{}", serde_json::to_string(&self.manifest)?)?;
            }
        };
        Ok(())
    }

    fn handle_view<W: Write>(self, request: ViewRequest, writer: &mut W) -> Result<()> {
        let opt = self.views.iter().find(|&v| v.name() == request.name());
        if let Some(view) = opt {
            let result = view.handle(request)?;
            write!(writer, "{}", result)?;
        } else {
            let message = format!("No view found for {}", request.name());
            error!("{}", message);
            return Err(Box::new(CustomError { message }));
        };
        Ok(())
    }

    fn handle_listener<W: Write>(self, request: ListenerRequest, _writer: &mut W) -> Result<()> {
        let opt = self.listeners.iter().find(|&v| v.name() == request.name());
        if let Some(listener) = opt {
            listener.handle(request)
        } else {
            let message = format!("No listener found for {}", request.name());
            error!("{}", message);
            Err(Box::new(CustomError { message }))
        }
    }

    fn handle_resource<W: Write>(self, request: ResourceRequest, writer: &mut W) -> Result<()> {
        let opt = self.resources.iter().find(|&v| v.name() == request.name());
        if let Some(resource) = opt {
            let result = resource.handle(request)?;
            writer.write_all(&result)?;
        } else {
            let message = format!("No resource found for {}", request.name());
            error!("{}", message);
            return Err(Box::new(CustomError { message }));
        };
        Ok(())
    }
}

pub trait HandleParams<R> {
    fn from_request(request: R) -> Self;
}

pub trait NamedRequest {
    fn name(&self) -> String;
}

pub trait RequestHandler<Req, Res>: Sized
where
    Req: NamedRequest,
{
    fn name(&self) -> String;
    fn handle(&self, request: Req) -> Result<Res>;
    fn create(name: &str, build_fn: Box<dyn Fn(Req) -> Result<Res>>) -> Self;
}

pub trait Handler<Req, Res>: RequestHandler<Req, Res>
where
    Req: NamedRequest,
{
    fn new<P, F>(name: &str, build_fn: F) -> Self
    where
        P: HandleParams<Req>,
        F: Fn(P) -> Result<Res> + 'static,
    {
        let boxed_fn: Box<dyn Fn(Req) -> Result<Res>> =
            Box::new(move |request: Req| build_fn(P::from_request(request)));
        Self::create(name, boxed_fn)
    }
}

pub(crate) fn from_opt_value<T>(opt: Option<Value>) -> Result<Option<T>>
where
    T: DeserializeOwned + 'static,
{
    Ok(match opt {
        Some(value) => match value.clone() {
            Value::Null => None,
            Value::Object(obj) => {
                if obj.is_empty() {
                    None
                } else {
                    Some(
                        serde_json::from_value(value)
                            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?,
                    )
                }
            }
            _ => Some(
                serde_json::from_value(value)
                    .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?,
            ),
        },
        None => None,
    })
}

/** The application input */
#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum Request {
    View(ViewRequest),
    Listener(ListenerRequest),
    Resource(ResourceRequest),
    Other(Value),
}

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub trait ComponentBuilder<T>: Sized + Clone + std::fmt::Debug
where
    T: serde::ser::Serialize + std::convert::TryFrom<Self>,
    T::Error: std::fmt::Display + std::fmt::Debug,
{
    fn build(self) -> T {
        T::try_from(self).unwrap()
    }
}

#[cfg(test)]
mod test {

    use serde_json::json;

    use crate::view::{ViewParams, ViewResponseGenerator};

    use super::*;

    #[test]
    fn simple_view() {
        let app = LenraApp {
            // manifest: Manifest {
            //     root_view: "test".into(),
            // },
            views: vec![View::new("test", |_: ViewParams| {
                Ok(json!({"type": "text", "value": "test"}).gen())
            })],
            ..Default::default()
        };
        let request = ViewRequest {
            view: "test".into(),
            data: None,
            props: None,
            context: None,
        };
        let mut buf = Vec::new();
        app.handle(Request::View(request), &mut buf).unwrap();
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(result, r#"{"type":"text","value":"test"}"#);
    }

    #[test]
    #[should_panic]
    fn unkown_view() {
        let app = LenraApp {
            // manifest: Manifest {
            //     root_view: "test".into(),
            // },
            views: vec![View::new("test", |_: ViewParams| {
                Ok(json!({"type": "text", "value": "test"}).gen())
            })],
            ..Default::default()
        };
        let request = ViewRequest {
            view: "test2".into(),
            data: None,
            props: None,
            context: None,
        };
        let mut buf = Vec::new();
        app.handle(Request::View(request), &mut buf).unwrap();
    }
}
