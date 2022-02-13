use hecs::World;
use tetra::{
    graphics::{self, Camera, Color},
    Context,
};

use crate::features::{
    map::{
        self, coordinate_to_position_system::coordinate_to_position_system,
        map_click_handler::map_click_handler, Coordinate,
    },
    rendering::{create_hex_entity, sprite_draw_system},
};

use super::Scene;
use super::Transition;

type SystemType = fn(&mut Context, &mut World);

pub struct InGameScene {
    world: World,
    update_systems: Vec<SystemType>,
    draw_systems: Vec<SystemType>,
    map: Vec<Coordinate>,
}

impl InGameScene {
    pub fn new(_: &mut Context) -> InGameScene {
        let mut scene = InGameScene {
            world: World::new(),
            update_systems: vec![],
            draw_systems: vec![],
            map: map::create_grid(8, map::MapShape::Hexagonal),
        };

        scene.draw_systems.push(sprite_draw_system);
        scene.update_systems.push(coordinate_to_position_system);

        for hex in scene.map.iter() {
            // scene.world.spawn((
            //     Sprite {
            //         entity_type: crate::EntityType::Hex,
            //         origin: Vec2::new(16., 16.),
            //         color: Color::WHITE,
            //     },
            //     Position(Vec2::new(100., 100.)),
            //     hex.clone() as Coordinate,
            // ));

            scene.world.spawn(create_hex_entity(hex.clone()));
        }

        scene
    }
}

impl Scene for InGameScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        for system in self.update_systems.iter() {
            system(ctx, &mut self.world);
        }
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        graphics::clear(ctx, Color::rgb(0.094, 0.11, 0.16));

        for system in self.draw_systems.iter() {
            system(ctx, &mut self.world);
        }

        Ok(Transition::None)
    }

    fn mouse_button_pressed(
        &mut self,
        ctx: &mut Context,
        mouse_button: tetra::input::MouseButton,
        camera: &Camera,
    ) -> () {
        map_click_handler(ctx, &mut self.world, mouse_button, camera);
    }
}
