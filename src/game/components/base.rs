use macroquad::prelude::Color;

use crate::tileset::Sprite;

pub struct Player;

pub struct Renderable {
    pub color: Color,
    pub sprite: Sprite,
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}
