use bracket_pathfinding::prelude::Point;
use macroquad::{
    prelude::{BLUE, WHITE},
    text::draw_text,
};

use crate::game::GameWorld;

pub fn render_map(world: &GameWorld) {
    let map = &world.map;
    let res = &world.resources;

    for y in world.camera.top_y..=world.camera.bottom_y {
        for x in world.camera.left_x..world.camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(world.camera.left_x, world.camera.top_y);
            let idx = map.map_idx(x, y);

            if map.in_bounds(pt.x, pt.y) {
                let tint = WHITE;
                let sprite = map.theme.tile_to_render(map.tiles[idx], idx);
                let pos = pt - offset;
                res.tileset().draw_tile(sprite, tint, pos.x, pos.y);
            }
        }
    }
}

pub fn render_entities(_world: &GameWorld) {
    println!("render entities");
    draw_text("render the entities", 1., 40., 30., BLUE);
}
