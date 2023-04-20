use std::error::Error;

use listener::ListenerRequest;
use log::{error, warn};
use manifest::Manifest;
use resource::ResourceRequest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use view::{View, ViewRequest};

pub mod api;
pub mod listener;
pub mod manifest;
pub mod resource;
pub mod view;

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

pub fn run(manifest: Manifest, views: Vec<View>) -> Result<()> {
    env_logger::init();

    let body = serde_json::from_reader(std::io::stdin());
    if let Ok(request) = body {
        match request {
            Request::View(view_req) => {
                let opt = views.iter().find(|&v| v.name() == view_req.name());
                match opt {
                    Some(view) => print!("{}", view.build(view_req)?),
                    None => error!("No view found for {}", view_req.name()),
                }
            }
            // Request::Listener(listener) => listener.handle(),
            // Request::Resource(resource) => resource.handle(),
            Request::Other(req) => {
                warn!("Not managed request: {}", req);
                print!("{}", manifest.to_value())
            }
        }
    } else {
        print!("{}", manifest.to_value());
    }
    Ok(())
}

/** The application input */
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum Request {
    View(ViewRequest),
    // Listener(ListenerRequest),
    // Resource(ResourceRequest),
    Other(Value),
}
