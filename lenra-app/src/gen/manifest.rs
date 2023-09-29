use serde::{Deserialize, Serialize};
///Element of type view
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentsView {
    ///The context projection. This field represents the projection of the context, allowing selective retrieval of specific elements. It is a map that specifies the desired elements to be included in the projection.
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub context: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub find: Option<ComponentsViewDefinitionsFind>,
    ///The name of the view
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<DefsProps>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&ComponentsView> for ComponentsView {
    fn from(value: &ComponentsView) -> Self {
        value.clone()
    }
}
impl ComponentsView {
    pub fn builder() -> builder::ComponentsView {
        builder::ComponentsView::default()
    }
}
///Find query for view components
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentsViewDefinitionsFind {
    ///the collection where the query is applied
    pub coll: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projection: Option<DataProjection>,
    pub query: DataQuery,
}
impl From<&ComponentsViewDefinitionsFind> for ComponentsViewDefinitionsFind {
    fn from(value: &ComponentsViewDefinitionsFind) -> Self {
        value.clone()
    }
}
impl ComponentsViewDefinitionsFind {
    pub fn builder() -> builder::ComponentsViewDefinitionsFind {
        builder::ComponentsViewDefinitionsFind::default()
    }
}
///Mongo data query projection
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataProjection(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for DataProjection {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<DataProjection> for serde_json::Map<String, serde_json::Value> {
    fn from(value: DataProjection) -> Self {
        value.0
    }
}
impl From<&DataProjection> for DataProjection {
    fn from(value: &DataProjection) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for DataProjection {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
///Mongo data query
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataQuery(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for DataQuery {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<DataQuery> for serde_json::Map<String, serde_json::Value> {
    fn from(value: DataQuery) -> Self {
        value.0
    }
}
impl From<&DataQuery> for DataQuery {
    fn from(value: &DataQuery) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for DataQuery {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
///Parameters passed to the listener
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DefsProps(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for DefsProps {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<DefsProps> for serde_json::Map<String, serde_json::Value> {
    fn from(value: DefsProps) -> Self {
        value.0
    }
}
impl From<&DefsProps> for DefsProps {
    fn from(value: &DefsProps) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for DefsProps {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Exposer {
    ///The routes of the application for this exposer
    pub routes: Vec<Route>,
    ///The exposer API version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl From<&Exposer> for Exposer {
    fn from(value: &Exposer) -> Self {
        value.clone()
    }
}
impl Exposer {
    pub fn builder() -> builder::Exposer {
        builder::Exposer::default()
    }
}
///The Lenra application manifest
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    ///The JSON exposer definition of the application
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json: Option<Exposer>,
    ///The Lenra exposer definition of the application
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lenra: Option<Exposer>,
}
impl From<&Manifest> for Manifest {
    fn from(value: &Manifest) -> Self {
        value.clone()
    }
}
impl Manifest {
    pub fn builder() -> builder::Manifest {
        builder::Manifest::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Route {
    pub path: String,
    pub view: ComponentsView,
}
impl From<&Route> for Route {
    fn from(value: &Route) -> Self {
        value.clone()
    }
}
impl Route {
    pub fn builder() -> builder::Route {
        builder::Route::default()
    }
}
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ComponentsView {
        context: Result<serde_json::Map<String, serde_json::Value>, String>,
        find: Result<Option<super::ComponentsViewDefinitionsFind>, String>,
        name: Result<String, String>,
        props: Result<Option<super::DefsProps>, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for ComponentsView {
        fn default() -> Self {
            Self {
                context: Ok(Default::default()),
                find: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                props: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ComponentsView {
        pub fn context<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self
                .context = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for context: {}", e)
                });
            self
        }
        pub fn find<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ComponentsViewDefinitionsFind>>,
            T::Error: std::fmt::Display,
        {
            self
                .find = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for find: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn props<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DefsProps>>,
            T::Error: std::fmt::Display,
        {
            self
                .props = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for props: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ComponentsView> for super::ComponentsView {
        type Error = String;
        fn try_from(value: ComponentsView) -> Result<Self, String> {
            Ok(Self {
                context: value.context?,
                find: value.find?,
                name: value.name?,
                props: value.props?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ComponentsView> for ComponentsView {
        fn from(value: super::ComponentsView) -> Self {
            Self {
                context: Ok(value.context),
                find: Ok(value.find),
                name: Ok(value.name),
                props: Ok(value.props),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ComponentsViewDefinitionsFind {
        coll: Result<String, String>,
        projection: Result<Option<super::DataProjection>, String>,
        query: Result<super::DataQuery, String>,
    }
    impl Default for ComponentsViewDefinitionsFind {
        fn default() -> Self {
            Self {
                coll: Err("no value supplied for coll".to_string()),
                projection: Ok(Default::default()),
                query: Err("no value supplied for query".to_string()),
            }
        }
    }
    impl ComponentsViewDefinitionsFind {
        pub fn coll<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .coll = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coll: {}", e));
            self
        }
        pub fn projection<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DataProjection>>,
            T::Error: std::fmt::Display,
        {
            self
                .projection = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for projection: {}", e)
                });
            self
        }
        pub fn query<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DataQuery>,
            T::Error: std::fmt::Display,
        {
            self
                .query = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for query: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ComponentsViewDefinitionsFind>
    for super::ComponentsViewDefinitionsFind {
        type Error = String;
        fn try_from(value: ComponentsViewDefinitionsFind) -> Result<Self, String> {
            Ok(Self {
                coll: value.coll?,
                projection: value.projection?,
                query: value.query?,
            })
        }
    }
    impl From<super::ComponentsViewDefinitionsFind> for ComponentsViewDefinitionsFind {
        fn from(value: super::ComponentsViewDefinitionsFind) -> Self {
            Self {
                coll: Ok(value.coll),
                projection: Ok(value.projection),
                query: Ok(value.query),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Exposer {
        routes: Result<Vec<super::Route>, String>,
        version: Result<Option<String>, String>,
    }
    impl Default for Exposer {
        fn default() -> Self {
            Self {
                routes: Err("no value supplied for routes".to_string()),
                version: Ok(Default::default()),
            }
        }
    }
    impl Exposer {
        pub fn routes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Route>>,
            T::Error: std::fmt::Display,
        {
            self
                .routes = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for routes: {}", e)
                });
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Exposer> for super::Exposer {
        type Error = String;
        fn try_from(value: Exposer) -> Result<Self, String> {
            Ok(Self {
                routes: value.routes?,
                version: value.version?,
            })
        }
    }
    impl From<super::Exposer> for Exposer {
        fn from(value: super::Exposer) -> Self {
            Self {
                routes: Ok(value.routes),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Manifest {
        json: Result<Option<super::Exposer>, String>,
        lenra: Result<Option<super::Exposer>, String>,
    }
    impl Default for Manifest {
        fn default() -> Self {
            Self {
                json: Ok(Default::default()),
                lenra: Ok(Default::default()),
            }
        }
    }
    impl Manifest {
        pub fn json<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Exposer>>,
            T::Error: std::fmt::Display,
        {
            self
                .json = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for json: {}", e));
            self
        }
        pub fn lenra<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Exposer>>,
            T::Error: std::fmt::Display,
        {
            self
                .lenra = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for lenra: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Manifest> for super::Manifest {
        type Error = String;
        fn try_from(value: Manifest) -> Result<Self, String> {
            Ok(Self {
                json: value.json?,
                lenra: value.lenra?,
            })
        }
    }
    impl From<super::Manifest> for Manifest {
        fn from(value: super::Manifest) -> Self {
            Self {
                json: Ok(value.json),
                lenra: Ok(value.lenra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Route {
        path: Result<String, String>,
        view: Result<super::ComponentsView, String>,
    }
    impl Default for Route {
        fn default() -> Self {
            Self {
                path: Err("no value supplied for path".to_string()),
                view: Err("no value supplied for view".to_string()),
            }
        }
    }
    impl Route {
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn view<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ComponentsView>,
            T::Error: std::fmt::Display,
        {
            self
                .view = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for view: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Route> for super::Route {
        type Error = String;
        fn try_from(value: Route) -> Result<Self, String> {
            Ok(Self {
                path: value.path?,
                view: value.view?,
            })
        }
    }
    impl From<super::Route> for Route {
        fn from(value: super::Route) -> Self {
            Self {
                path: Ok(value.path),
                view: Ok(value.view),
            }
        }
    }
}
