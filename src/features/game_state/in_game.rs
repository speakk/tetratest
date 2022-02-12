use std::sync::Arc;

use hecs::World;
use tetra::{
    graphics::{self, Color},
    math::Vec2,
    Context,
};

use crate::features::{
    rendering::{sprite_draw_system, Sprite},
    shared::Position,
};

use super::Assets;
use super::Scene;
use super::Transition;

type SystemType = fn(&mut Context, &mut World, Arc<Assets>);
//type SystemType = Box<dyn Fn() -> ()>;
//type SystemType = Fn() -> ();

pub struct InGameScene {
    world: World,
    assets: Arc<Assets>,
    draw_systems: Vec<SystemType>,
}

impl InGameScene {
    pub fn new(_: &mut Context, assets: Arc<Assets>) -> InGameScene {
        let mut scene = InGameScene {
            world: World::new(),
            assets,
            draw_systems: vec![],
        };

        scene.add_system(sprite_draw_system);

        scene.world.spawn((
            Sprite {
                texture: crate::EntityType::Hex,
                origin: Vec2::new(16., 16.),
            },
            Position(Vec2::new(100., 100.)),
        ));

        scene
    }

    pub fn add_system(&mut self, system: SystemType) {
        self.draw_systems.push(system);
    }
}

impl Scene for InGameScene {
    fn update(&mut self, _ctx: &mut Context, _assets: &Assets) -> tetra::Result<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, _assets: &Assets) -> tetra::Result<Transition> {
        graphics::clear(ctx, Color::rgb(0.094, 0.11, 0.16));

        for system in self.draw_systems.iter() {
            system(ctx, &mut self.world, self.assets.clone());
        }

        Ok(Transition::None)
    }
}
