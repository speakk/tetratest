use hecs::World;
use tetra::{
    graphics::{self, Color},
    Context,
};

use super::Assets;
use super::Scene;
use super::Transition;

pub struct InGameScene {
    world: World,
}

impl InGameScene {
    pub fn new(ctx: &mut Context, assets: &Assets) -> InGameScene {
        InGameScene {
            world: World::new(),
        }
    }
}

impl Scene for InGameScene {
    fn update(&mut self, ctx: &mut Context, assets: &Assets) -> tetra::Result<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> tetra::Result<Transition> {
        graphics::clear(ctx, Color::rgb(0.094, 0.11, 0.16));
        Ok(Transition::None)
    }
}
