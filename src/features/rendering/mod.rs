use itertools::Itertools;

use hecs::World;
use tetra::{
    graphics::{Color, DrawParams},
    math::Vec2,
    Context,
};

use crate::{EntityType, ASSET_MANAGER};

use super::{map::Coordinate, shared::Position};

pub fn create_hex_entity(coordinate: Coordinate) -> (Sprite, Position, Coordinate) {
    (
        Sprite::new(EntityType::Hex, None, None),
        Position(Vec2::new(0., 0.)),
        coordinate,
    )
}

pub struct Sprite {
    pub entity_type: crate::EntityType,
    pub origin: Vec2<f32>,
    pub color: Color,
}

impl Sprite {
    pub fn new(entity_type: EntityType, origin: Option<Vec2<f32>>, color: Option<Color>) -> Self {
        ASSET_MANAGER.with(|asset_manager| {
            let textures = &asset_manager.borrow().textures;

            let texture = textures
                .get(&entity_type)
                .expect("No texture found for sprite");
            Sprite {
                entity_type,
                origin: origin.unwrap_or(Vec2::new(
                    texture.width() as f32 / 2.,
                    texture.height() as f32 / 2.,
                )),
                color: color.unwrap_or(Color::WHITE),
            }
        })
    }
}

pub fn sprite_draw_system(ctx: &mut Context, world: &mut World) {
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
        ASSET_MANAGER.with(|asset_manager| {
            asset_manager
                .borrow()
                .textures
                .get(&sprite.entity_type)
                .expect("No texture found for sprite entity type")
                .draw(
                    ctx,
                    DrawParams::new().position(position.0).origin(sprite.origin),
                );
        });
    }
}
