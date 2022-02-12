use features::game_state::GameState;
use tetra::ContextBuilder;

#[macro_use]

mod features;

pub const WIDTH: i32 = 640;
pub const HEIGHT: i32 = 480;

#[derive(Hash, Eq, PartialEq)]
pub enum EntityType {
    Skelly,
    Hex,
}

fn main() -> tetra::Result {
    ContextBuilder::new("Hello world", 900, 900)
        .quit_on_escape(true)
        .resizable(true)
        .build()?
        .run(GameState::new)
}
