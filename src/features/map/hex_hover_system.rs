use tetra::{graphics::Color, Context};

use crate::features::{game_state::Resources, rendering::Sprite};

use super::{pixel_to_pointy_hex, Coordinate, Hex};

pub fn hex_hover_system(ctx: &mut Context, resources: &mut Resources) {
    let camera = &resources.camera;
    let scaler = &resources.scaler.lock().unwrap();
    let pos = camera.project(scaler.mouse_position(ctx));
    let coordinate = pixel_to_pointy_hex(pos.x, pos.y);

    resources.last_hovered_hex = Some(coordinate);

    resources
        .world
        .query::<(&mut Sprite, &Coordinate, &Hex)>()
        .iter()
        .filter(|&(_id, (_, cordinate_comp, _))| *cordinate_comp == coordinate)
        .for_each(|(_id, (sprite, _, _))| sprite.color = Color::rgb(1., 0.6, 0.6));
}
