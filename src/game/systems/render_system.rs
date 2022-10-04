use macroquad::prelude::WHITE;
use specs::prelude::*;

use crate::{
    game::{
        components::base::{Position, Renderable},
        Camera,
    },
    map::Map,
    resources::Resources,
};

#[derive(SystemData)]
pub struct MapRenderData<'a> {
    map: ReadExpect<'a, Map>,
    cam: ReadExpect<'a, Camera>,
    res: ReadExpect<'a, Resources>,
}

pub struct RenderMap;

impl<'a> System<'a> for RenderMap {
    type SystemData = MapRenderData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        for y in data.cam.top_y..=data.cam.bottom_y {
            for x in data.cam.left_x..data.cam.right_x {
                let idx = data.map.map_idx(x, y);

                if data.map.in_bounds(x, y) {
                    let tint = WHITE;
                    let sprite = data.map.theme.tile_to_render(data.map.tiles[idx], idx);
                    data.res.tileset().draw_tile(
                        sprite,
                        tint,
                        x - data.cam.left_x,
                        y - data.cam.top_y,
                    );
                }
            }
        }
    }
}

#[derive(SystemData)]
pub struct EntitiesRenderData<'a> {
    cam: ReadExpect<'a, Camera>,
    res: ReadExpect<'a, Resources>,
    positions: ReadStorage<'a, Position>,
    renderings: ReadStorage<'a, Renderable>,
}

pub struct RenderEntities;

impl<'a> System<'a> for RenderEntities {
    type SystemData = EntitiesRenderData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        for (pos, render) in (&data.positions, &data.renderings).join() {
            let pos_x = pos.x - data.cam.left_x;
            let pos_y = pos.y - data.cam.top_y;
            data.res
                .tileset()
                .draw_tile(render.sprite, render.color, pos_x, pos_y);
        }
    }
}
