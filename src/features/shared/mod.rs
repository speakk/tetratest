use bevy_ecs::prelude::Component;
use tetra::math::Vec2;

#[derive(Default, Component)]
pub struct Position(pub Vec2<f32>);
