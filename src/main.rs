mod game;
mod gui;
mod map;
mod resources;
mod tileset;
mod utils;

use game::GameWorld;
use macroquad::prelude::*;
use map::Map;
use resources::Resources;

use crate::tileset::Sprite;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref RANDOM_FLOOR_TILES: Vec<Sprite> = gen_random_tiles(0, 16);
    pub static ref RANDOM_TREE_TILES: Vec<Sprite> = gen_random_tiles(16, 20);
}

fn gen_random_tiles(lower: usize, upper: usize) -> Vec<Sprite> {
    (0..=(SCREEN_WIDTH * SCREEN_HEIGHT))
        .map(|_| rand::gen_range(lower, upper) as Sprite)
        .collect()
}

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 3;
pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 3;

pub enum GameState {
    SplashScreen,
    MainMenu,
    LoadGame,
    CreateGame,
    InGame,
    Quit,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "World of YARR".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let gui_resources = gui::GuiResources::new();
    collections::storage::store(gui_resources);

    rand::srand(4711 as _);

    let mut game = GameWorld::new();

    let mut next_scene = GameState::SplashScreen;
    loop {
        match next_scene {
            GameState::SplashScreen => {
                next_scene = gui::splash().await;
            }
            GameState::MainMenu => {
                next_scene = gui::main_menu().await;
            }
            GameState::LoadGame => {
                next_scene = gui::loading().await;
            }
            GameState::CreateGame => {
                let resources = Resources::new().await.unwrap();
                game.ecs.insert(resources);

                let map = Map::new().await;
                game.ecs.insert(map);

                game.tick_startup_systems().await;

                next_scene = gui::game_loading().await;
            }
            GameState::InGame => {
                clear_background(BLACK);

                game.tick().await;

                gui::base_hud().await;
                if is_key_down(KeyCode::Escape) {
                    next_scene = GameState::MainMenu;
                }

                next_frame().await;
            }
            _ => break,
        }
    }
}
