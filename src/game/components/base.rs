use bracket_geometry::prelude::Point;
use macroquad::prelude::Color;
use specs::prelude::*;
use specs_derive::*;

use crate::tileset::Sprite;

#[derive(Component, Debug, Clone, Copy)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy)]
pub struct Renderable {
    pub color: Color,
    pub sprite: Sprite,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub Point);
