use itertools::Itertools;
use std::sync::Arc;

use hecs::World;
use tetra::{graphics::DrawParams, math::Vec2, Context};

use super::{
    game_state::Assets,
    map::{self, Coordinate},
    shared::Position,
};

pub struct Sprite {
    pub texture: crate::EntityType,
    pub origin: Vec2<f32>,
}

pub fn sprite_draw_system(ctx: &mut Context, world: &mut World, assets: Arc<Assets>) {
    for (_id, (sprite, position, coordinate)) in world
        .query::<(&Sprite, &Position, Option<&Coordinate>)>()
        .into_iter()
        // TODO: Err lol yeah. Update Position based on Coordinate, then here just sort by
        // position.y
        .sorted_by(|(_, (_, _, coord_a)), (_, (_, _, coord_b))| {
            coord_b.unwrap().q.cmp(&coord_a.unwrap().q)
        })
    {
        let position = {
            if let Some(coordinate) = coordinate {
                map::pointy_hex_to_pixel(coordinate.clone())
            } else {
                position.0
            }
        };

        assets
            .textures
            .get(&sprite.texture)
            .expect("No texture found for sprite entity type")
            .draw(
                ctx,
                DrawParams::new().position(position).origin(sprite.origin),
            );
    }
    //         self.textures.skelly.draw(
    //             ctx,
    //             DrawParams::new()
    //                 .position(Vec2::new(32., 32.))
    //                 .origin(Vec2::new(16., 16.)),
    //         );
}
