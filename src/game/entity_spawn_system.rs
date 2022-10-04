use macroquad::prelude::WHITE;
use specs::prelude::*;

use crate::{map::Map, tileset::TileSet};

use super::components::base::{Player, Position, Renderable};

pub fn spawn_player(ecs: &mut World) {
    let p_pos = {
        let map = ecs.read_resource::<Map>();
        map.player_start_pos.clone()
    };

    ecs.create_entity()
        .with(Player)
        .with(Position {
            x: p_pos.0,
            y: p_pos.1,
        })
        .with(Renderable {
            color: WHITE,
            sprite: TileSet::SPRITE_PLAYER,
        })
        .build();
}
