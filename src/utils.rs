#![allow(dead_code)]

use bracket_pathfinding::prelude::Point;
use macroquad::prelude::*;

use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH, SCREEN_WIDTH};

pub fn tile_size() -> Vec2 {
    vec2(tile_width(), tile_height())
}

pub fn tile_width() -> f32 {
    screen_width() / DISPLAY_WIDTH as f32
}

pub fn tile_height() -> f32 {
    screen_height() / DISPLAY_HEIGHT as f32
}

pub fn tile_pos_x(x: i32) -> f32 {
    x as f32 * tile_width()
}

pub fn tile_pos_y(y: i32) -> f32 {
    y as f32 * tile_height()
}

pub fn text_pos_x(x: i32) -> f32 {
    x as f32 * screen_width() / SCREEN_WIDTH as f32
}

pub fn mouse_tile_position() -> (i32, i32) {
    let pos = mouse_position();
    (
        (pos.0 / tile_width()) as i32,
        (pos.1 / tile_height()) as i32,
    )
}

pub fn print_centered<S>(line: i32, text: S)
where
    S: ToString,
{
    print_color_centered(line, text, WHITE);
}

pub fn print_color_centered<S>(line: i32, text: S, text_color: Color)
where
    S: ToString,
{
    let x = SCREEN_WIDTH / 2 - (text.to_string().len() / 2) as i32;
    print_color_pos(Point::new(x, line), text, text_color);
}

pub fn print_color_right<S>(pos: Point, text: S, text_color: Color)
where
    S: ToString,
{
    let offset = Point::new(text.to_string().len() as i32, 0);
    print_color_pos(pos - offset, text, text_color);
}

pub fn print_pos<S>(pos: Point, text: S)
where
    S: ToString,
{
    print_color_pos(pos, text, WHITE);
}

pub fn print_color_pos<S>(pos: Point, text: S, text_color: Color)
where
    S: ToString,
{
    let text_params = TextParams {
        color: text_color,
        font_size: tile_height() as u16,
        ..TextParams::default()
    };
    let dimensions = measure_text(
        &text.to_string(),
        Some(Font::default()),
        text_params.font_size,
        text_params.font_scale,
    );
    let x = text_pos_x(pos.x);
    let fudge = (dimensions.height - dimensions.offset_y) / 2.;
    let y = tile_pos_y(pos.y) + fudge + dimensions.offset_y;
    draw_text_ex(&text.to_string(), x, y, text_params);
}

pub fn bar_horizontal(
    pos: Point,
    width: i32,
    current: i32,
    max: i32,
    color: Color,
    background: Color,
) {
    let x = tile_pos_x(pos.x);
    let y = tile_pos_y(pos.y);
    let bar_width = tile_pos_x(width);
    let current_width = current as f32 / max as f32 * bar_width;
    draw_rectangle(x, y, bar_width, tile_height(), background);
    draw_rectangle(x, y, current_width, tile_height(), color);
}

pub fn random_slice_index<T>(slice: &[T]) -> Option<usize> {
    if slice.is_empty() {
        None
    } else {
        let size = slice.len();
        if size == 1 {
            Some(0)
        } else {
            Some(rand::gen_range(1, size as i32) as usize - 1)
        }
    }
}
