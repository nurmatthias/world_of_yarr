use macroquad::prelude::WHITE;
use specs::prelude::*;

use crate::{map::Map, tileset::TileSet};

use super::components::base::*;

pub fn spawn_player(world: &mut World) {
    let start_pos = {
        let map = world.read_resource::<Map>();
        map.player_start_pos
    };

    world
        .create_entity()
        .with(Player)
        .with(Position(start_pos))
        .with(Renderable {
            color: WHITE,
            sprite: TileSet::SPRITE_PLAYER,
        })
        .build();
}
