use crate::{ComponentBuilder, from_value};

include!("../gen/components_listener.rs");

impl ComponentBuilder<Listener> for builder::Listener {}


from_value!(DefsProps);
