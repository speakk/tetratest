use std::sync::Arc;

use hecs::World;
use tetra::{
    graphics::{self, Color},
    math::Vec2,
    Context,
};

use crate::features::{
    map::{self, coordinate_to_position_system::coordinate_to_position_system, Coordinate},
    rendering::{sprite_draw_system, Sprite},
    shared::Position,
};

use super::Assets;
use super::Scene;
use super::Transition;

type SystemType = fn(&mut Context, &mut World, Arc<Assets>);

pub struct InGameScene {
    world: World,
    assets: Arc<Assets>,
    update_systems: Vec<SystemType>,
    draw_systems: Vec<SystemType>,
    map: Vec<Coordinate>,
}

impl InGameScene {
    pub fn new(_: &mut Context, assets: Arc<Assets>) -> InGameScene {
        let mut scene = InGameScene {
            world: World::new(),
            assets,
            update_systems: vec![],
            draw_systems: vec![],
            map: map::create_grid(8, map::MapShape::Hexagonal),
        };

        scene.draw_systems.push(sprite_draw_system);
        scene.update_systems.push(coordinate_to_position_system);

        for hex in scene.map.iter() {
            scene.world.spawn((
                Sprite {
                    texture: crate::EntityType::Hex,
                    origin: Vec2::new(16., 16.),
                },
                Position(Vec2::new(100., 100.)),
                hex.clone() as Coordinate,
            ));
        }

        scene
    }
}

impl Scene for InGameScene {
    fn update(&mut self, ctx: &mut Context, _assets: &Assets) -> tetra::Result<Transition> {
        for system in self.update_systems.iter() {
            system(ctx, &mut self.world, self.assets.clone());
        }
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
