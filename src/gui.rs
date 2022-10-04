use macroquad::{
    math::vec2,
    time::get_time,
    ui::{Skin, Ui},
    window::{screen_height, screen_width},
};

mod hud;
mod loading_screen;
mod main_menu;
mod map_loading_screen;
mod splash_screen;
mod style;

pub use hud::*;
pub use loading_screen::*;
pub use main_menu::*;
pub use map_loading_screen::*;
pub use splash_screen::*;
pub use style::*;

pub fn in_progress_gui(ui: &mut Ui, label: &str, skin: &Skin) {
    let dots_amount = (get_time() as i32) % 4;

    ui.push_skin(skin);

    let label_size = ui.calc_size(&format!("{}{}", label, ".".repeat(3 as usize)));

    ui.label(
        Some(vec2(
            screen_width() - label_size.x - 10.,
            screen_height() - label_size.y - 10.,
        )),
        &format!("{}{}", label, ".".repeat(dots_amount as usize)),
    );
    ui.pop_skin();
}
