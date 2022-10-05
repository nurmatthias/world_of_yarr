mod components;
mod entity_spawn_system;
mod systems;

use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};

use specs::prelude::*;

use self::{
    systems::render_system::{RenderEntities, RenderMap}, entity_spawn_system::SpawnPlayer,
};

pub struct GameWorld {
    pub ecs: World,
    setup_dispatcher: Dispatcher<'static, 'static>,
    run_dispatcher: Dispatcher<'static, 'static>,
}

impl GameWorld {
    pub fn new() -> Self {
        let mut ecs = World::new();

        let mut setup_dispatcher = Self::create_setup_dispatcher();
        setup_dispatcher.setup(&mut ecs);

        let mut run_dispatcher = Self::create_run_dispatcher();
        run_dispatcher.setup(&mut ecs);

        let camera = Camera::new();
        ecs.insert(camera);

        GameWorld {
            ecs,
            setup_dispatcher,
            run_dispatcher,
        }
    }

    fn create_setup_dispatcher() -> Dispatcher<'static, 'static> {
        DispatcherBuilder::new()
            .with(SpawnPlayer, "SpawnPlayer", &[])
            .build()
    }

    fn create_run_dispatcher() -> Dispatcher<'static, 'static> {
        DispatcherBuilder::new()
            .with_barrier()
            .with(RenderMap, "RenderMap", &[])
            .with(RenderEntities, "RenderEntities", &["RenderMap"])
            .build()
    }

    pub async fn tick_startup_systems(self: &mut Self) {
        self.setup_dispatcher.dispatch(&self.ecs);
        self.ecs.maintain();
    }

    pub async fn tick(self: &mut Self) {
        self.run_dispatcher.dispatch(&self.ecs);
        self.ecs.maintain();
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
