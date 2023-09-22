// use serde::{Deserialize, Serialize};
// use typify::import_types;
// import_types!(
//     schema = "api/view_response.schema.json",
//     struct_builder = true
// );

include!(concat!(env!("OUT_DIR"), "/components_lenra.rs"));

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use serde_json::{json, Map};

    use super::*;

    #[test]
    fn simple_view() {
        let icon: Icon = Icon::builder()
            .type_("icon")
            .value(StylesIconName::AcUnit)
            .try_into()
            .unwrap();
        assert_eq!(
            serde_json::to_string(&icon).unwrap(),
            r#"{"_type":"icon","value":"ac_unit"}"#
        );
    }
}
