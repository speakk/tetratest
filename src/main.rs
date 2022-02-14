use std::{cell::RefCell, collections::HashMap, rc::Rc};

use features::game_state::GameState;
use tetra::{
    graphics::{Shader, Texture},
    ContextBuilder,
};

#[macro_use]

mod features;

pub const WIDTH: i32 = 640;
pub const HEIGHT: i32 = 480;

pub struct Shaders {
    pub outline: Shader,
}

pub struct AssetManager {
    pub textures: HashMap<EntityType, Texture>,
    pub shaders: Shaders,
}

thread_local!(pub static ASSET_MANAGER: Rc<RefCell<Option<AssetManager>>> = Rc::new(RefCell::new(None)));
/*AssetManager {

    textures: HashMap::new(),
    shaders: Shaders {
        outline: Shader::default().unwrap()
    }
}));
*/

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
