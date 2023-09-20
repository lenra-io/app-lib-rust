// use serde::{Deserialize, Serialize};
// use typify::import_types;
// import_types!(
//     schema = "api/view-response.schema.json",
//     struct_builder = true
// );

include!(concat!(env!("OUT_DIR"), "/view-response.rs"));

#[cfg(test)]
mod test {

    use serde_json::Map;

    use super::*;

    #[test]
    fn simple_view() {
        let request: ViewResponse = ViewResponse {
            type_: "text".into(),
            value: "test".into(),
        };
        assert_eq!("test", r#"{"type":"text","value":"test"}"#);
    }
}
