mod components;
mod entity_spawn_system;
mod systems;

use crate::{
    map::Map, resources::Resources, DISPLAY_HEIGHT, DISPLAY_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use self::systems::render_system;
use hecs::World;

pub struct GameWorld {
    pub ecs: World,
    pub camera: Camera,

    pub map: Map,
    pub resources: Resources,
}

impl GameWorld {
    pub async fn new() -> Self {
        let mut ecs = World::new();
        let camera = Camera::new();
        let resources = Resources::new().await.unwrap();

        let map = Map::new().await;

        entity_spawn_system::spawn_player(&mut ecs, map.player_start_pos);

        GameWorld {
            ecs,
            camera,
            map,
            resources,
        }
    }

    pub async fn tick(self: &mut Self) {
        render_system::render_map(&self);
        render_system::render_entities(&self);
    }
}

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            left_x: SCREEN_WIDTH / 2 - DISPLAY_WIDTH / 2,
            right_x: SCREEN_WIDTH / 2 + DISPLAY_WIDTH / 2,
            top_y: SCREEN_HEIGHT / 2 - DISPLAY_HEIGHT / 2,
            bottom_y: SCREEN_HEIGHT / 2 + DISPLAY_HEIGHT / 2,
        }
    }

    #[allow(dead_code)]
    pub fn set_position(&mut self, pos_x: i32, pos_y: i32) {
        self.left_x = pos_x - DISPLAY_WIDTH / 2;
        self.right_x = pos_x + DISPLAY_WIDTH / 2;
        self.top_y = pos_y - DISPLAY_HEIGHT / 2;
        self.bottom_y = pos_y + DISPLAY_HEIGHT / 2;
    }
}
