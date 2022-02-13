use std::collections::HashMap;
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::{self, Camera, Color, Texture};
use tetra::input::MouseButton;
//use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, Event, State};

use crate::{ASSET_MANAGER, HEIGHT, WIDTH};

mod in_game;

trait Scene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
    fn mouse_button_pressed(
        &mut self,
        _ctx: &mut Context,
        _mouse_button: MouseButton,
        _camera: &Camera,
    ) -> () {
    }
}

#[allow(dead_code)]
enum Transition {
    None,
    Push(Box<dyn Scene>),
    Pop,
}

pub struct GameState {
    scenes: Vec<Box<dyn Scene>>,
    scaler: ScreenScaler,
    camera: Camera,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        ASSET_MANAGER.with(|asset_manager| {
            asset_manager.borrow_mut().textures = HashMap::from([
                (
                    crate::EntityType::Skelly,
                    Texture::new(ctx, "./assets/sprites/skelly.png").unwrap(),
                ),
                (
                    crate::EntityType::Hex,
                    Texture::new(ctx, "./assets/sprites/hexagon.png").unwrap(),
                ),
            ]);
        });

        let initial_scene = in_game::InGameScene::new(ctx);

        Ok(GameState {
            scenes: vec![Box::new(initial_scene)],
            scaler: ScreenScaler::with_window_size(
                ctx,
                crate::WIDTH,
                crate::HEIGHT,
                ScalingMode::ShowAllPixelPerfect,
            )?,
            camera: Camera::new(WIDTH as f32, HEIGHT as f32),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.update(ctx)? {
                Transition::None => {}
                Transition::Push(s) => {
                    self.scenes.push(s);
                }
                Transition::Pop => {
                    self.scenes.pop();
                }
            },
            None => window::quit(ctx),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::BLACK);

        graphics::set_transform_matrix(ctx, self.camera.as_matrix());

        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.draw(ctx)? {
                Transition::None => {}
                Transition::Push(s) => {
                    self.scenes.push(s);
                }
                Transition::Pop => {
                    self.scenes.pop();
                }
            },
            None => window::quit(ctx),
        }

        graphics::reset_transform_matrix(ctx);

        graphics::reset_canvas(ctx);

        graphics::clear(ctx, Color::BLACK);

        self.scaler.draw(ctx);

        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        if let Event::Resized { width, height } = event {
            self.scaler.set_outer_size(width, height);
        }

        if let Event::MouseButtonPressed { button } = event {
            if let Some(active_scene) = self.scenes.last_mut() {
                active_scene.mouse_button_pressed(ctx, button, &self.camera);
            }
        }

        Ok(())
    }
}
