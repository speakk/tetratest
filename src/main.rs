use std::{cell::RefCell, collections::HashMap};

use features::game_state::GameState;
use tetra::{graphics::Texture, ContextBuilder};

#[macro_use]

mod features;

pub const WIDTH: i32 = 640;
pub const HEIGHT: i32 = 480;

pub struct AssetManager {
    pub textures: HashMap<EntityType, Texture>,
}

thread_local!(pub static ASSET_MANAGER: RefCell<AssetManager> = RefCell::new(AssetManager {
    textures: HashMap::new()
}));

#[derive(Hash, Eq, PartialEq)]
pub enum EntityType {
    Skelly,
    Hex,
}

fn main() -> tetra::Result {
    ContextBuilder::new("Hello world", 900, 900)
        .quit_on_escape(true)
        .show_mouse(true)
        .resizable(true)
        .build()?
        .run(GameState::new)
}
