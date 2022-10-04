use macroquad::prelude::Color;
use specs::prelude::*;
use specs_derive::*;

use crate::tileset::Sprite;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Renderable {
    pub color: Color,
    pub sprite: Sprite,
}

#[derive(Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
