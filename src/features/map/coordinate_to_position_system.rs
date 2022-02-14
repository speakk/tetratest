use tetra::Context;

use crate::features::{game_state::Resources, shared::Position};

use super::{pointy_hex_to_pixel, Coordinate};

pub fn coordinate_to_position_system(_ctx: &mut Context, resources: &mut Resources) {
    let world = &resources.world;
    for (_id, (position, coordinate)) in world.query::<(&mut Position, &Coordinate)>().into_iter() {
        let new_pos = pointy_hex_to_pixel(*coordinate);
        position.0 = new_pos;
    }
}
