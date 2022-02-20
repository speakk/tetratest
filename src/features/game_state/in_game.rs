use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bevy_ecs::{
    prelude::World,
    schedule::{Schedule, SystemStage},
};
use tetra::{
    graphics::{self, scaling::ScreenScaler, Camera, Color, Shader},
    Context,
};

use crate::{
    features::{map, rendering::color_interpolate_system},
    Textures,
};

use super::Scene;
use super::Transition;

pub struct Shaders {
    pub outline: Shader,
}

pub struct InGameScene {
    // pub update_systems: Vec<SystemType>,
    // pub draw_systems: Vec<SystemType>,
    //pub map: Vec<Coordinate>,
    //pub resources: Resources,
    pub world: World,
    // pub logic_schedule: Schedule,
    // pub draw_schedule: Schedule,
    pub schedule: Schedule,
}

impl InGameScene {
    pub fn new(
        ctx: &mut Context,
        camera: Arc<Camera>,
        scaler: Arc<Mutex<ScreenScaler>>,
    ) -> InGameScene {
        let outline_shader = Shader::from_fragment_file(ctx, "assets/shaders/outline.frag")
            .expect("Could not load outline shader");

        let mut schedule = Schedule::default();
        let mut world = World::new();

        // let mut scene = InGameScene {
        //     world: World::new(),
        //     schedule: Schedule {
        //         ..Default::default()
        //     },
        //     // resources: Resources {
        //     //     camera,
        //     //     last_hovered_hex: None,
        //     //     scaler,
        //     //     nine_slice: NineSlice::with_border(Rectangle::new(0., 0., 32., 32.), 4.),
        //     // },
        //     // update_systems: vec![],
        //     // draw_systems: vec![],
        //     //map: map::create_grid(8, map::MapShape::Hexagonal),
        // };

        //let map = map::create_grid(8, map::MapShape::Hexagonal);

        const DRAW_STAGE: &str = "draw";
        const UPDATE_STAGE: &str = "update";

        //scene.draw_systems.push(sprite_draw_system);
        schedule.add_stage(DRAW_STAGE, SystemStage::single_threaded());
        schedule.add_stage(UPDATE_STAGE, SystemStage::single_threaded());
        //.add_system_to_stage(DRAW_STAGE, sprite_draw_system)
        schedule.add_system_to_stage(UPDATE_STAGE, color_interpolate_system);

        //scene.world.resource_scope
        world.insert_non_send(HashMap::new() as Textures);
        world.insert_non_send(Shaders {
            outline: outline_shader,
        });

        // let mut hmm = scene
        //     .draw_schedule
        //     .get_stage_mut::<SystemStage>(&"draw")
        //     .unwrap();

        // hmm.run(&mut scene.world);

        // scene.update_systems.push(coordinate_to_position_system);
        // scene.update_systems.push(color_interpolate_system);
        // scene.update_systems.push(hex_hover_system);

        // for hex in scene.map.iter() {
        //     scene.resources.world.spawn(create_hex_entity(*hex));
        // }

        // let unit = scene.resources.world.spawn(create_unit_entity());
        // scene
        //     .world
        //     //.insert_one(unit, Coordinate { q: 1, r: 3 })
        //     .unwrap();

        InGameScene { world, schedule }
    }
}

impl Scene for InGameScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        // for system in self.update_systems.iter() {
        //     system(ctx, &mut self.resources);
        // }
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        graphics::clear(ctx, Color::rgb(0.094, 0.11, 0.16));

        // for system in self.draw_systems.iter() {
        //     system(ctx, &mut self.resources);
        // }

        Ok(Transition::None)
    }

    fn mouse_button_pressed(&mut self, ctx: &mut Context, mouse_button: tetra::input::MouseButton) {
        //map_click_handler(ctx, mouse_button, &mut self.resources);
    }
}
