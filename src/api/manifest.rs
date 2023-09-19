use serde::{Deserialize, Serialize};
use typify::import_types;

// include!(concat!(env!("OUT_DIR"), "/manifest.rs"));
import_types!(
    schema = "api/manifest.schema.json",
);

#[cfg(test)]
mod test {

    use serde_json::Map;

    use super::*;

    #[test]
    fn simple_view() {
        let manifest: Manifest = Manifest {
            lenra: None,
            json: None,
        };
        assert_eq!("test", r#"{"type":"text","value":"test"}"#);
    }
}
