
use specs::prelude::*;

use crate::game::components::{base::{Player, Position, Renderable}, spawn::{EntitySpawnData, EntityType}, SpawnComponent};


#[derive(SystemData)]
pub struct SpawnData<'a> {
    entities: Entities<'a>,
    updater: Read<'a, LazyUpdate>,
    spawn_data: ReadStorage<'a, EntitySpawnData>,
}

pub struct SpawnEntity;

impl<'a> System<'a> for SpawnEntity {
    type SystemData = SpawnData<'a>;

    fn run(&mut self, data: Self::SystemData) {

        let mut to_remove = Vec::new();
        for (e, spawn_data) in (&data.entities, &data.spawn_data).join() {

            let spawn = data.entities.create();

            match spawn_data.entity_type {
                EntityType::Player => {
                    data.updater.insert(spawn, Player);
                }
            }
            
            spawn_data.components.iter().for_each(|c| {
                match c {
                    SpawnComponent::PositionComponent(comp) => data.updater.insert(spawn, *comp),
                    SpawnComponent::RenderableComponent(comp) => data.updater.insert(spawn, *comp),
                }
                
            });

            to_remove.push(e);
        }

        to_remove.iter().for_each(|e| { data.entities.delete(*e).expect("can't remove spawndata");});
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        world.register::<Player>();
        world.register::<Position>();
        world.register::<Renderable>();
    }
}