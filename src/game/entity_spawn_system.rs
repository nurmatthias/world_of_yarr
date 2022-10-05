use macroquad::prelude::WHITE;
use specs::prelude::*;

use crate::{map::Map, tileset::TileSet};

use super::components::base::{Player, Position, Renderable};


#[derive(SystemData)]
pub struct SpawnData<'a> {
    entities: Entities<'a>,
    updater: Read<'a, LazyUpdate>,
    map: Read<'a, Map>,
}

pub struct SpawnPlayer;

impl<'a> System<'a> for SpawnPlayer {
    type SystemData = SpawnData<'a>;

    fn run(&mut self, data: Self::SystemData) {

        let player_pos = data.map.player_start_pos;

        let player = data.entities.create();
        data.updater.insert(player, Player);
        data.updater.insert(player,Position {
            x: player_pos.0,
            y: player_pos.1,
        });
        data.updater.insert(player,Renderable {
            color: WHITE,
            sprite: TileSet::SPRITE_PLAYER,
        });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        world.register::<Player>();
        world.register::<Position>();
        world.register::<Renderable>();
    }
}