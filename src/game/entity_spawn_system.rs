use hecs::World;
use macroquad::prelude::WHITE;

use crate::tileset::TileSet;

use super::components::base::{Player, Position, Renderable};

pub fn spawn_player(ecs: &mut World, position: (i32, i32)) {
    ecs.spawn((
        Player,
        Position {
            x: position.0,
            y: position.1,
        },
        Renderable {
            color: WHITE,
            sprite: TileSet::SPRITE_PLAYER,
        },
    ));
}
