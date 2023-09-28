use crate::ComponentBuilder;

use self::lenra::{builder, Listener, ListenerName};

// pub mod json;
pub mod lenra;

pub fn listener(name: ListenerName) -> builder::Listener {
    Listener::builder().type_("listener").name(name)
}

impl ComponentBuilder<Listener> for builder::Listener {}
