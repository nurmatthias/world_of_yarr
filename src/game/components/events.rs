use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct WantToMove {
    pub entity: Entity,
    pub target: (i32, i32),
}