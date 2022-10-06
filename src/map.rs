mod map_builder;
mod themes;

use bracket_geometry::prelude::Point;
use macroquad::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use self::themes::{DungeonTheme, MapTheme};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    width: i32,
    height: i32,

    pub tiles: Vec<TileType>,
    pub theme: Box<dyn MapTheme>,

    pub player_start_pos: Point,
}

impl Map {
    pub async fn new() -> Self {
        let mut map_base = Self::default();

        let mut map = map_base.tiles.clone();

        // Make the boundaries walls
        for x in 0..map_base.width {
            map[map_base.map_idx(x, 0)] = TileType::Wall;
            map[map_base.map_idx(x, map_base.height - 1)] = TileType::Wall;
        }
        for y in 0..map_base.height {
            map[map_base.map_idx(0, y)] = TileType::Wall;
            map[map_base.map_idx(map_base.width - 1, y)] = TileType::Wall;
        }

        // create some random walls
        for _i in 0..400 {
            let x = rand::gen_range(1, map_base.width - 1);
            let y = rand::gen_range(1, map_base.height - 1);
            let idx = map_base.map_idx(x, y);
            if idx != map_base.map_idx(map_base.width / 2, map_base.height / 2) {
                map[idx] = TileType::Wall;
            }
        }

        map_base.tiles = map;

        map_base
    }

    // pub fn new_map_part(self: &Self) {}

    pub fn map_idx(self: &Self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < SCREEN_WIDTH && y >= 0 && y < SCREEN_HEIGHT
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            width: 80,
            height: 50,
            theme: DungeonTheme::new(),

            tiles: vec![TileType::Floor; 80 * 50],
            player_start_pos: Point::new(40, 25),
        }
    }
}
