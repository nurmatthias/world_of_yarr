pub mod base;
pub mod events;


use specs::prelude::*;

pub fn register_components(world: &mut World) {

    world.register::<base::Player>();
    world.register::<base::Position>();
    world.register::<base::Renderable>();

    world.register::<events::WantToMove>();

}