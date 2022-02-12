use models::Coordinate;
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, Event, State};

#[macro_use]

mod models;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

struct Textures {
    skelly: Texture,
    hex: Texture,
}

struct GameState {
    textures: Textures,
    scaler: ScreenScaler,
    grid: Vec<Coordinate>,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut grid = models::create_grid(8, models::MapShape::Hexagonal);
        grid.sort_by(|a, b| b.q.cmp(&a.q));

        Ok(GameState {
            textures: Textures {
                skelly: Texture::new(ctx, "./assets/sprites/skelly.png")?,
                hex: Texture::new(ctx, "./assets/sprites/hexagon.png")?,
            },
            scaler: ScreenScaler::with_window_size(
                ctx,
                WIDTH,
                HEIGHT,
                ScalingMode::ShowAllPixelPerfect,
            )?,
            grid,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::rgb(0.18, 0.13, 0.15));

        self.textures.skelly.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(32., 32.))
                .origin(Vec2::new(16., 16.)),
        );

        for hex in self.grid.iter() {
            self.textures.hex.draw(
                ctx,
                DrawParams::new()
                    .position(
                        Vec2::new(WIDTH as f32 / 2., HEIGHT as f32 / 2.)
                            + models::map::pointy_hex_to_pixel(hex.q, hex.r),
                    )
                    .origin(Vec2::new(16., 16.)),
            );
        }

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

fn main() -> tetra::Result {
    ContextBuilder::new("Hello world", 900, 900)
        .quit_on_escape(true)
        .resizable(true)
        .build()?
        .run(GameState::new)
}
