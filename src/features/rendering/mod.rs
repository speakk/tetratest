use itertools::Itertools;
use std::sync::Arc;

use hecs::World;
use tetra::{graphics::DrawParams, math::Vec2, Context};

use super::{game_state::Assets, shared::Position};

pub struct Sprite {
    pub texture: crate::EntityType,
    pub origin: Vec2<f32>,
}

pub fn sprite_draw_system(ctx: &mut Context, world: &mut World, assets: Arc<Assets>) {
    for (_id, (sprite, position)) in world.query::<(&Sprite, &Position)>().into_iter().sorted_by(
        |(_, (_, a_pos)), (_, (_, b_pos))| {
            a_pos
                .0
                .y
                .partial_cmp(&b_pos.0.y)
                .unwrap()
                .then(a_pos.0.x.partial_cmp(&b_pos.0.x).unwrap())
        },
    ) {
        assets
            .textures
            .get(&sprite.texture)
            .expect("No texture found for sprite entity type")
            .draw(
                ctx,
                DrawParams::new().position(position.0).origin(sprite.origin),
            );
    }
    //         self.textures.skelly.draw(
    //             ctx,
    //             DrawParams::new()
    //                 .position(Vec2::new(32., 32.))
    //                 .origin(Vec2::new(16., 16.)),
    //         );
}
