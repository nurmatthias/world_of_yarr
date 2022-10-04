use crate::{
    tileset::{Sprite, TileSet},
    RANDOM_FLOOR_TILES, RANDOM_TREE_TILES,
};

use super::TileType;

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType, idx: usize) -> Sprite;
}

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType, idx: usize) -> Sprite {
        match tile_type {
            TileType::Floor => RANDOM_FLOOR_TILES[idx],
            TileType::Wall => TileSet::SPRITE_WALL,
        }
    }
}

pub struct ForestTheme {}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType, idx: usize) -> Sprite {
        match tile_type {
            TileType::Floor => TileSet::SPRITE_GROUND,
            TileType::Wall => RANDOM_TREE_TILES[idx],
        }
    }
}

impl ForestTheme {
    #[allow(dead_code)]
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}
