use crate::ComponentBuilder;

use self::lenra::{builder, Listener, ListenerName};

// pub mod json;
pub mod lenra;

pub fn listener<T>(name: T) -> builder::Listener
where
    T: std::convert::TryInto<ListenerName>,
    T::Error: std::fmt::Display,
{
    Listener::builder().type_("listener").name(name)
}

impl ComponentBuilder<Listener> for builder::Listener {}
