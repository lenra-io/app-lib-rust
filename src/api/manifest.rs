// use serde::{Deserialize, Serialize};
// use typify::import_types;

// import_types!(schema = "api/manifest.schema.json", struct_builder = true);

include!(concat!(env!("OUT_DIR"), "/manifest.rs"));

impl Default for Manifest {
    fn default() -> Self {
        Manifest::builder().try_into().unwrap()
    }
}

#[cfg(test)]
mod test {
    use serde_json::{json, Map, Value};

    use super::*;

    fn path_user_to_counter_route((path, user): (&str, &str)) -> Result<Route, String> {
        let query: DataQuery =
            Map::from_iter([("user".to_string(), Value::String(user.into()))]).into();
        let find: ComponentsViewDefinitionsFind = ComponentsViewDefinitionsFind::builder()
            .coll("counter")
            .query(query)
            .try_into()?;
        Route::builder()
            .path(path)
            .view(
                ComponentsView::builder()
                    .type_("view")
                    .name("counter")
                    .find(find),
            )
            .try_into()
    }

    #[test]
    fn only_json_routes() -> Result<(), String> {
        let manifest: Manifest = Manifest::builder()
            .json(Some(
                Exposer::builder()
                    .routes(
                        vec![("/counter/global", "global"), ("/counter/me", "@me")]
                            .iter()
                            .map(|&(path, user)| path_user_to_counter_route((path, user)))
                            .collect::<Result<Vec<Route>, String>>()?,
                    )
                    .try_into()?,
            ))
            .try_into()?;

        assert_eq!(
            format!(
                "{}",
                serde_json::to_string(&manifest).map_err(|er| er.to_string())?
            ),
            r#"{"json":{"routes":[{"path":"/counter/global","view":{"find":{"coll":"counter","query":{"user":"global"}},"name":"counter","_type":"view"}},{"path":"/counter/me","view":{"find":{"coll":"counter","query":{"user":"@me"}},"name":"counter","_type":"view"}}]}}"#
        );
        Ok(())
    }

    #[test]
    fn from_string() -> Result<(), String> {
        let manifest: Manifest = serde_json::from_str(
            r#"{"json":{"routes":[{"path":"/counter/global","view":{"find":{"coll":"counter","query":{"user":"global"}},"name":"counter","_type":"view"}},{"path":"/counter/me","view":{"find":{"coll":"counter","query":{"user":"@me"}},"name":"counter","_type":"view"}}]}}"#,
        ).map_err(|er| er.to_string())?;

        assert_eq!(
            format!(
                "{}",
                serde_json::to_string(&manifest).map_err(|er| er.to_string())?
            ),
            r#"{"json":{"routes":[{"path":"/counter/global","view":{"find":{"coll":"counter","query":{"user":"global"}},"name":"counter","_type":"view"}},{"path":"/counter/me","view":{"find":{"coll":"counter","query":{"user":"@me"}},"name":"counter","_type":"view"}}]}}"#
        );
        Ok(())
    }

    #[test]
    fn from_json_definition() -> Result<(), String> {
        let value: Value = json!(
            {
                "json": {
                    "routes": [
                        {
                            "path": "/counter/global",
                            "view": {
                                "_type": "view",
                                "name": "counter",
                                "find": {
                                    "coll": "counter",
                                    "query": {
                                        "user": "global"
                                    }
                                }
                            }
                        },
                        {
                            "path": "/counter/me",
                            "view": {
                                "_type": "view",
                                "name": "counter",
                                "find": {
                                    "coll": "counter",
                                    "query": {
                                        "user": "@me"
                                    }
                                }
                            }
                        }
                    ]
                }
            }
        );
        let manifest: Manifest = serde_json::from_value(value).map_err(|er| er.to_string())?;

        assert_eq!(
            format!(
                "{}",
                serde_json::to_string(&manifest).map_err(|er| er.to_string())?
            ),
            r#"{"json":{"routes":[{"path":"/counter/global","view":{"find":{"coll":"counter","query":{"user":"global"}},"name":"counter","_type":"view"}},{"path":"/counter/me","view":{"find":{"coll":"counter","query":{"user":"@me"}},"name":"counter","_type":"view"}}]}}"#
        );
        Ok(())
    }
}
