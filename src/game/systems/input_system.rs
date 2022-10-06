use specs::prelude::*;
use macroquad::prelude::*;

use crate::game::components::{base::Player, events::WantToMove};

#[derive(SystemData)]
pub struct InputData<'a> {
    player: ReadStorage<'a, Player>,
    events: WriteStorage<'a, WantToMove>,
}

pub struct PlayerInput;

impl<'a> System<'a> for PlayerInput {
    type SystemData = InputData<'a>;

    fn run(&mut self, data: Self::SystemData) {

    }
}

