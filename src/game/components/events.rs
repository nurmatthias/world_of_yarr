use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct RequestMove {
    pub entity: Entity,
    pub request: (i32, i32),
}

#[derive(Component, Debug, Clone, Copy)]
pub struct LocationMove {
    pub entity: Entity,
    pub location: (i32, i32),
}
