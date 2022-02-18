use std::sync::{Arc, Mutex};

use hecs::World;
use tetra::{
    graphics::{self, scaling::ScreenScaler, Camera, Color, NineSlice, Rectangle, Shader},
    Context,
};

use crate::features::{
    map::{
        self, coordinate_to_position_system::coordinate_to_position_system, create_hex_entity,
        hex_hover_system::hex_hover_system, map_click_handler::map_click_handler, Coordinate,
    },
    rendering::{color_interpolate_system, sprite_draw_system},
    units::create_unit_entity,
};

use super::{Resources, Scene};
use super::{SystemType, Transition};

pub struct Shaders {
    pub outline: Shader,
}

pub struct InGameScene {
    pub update_systems: Vec<SystemType>,
    pub draw_systems: Vec<SystemType>,
    pub map: Vec<Coordinate>,
    pub resources: Resources,
}

impl InGameScene {
    pub fn new(
        ctx: &mut Context,
        camera: Arc<Camera>,
        scaler: Arc<Mutex<ScreenScaler>>,
    ) -> InGameScene {
        let outline_shader = Shader::from_fragment_file(ctx, "assets/shaders/outline.frag")
            .expect("Could not load outline shader");

        let mut scene = InGameScene {
            resources: Resources {
                world: World::new(),
                camera,
                last_hovered_hex: None,
                scaler,
                nine_slice: NineSlice::with_border(Rectangle::new(0., 0., 32., 32.), 4.),
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
            scene.resources.world.spawn(create_hex_entity(*hex));
        }

        let unit = scene.resources.world.spawn(create_unit_entity());
        scene
            .resources
            .world
            .insert_one(unit, Coordinate { q: 1, r: 3 })
            .unwrap();

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

    fn mouse_button_pressed(&mut self, ctx: &mut Context, mouse_button: tetra::input::MouseButton) {
        map_click_handler(ctx, mouse_button, &mut self.resources);
    }
}
