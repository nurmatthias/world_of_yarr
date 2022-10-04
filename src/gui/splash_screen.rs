use macroquad::{
    prelude::{collections::storage, BLACK},
    time::get_time,
    ui::root_ui,
    window::{clear_background, next_frame},
};

use crate::GameState;

use super::{in_progress_gui, GuiResources};

pub async fn splash() -> GameState {
    let start_time = get_time();

    loop {
        clear_background(BLACK);

        let resources = storage::get::<GuiResources>();
        in_progress_gui(&mut root_ui(), "Going in", &resources.default_skin);

        next_frame().await;

        if get_time() - start_time >= 1. {
            return GameState::MainMenu;
        }
    }
}
