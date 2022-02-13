use std::sync::{Arc, Mutex};

use hecs::World;
use tetra::{
    graphics::{self, scaling::ScreenScaler, Camera, Color},
    Context,
};

use crate::features::{
    map::{
        self, coordinate_to_position_system::coordinate_to_position_system,
        hex_hover_system::hex_hover_system, map_click_handler::map_click_handler, Coordinate,
    },
    rendering::{color_interpolate_system, create_hex_entity, sprite_draw_system},
};

use super::Scene;
use super::Transition;

type SystemType = fn(&mut Context, &mut Resources);

pub struct Resources {
    pub world: World,
    pub camera: Arc<Camera>,
    pub scaler: Arc<Mutex<ScreenScaler>>,
    pub last_hovered_hex: Option<Coordinate>,
}

pub struct InGameScene {
    pub update_systems: Vec<SystemType>,
    pub draw_systems: Vec<SystemType>,
    pub map: Vec<Coordinate>,
    pub resources: Resources,
}

impl InGameScene {
    pub fn new(
        _: &mut Context,
        camera: Arc<Camera>,
        scaler: Arc<Mutex<ScreenScaler>>,
    ) -> InGameScene {
        let mut scene = InGameScene {
            resources: Resources {
                world: World::new(),
                camera: camera.clone(),
                last_hovered_hex: None,
                scaler: scaler.clone(),
            },
            update_systems: vec![],
            draw_systems: vec![],
            map: map::create_grid(8, map::MapShape::Hexagonal),
        };

        scene.draw_systems.push(sprite_draw_system);

        scene.update_systems.push(coordinate_to_position_system);
        scene.update_systems.push(color_interpolate_system);
        scene.update_systems.push(hex_hover_system);

        for hex in scene.map.iter() {
            scene.resources.world.spawn(create_hex_entity(hex.clone()));
        }

        scene
    }
}

impl Scene for InGameScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        for system in self.update_systems.iter() {
            system(ctx, &mut self.resources);
        }
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        graphics::clear(ctx, Color::rgb(0.094, 0.11, 0.16));

        for system in self.draw_systems.iter() {
            system(ctx, &mut self.resources);
        }

        Ok(Transition::None)
    }

    fn mouse_button_pressed(
        &mut self,
        ctx: &mut Context,
        mouse_button: tetra::input::MouseButton,
        camera: &Camera,
    ) -> () {
        map_click_handler(ctx, &mut self.resources.world, mouse_button, camera);
    }
}
