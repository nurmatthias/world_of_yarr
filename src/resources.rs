use macroquad::texture::{load_texture, FilterMode, Texture2D};

use crate::tileset::TileSet;

pub struct Resources {
    pub default_tileset: Texture2D,
}

impl Resources {
    pub async fn new() -> Result<Resources, macroquad::prelude::FileError> {
        let default_tileset = load_texture("assets/textures/tileset.png").await?;
        default_tileset.set_filter(FilterMode::Nearest);

        Ok(Resources { default_tileset })
    }

    pub fn tileset(self: &Self) -> TileSet {
        TileSet {
            texture: self.default_tileset,
            tile_width: 32,
            tile_height: 32,
            columns: 16,
        }
    }
}
