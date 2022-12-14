use macroquad::{
    prelude::{collections::storage, BLACK},
    ui::root_ui,
    window::{clear_background, next_frame},
};

use crate::GameState;

use super::{in_progress_gui, GuiResources};

pub async fn game_loading() -> GameState {
    clear_background(BLACK);

    let resources = storage::get::<GuiResources>();
    in_progress_gui(&mut root_ui(), "creating map", &resources.default_skin);

    next_frame().await;

    return GameState::InGame;
}
