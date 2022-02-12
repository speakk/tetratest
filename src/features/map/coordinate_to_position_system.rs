use std::sync::Arc;

use hecs::World;
use tetra::Context;

use crate::features::{game_state::Assets, shared::Position};

use super::{pointy_hex_to_pixel, Coordinate};

pub fn coordinate_to_position_system(_ctx: &mut Context, world: &mut World, _assets: Arc<Assets>) {
    for (_id, (position, coordinate)) in world.query::<(&mut Position, &Coordinate)>().into_iter() {
        let new_pos = pointy_hex_to_pixel(coordinate.clone());
        position.0 = new_pos;
    }
}
