use specs::prelude::*;
use specs_derive::*;

use super::SpawnComponent;


#[derive(Debug)]
pub enum EntityType {
    Player,
}

#[derive(Component, Debug)]
pub struct EntitySpawnData {
    pub entity_type: EntityType,
    pub components: Vec<SpawnComponent>
}