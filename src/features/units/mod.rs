use tetra::math::Vec2;

use crate::EntityType;

use super::{rendering::Sprite, shared::Position};

pub struct Unit;
pub struct Selected;

pub fn create_unit_entity() -> (Sprite, Position, Unit) {
    (
        Sprite::new(EntityType::Skelly, Some(Vec2::new(0.5, 0.9)), None),
        Position::default(),
        Unit,
    )
}
