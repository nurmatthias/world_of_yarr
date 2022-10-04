use macroquad::{prelude::collections::storage, ui::root_ui};

use super::{in_progress_gui, GuiResources};

pub async fn base_hud() {
    let resources = storage::get::<GuiResources>();
    in_progress_gui(&mut root_ui(), "playing", &resources.default_skin);
}
