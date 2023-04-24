use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub root_view: String,
}

impl Manifest {
    pub(crate) fn to_value(&self) -> Value {
        json!({ "manifest": self })
    }
}
