use serde::{Deserialize, Serialize};
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
pub struct Listener {
    ///The name of the listener to call
    pub name: ListenerName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<DefsProps>,
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&Listener> for Listener {
    fn from(value: &Listener) -> Self {
        value.clone()
    }
}
impl Listener {
    pub fn builder() -> builder::Listener {
        builder::Listener::default()
    }
}
///The name of the listener to call
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ListenerName(String);
impl std::ops::Deref for ListenerName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ListenerName> for String {
    fn from(value: ListenerName) -> Self {
        value.0
    }
}
impl From<&ListenerName> for ListenerName {
    fn from(value: &ListenerName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ListenerName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^(@lenra:)?[a-zA-Z_$][a-zA-Z_.$0-9]*$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^(@lenra:)?[a-zA-Z_$][a-zA-Z_.$0-9]*$\"",
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for ListenerName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ListenerName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ListenerName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for ListenerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Listener {
        name: Result<super::ListenerName, String>,
        props: Result<Option<super::DefsProps>, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for Listener {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                props: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Listener {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ListenerName>,
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
    impl std::convert::TryFrom<Listener> for super::Listener {
        type Error = String;
        fn try_from(value: Listener) -> Result<Self, String> {
            Ok(Self {
                name: value.name?,
                props: value.props?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Listener> for Listener {
        fn from(value: super::Listener) -> Self {
            Self {
                name: Ok(value.name),
                props: Ok(value.props),
                type_: Ok(value.type_),
            }
        }
    }
}


pub fn listener(name: ListenerName) -> builder::Listener {
    Listener::builder()
        .type_("listener")
        .name(name)
}