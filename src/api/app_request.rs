use serde::{Deserialize, Serialize};
use typify::import_types;

// include!(concat!(env!("OUT_DIR"), "/app-request.rs"));
import_types!(
    schema = "api/app-request.schema.json",
);

#[cfg(test)]
mod test {

    use serde_json::Map;

    use super::*;

    #[test]
    fn simple_view() {
        let request: ViewRequest = ViewRequest {
            view: "test".into(),
            data: None,
            props: None,
            context: Map::new(),
        };
        assert_eq!("test", r#"{"type":"text","value":"test"}"#);
    }
}
