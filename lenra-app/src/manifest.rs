use crate::{components::lenra, from_value};

include!("gen/manifest.rs");

impl Default for Manifest {
    fn default() -> Self {
        Manifest::builder().try_into().unwrap()
    }
}

impl Into<ComponentsView> for lenra::builder::View {
    fn into(self) -> ComponentsView {
        let view: lenra::View = self.try_into().unwrap();
        ComponentsView {
            context: view.context,
            find: view.find.map(|find| find.into()),
            name: view.name,
            props: view.props.map(|props| props.into()),
            type_: view.type_,
        }
    }
}

impl Into<DefsProps> for lenra::DefsProps {
    fn into(self) -> DefsProps {
        DefsProps(self.0)
    }
}

impl Into<DataQuery> for lenra::DataQuery {
    fn into(self) -> DataQuery {
        DataQuery(self.0)
    }
}

impl Into<DataProjection> for lenra::DataProjection {
    fn into(self) -> DataProjection {
        DataProjection(self.0)
    }
}

impl Into<ComponentsViewDefinitionsFind> for lenra::ViewDefinitionsFind {
    fn into(self) -> ComponentsViewDefinitionsFind {
        ComponentsViewDefinitionsFind {
            coll: self.coll,
            query: self.query.into(),
            projection: self.projection.map(|projection| projection.into()),
        }
    }
}

from_value!(DefsProps);
from_value!(DataQuery);
from_value!(DataProjection);

#[cfg(test)]
mod test {
    use std::vec;

    use serde_json::{json, Map, Value};

    use crate::components::lenra::{self, view, ViewDefinitionsFind};

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
    fn from_lenra_view() -> Result<(), String> {
        let manifest: Manifest = Manifest::builder()
            .json(Some(
                Exposer::builder()
                    .routes(vec![Route::builder()
                        .path("/counter/global")
                        .view(
                            view("counter").find(Some(
                                ViewDefinitionsFind::builder()
                                    .coll("counter")
                                    .query(lenra::DataQuery::from(Map::from_iter(vec![(
                                        "user".to_string(),
                                        Value::String("global".into()),
                                    )])))
                                    .try_into()
                                    .unwrap(),
                            )),
                        )
                        .try_into()?])
                    .try_into()?,
            ))
            .try_into()?;

        assert_eq!(
            format!(
                "{}",
                serde_json::to_string(&manifest).map_err(|er| er.to_string())?
            ),
            r#"{"json":{"routes":[{"path":"/counter/global","view":{"find":{"coll":"counter","query":{"user":"global"}},"name":"counter","_type":"view"}}]}}"#
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
