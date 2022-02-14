use itertools::Itertools;

use tetra::{
    graphics::{Color, DrawParams},
    math::{Lerp, Vec2, Vec4},
    Context,
};

use crate::{EntityType, ASSET_MANAGER};

use super::{game_state::in_game::Resources, map::Coordinate, shared::Position};

pub fn create_hex_entity(coordinate: Coordinate) -> (Sprite, Position, Coordinate) {
    (
        Sprite::new(EntityType::Hex, Some(Vec2::new(0.5, 0.35)), None),
        Position(Vec2::new(0., 0.)),
        coordinate,
    )
}

#[derive(Debug)]
pub struct Origin {
    pub relative: Vec2<f32>,
    pub pixels: Vec2<f32>,
}

pub struct Sprite {
    pub entity_type: crate::EntityType,
    pub origin: Origin,
    pub color: Color,
    pub original_color: Color,
}

impl Sprite {
    pub fn new(
        entity_type: EntityType,
        origin_relative: Option<Vec2<f32>>,
        color: Option<Color>,
    ) -> Self {
        ASSET_MANAGER.with(|asset_manager| {
            let textures = &asset_manager.borrow().textures;

            let texture = textures
                .get(&entity_type)
                .expect("No texture found for sprite");

            let origin_relative = origin_relative.unwrap_or_else(|| Vec2::new(0.5, 0.5));

            Sprite {
                entity_type,
                origin: Origin {
                    relative: origin_relative,
                    pixels: Vec2::new(
                        texture.width() as f32 * origin_relative.x,
                        texture.height() as f32 * origin_relative.y,
                    ),
                },
                color: color.unwrap_or(Color::WHITE),
                original_color: color.unwrap_or(Color::WHITE),
            }
        })
    }
}

pub fn sprite_draw_system(ctx: &mut Context, resources: &mut Resources) {
    let world = &resources.world;
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
                    DrawParams::new()
                        .position(position.0)
                        .origin(sprite.origin.pixels)
                        .color(sprite.color),
                );
        });
    }
}

pub fn color_interpolate_system(_ctx: &mut Context, resources: &mut Resources) {
    let world = &resources.world;
    for (_, mut sprite) in world.query::<&mut Sprite>().iter() {
        //sprite.color = sprite.color * (sprite.original_color * Color::rgb(0.6, 0.6, 0.6) + 0.4);
        let vec1: Vec4<f32> = Vec4::from(sprite.color);
        let vec2: Vec4<f32> = Vec4::from(sprite.original_color);
        sprite.color = Color::from(Lerp::lerp(vec1, vec2, 0.10));
    }
}
