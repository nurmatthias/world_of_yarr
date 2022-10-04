use macroquad::{
    prelude::{collections::storage, coroutines::Coroutine, BLACK},
    ui::root_ui,
    window::{clear_background, next_frame},
};

use crate::GameState;

use super::{in_progress_gui, GuiResources};

pub async fn game_loading(map_loading: Coroutine) -> GameState {
    println!("show loading screen");

    while map_loading.is_done() == false {
        clear_background(BLACK);

        let resources = storage::get::<GuiResources>();
        in_progress_gui(&mut root_ui(), "creating map", &resources.default_skin);

        next_frame().await;
    }

    println!("map has finished loading");
    return GameState::InGame;
}
