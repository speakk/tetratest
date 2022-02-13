use hecs::World;
use tetra::{graphics::Camera, input::MouseButton, Context};

use crate::features::units::{Selected, Unit};

use super::{pixel_to_pointy_hex, Coordinate};

pub fn map_click_handler(
    ctx: &mut Context,
    world: &mut World,
    mouse_button: MouseButton,
    camera: &Camera,
) {
    let pos = camera.mouse_position(ctx);
    let coordinate = pixel_to_pointy_hex(pos.x, pos.y);

    match mouse_button {
        MouseButton::Left => {
            let units = world
                .query::<(&Unit, &Coordinate)>()
                .iter()
                .filter(|&(_id, (_, cordinate_comp))| *cordinate_comp == coordinate)
                .map(|(id, (_, _))| id.clone())
                .collect::<Vec<_>>();

            let unit = units.last();

            if let Some(unit) = unit {
                world.insert_one(*unit, Selected).unwrap();
            }
        }
        MouseButton::Right => {
            let selecteds = world
                .query::<&Selected>()
                .iter()
                .map(|(id, _)| id.clone())
                .collect::<Vec<_>>();

            for selected in selecteds.iter() {
                world.remove_one::<Selected>(*selected).unwrap();
            }
        }
        _ => {}
    }
}
