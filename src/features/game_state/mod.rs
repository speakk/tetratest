use std::collections::HashMap;
use std::sync::Arc;
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::{self, Camera, Color, Texture};
//use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, Event, State};

use crate::{HEIGHT, WIDTH};

mod in_game;

type Textures = HashMap<crate::EntityType, Texture>;

pub struct Assets {
    pub textures: Textures,
}

trait Scene {
    fn update(&mut self, ctx: &mut Context, assets: &Assets) -> tetra::Result<Transition>;
    fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> tetra::Result<Transition>;
}

#[allow(dead_code)]
enum Transition {
    None,
    Push(Box<dyn Scene>),
    Pop,
}

pub struct GameState {
    scenes: Vec<Box<dyn Scene>>,
    assets: Arc<Assets>,
    scaler: ScreenScaler,
    camera: Camera,
}

impl<'a> GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        //let mut grid = map::create_grid(8, map::MapShape::Hexagonal);
        //grid.sort_by(|a, b| b.q.cmp(&a.q));

        let assets = Assets {
            textures: HashMap::from([
                (
                    crate::EntityType::Skelly,
                    Texture::new(ctx, "./assets/sprites/skelly.png")?,
                ),
                (
                    crate::EntityType::Hex,
                    Texture::new(ctx, "./assets/sprites/hexagon.png")?,
                ),
            ]),
        };

        let assets = Arc::new(assets);

        let initial_scene = in_game::InGameScene::new(ctx, assets.clone());

        Ok(GameState {
            scenes: vec![Box::new(initial_scene)],
            scaler: ScreenScaler::with_window_size(
                ctx,
                crate::WIDTH,
                crate::HEIGHT,
                ScalingMode::ShowAllPixelPerfect,
            )?,
            camera: Camera::new(WIDTH as f32, HEIGHT as f32),
            assets,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.update(ctx, &self.assets)? {
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
            Some(active_scene) => match active_scene.draw(ctx, &self.assets)? {
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

    fn event(&mut self, _: &mut Context, event: Event) -> tetra::Result {
        if let Event::Resized { width, height } = event {
            self.scaler.set_outer_size(width, height);
        }

        Ok(())
    }
}
