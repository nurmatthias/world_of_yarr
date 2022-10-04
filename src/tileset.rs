#![allow(dead_code)]
use macroquad::prelude::*;

use crate::utils::{tile_pos_x, tile_pos_y, tile_size};

pub type Sprite = u16;

#[derive(Debug)]
pub struct TileSet {
    pub texture: Texture2D,
    pub tile_width: i32,
    pub tile_height: i32,
    pub columns: u16,
}

impl TileSet {
    pub const SPRITE_PLAYER: Sprite = 64;
    pub const SPRITE_WALL: Sprite = 35;
    pub const SPRITE_GROUND: Sprite = 59;
    pub const SPRITE_STAIRS: Sprite = 62;
    pub const SPRITE_AMULET: Sprite = 124;

    pub fn sprite_rect(&self, ix: Sprite) -> Rect {
        let sw = self.tile_width as f32;
        let sh = self.tile_height as f32;
        let sx = (ix % self.columns) as f32 * sw as f32;
        let sy = (ix / self.columns) as f32 * sh as f32;

        Rect::new(sx, sy, sw, sh)
    }

    pub fn draw_tile(&self, sprite: Sprite, color: Color, x: i32, y: i32) {
        let spr_rect = self.sprite_rect(sprite);
        draw_texture_ex(
            self.texture,
            tile_pos_x(x),
            tile_pos_y(y),
            color,
            DrawTextureParams {
                dest_size: Some(tile_size()),
                source: Some(spr_rect),
                ..Default::default()
            },
        );
    }
}
