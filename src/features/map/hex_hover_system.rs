use tetra::Context;

use crate::features::{game_state::in_game::Resources, rendering::Sprite};

use super::{pixel_to_pointy_hex, Coordinate};

pub fn hex_hover_system(ctx: &mut Context, resources: &mut Resources) {
    let camera = &resources.camera;
    let scaler = &resources.scaler.lock().unwrap();
    //let pos = camera.project(scaler.mouse_position(ctx));
    let pos = camera.project(scaler.mouse_position(ctx));

    let coordinate = pixel_to_pointy_hex(pos.x, pos.y);

    resources.last_hovered_hex = Some(coordinate);

    resources
        .world
        .query::<(&mut Sprite, &Coordinate)>()
        .iter()
        .filter(|&(_id, (_, cordinate_comp))| *cordinate_comp == coordinate)
        .for_each(|(_id, (sprite, _))| sprite.color.r = 0.);
}
