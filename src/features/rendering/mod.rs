use bevy_ecs::{prelude::Component, system::Query};
use tetra::{
    graphics::Color,
    math::{Lerp, Vec2, Vec4},
};

use crate::EntityType;

#[derive(Debug)]
pub struct Origin {
    pub relative: Vec2<f32>,
    //pub pixels: Vec2<f32>,
    pub cached_pixel_origin: Option<Vec2<f32>>,
}

#[derive(Component)]
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
        //ASSET_MANAGER.with(|asset_manager| {
        //let asset_manager = asset_manager.borrow();
        //let textures = &asset_manager.borrow().unwrap().textures;
        //let textures = &asset_manager.as_ref().unwrap().textures;

        // let texture = textures
        //     .get(&entity_type)
        //     .expect("No texture found for sprite");

        let origin_relative = origin_relative.unwrap_or_else(|| Vec2::new(0.5, 0.5));

        Sprite {
            entity_type,
            origin: Origin {
                relative: origin_relative,
                cached_pixel_origin: None,
                // pixels: Vec2::new(
                //     texture.width() as f32 * origin_relative.x,
                //     texture.height() as f32 * origin_relative.y,
                // ),
            },
            color: color.unwrap_or(Color::WHITE),
            original_color: color.unwrap_or(Color::WHITE),
        }
        //})
    }
}

// pub fn sprite_draw(
//     query: Query<(&Sprite, &Position, Option<&Selected>)>,
//     textures: NonSend<Textures>,
// ) {
//     for (sprite, position, selected) in
//         query
//             .iter()
//             .into_iter()
//             .sorted_by(|(_, a_pos, _), (_, b_pos, _)| {
//                 a_pos
//                     .0
//                     .y
//                     .partial_cmp(&b_pos.0.y)
//                     .unwrap()
//                     .then(a_pos.0.x.partial_cmp(&b_pos.0.x).unwrap())
//             })
//     {
//         if selected.is_some() {
//             graphics::set_shader(ctx, &asset_manager.as_ref().unwrap().shaders.outline);
//         }
//         .textures
//         .get(&sprite.entity_type)
//         .expect("No texture found for sprite entity type")
//         .draw(
//             ctx,
//             DrawParams::new()
//                 .position(position.0)
//                 .origin(sprite.origin.pixels)
//                 .color(sprite.color),
//         );
//         if selected.is_some() {
//             graphics::reset_shader(ctx);
//         }
//     }
// }

pub fn color_interpolate_system(mut query: Query<&mut Sprite>) {
    for mut sprite in query.iter_mut() {
        //sprite.color = sprite.color * (sprite.original_color * Color::rgb(0.6, 0.6, 0.6) + 0.4);
        let vec1: Vec4<f32> = Vec4::from(sprite.color);
        let vec2: Vec4<f32> = Vec4::from(sprite.original_color);
        sprite.color = Color::from(Lerp::lerp(vec1, vec2, 0.05));
    }
}
