use specs::Component;

use self::base::{Position, Renderable};

pub mod base;
pub mod spawn;


#[derive(Debug)]
pub enum SpawnComponent {
    PositionComponent(Position),
    RenderableComponent(Renderable)
}
