use serde::{Deserialize, Serialize};
use typify::import_types;

// include!(concat!(env!("OUT_DIR"), "/app-request.rs"));
import_types!(
    schema = "api/app-request.schema.json",
    struct_builder = true
);

#[cfg(test)]
mod test {
    use serde_json::Map;

    use super::*;

    #[test]
    fn simple_view() -> Result<(), String> {
        let request: AppRequest = AppRequest::ViewRequest(ViewRequest {
            view: "test".into(),
            data: None,
            props: None,
            context: Map::new(),
        });
        assert_eq!(
            format!(
                "{}",
                serde_json::to_string(&request).map_err(|err| err.to_string())?
            ),
            r#"{"view":"test"}"#
        );
        Ok(())
    }
}
