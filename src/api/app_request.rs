// use serde::{Deserialize, Serialize};
// use typify::import_types;
// import_types!(
//     schema = "api/app-request.schema.json",
//     struct_builder = true
// );

include!(concat!(env!("OUT_DIR"), "/requests_app.rs"));

#[cfg(test)]
mod test {
    use serde_json::Map;

    use super::*;

    #[test]
    fn simple_view() -> Result<(), String> {
        let request: AppRequest =
            AppRequest::View(View::builder().view("test").try_into().unwrap());
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
