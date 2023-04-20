use serde::{Deserialize, Serialize};

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
