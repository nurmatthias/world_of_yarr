use macroquad::prelude::WHITE;

use crate::game::{
    components::base::{Position, Renderable},
    GameWorld,
};

pub fn render_map(world: &GameWorld) {
    let map = &world.map;
    let cam = &world.camera;
    let res = &world.resources;

    for y in cam.top_y..=cam.bottom_y {
        for x in cam.left_x..cam.right_x {
            let idx = map.map_idx(x, y);

            if map.in_bounds(x, y) {
                let tint = WHITE;
                let sprite = map.theme.tile_to_render(map.tiles[idx], idx);
                res.tileset()
                    .draw_tile(sprite, tint, x - cam.left_x, y - cam.top_y);
            }
        }
    }
}

pub fn render_entities(world: &GameWorld) {
    let res = &world.resources;

    for (_id, (pos, render)) in &mut world.ecs.query::<(&Position, &Renderable)>() {
        let pos_x = pos.x - world.camera.left_x;
        let pos_y = pos.y - world.camera.top_y;
        res.tileset()
            .draw_tile(render.sprite, render.color, pos_x, pos_y);
    }
}
