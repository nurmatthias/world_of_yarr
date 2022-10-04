use macroquad::{
    experimental::collections::storage,
    math::vec2,
    prelude::BLACK,
    ui::{root_ui, widgets},
    window::{clear_background, next_frame, screen_height, screen_width},
};

use crate::GameState;

use super::GuiResources;

pub async fn main_menu() -> GameState {
    loop {
        clear_background(BLACK);

        let resources = storage::get::<GuiResources>();
        root_ui().push_skin(&resources.title_skin);

        let title = "World of YARR";
        let label_size = root_ui().calc_size(title);
        let label_pos = vec2(screen_width() / 2. - label_size.x / 2., 100.);
        root_ui().label(Some(label_pos), title);

        let button_width = 300.0;
        let menu_start = screen_height() - 360.;

        if widgets::Button::new("New game")
            .size(vec2(button_width, 100.))
            .position(vec2(screen_width() / 2. - button_width / 2., menu_start))
            .ui(&mut *root_ui())
        {
            root_ui().pop_skin();
            return GameState::CreateGame;
        }

        if widgets::Button::new("Load")
            .size(vec2(button_width, 100.))
            .position(vec2(
                screen_width() / 2. - button_width / 2.,
                menu_start + 110.,
            ))
            .ui(&mut *root_ui())
        {
            root_ui().pop_skin();
            return GameState::LoadGame;
        }

        if widgets::Button::new("Exit")
            .size(vec2(button_width, 100.))
            .position(vec2(
                screen_width() / 2. - button_width / 2.,
                menu_start + 220.,
            ))
            .ui(&mut *root_ui())
        {
            root_ui().pop_skin();
            return GameState::Quit;
        }

        root_ui().pop_skin();

        next_frame().await;
    }
}
