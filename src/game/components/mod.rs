pub mod base;


use specs::prelude::*;
use self::base::*;

pub fn register_components(world: &mut World) {

    world.register::<Player>();
    world.register::<Position>();
    world.register::<Renderable>();

}