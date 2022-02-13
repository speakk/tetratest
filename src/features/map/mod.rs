use std::cmp;
use tetra::math::Vec2;

pub mod coordinate_to_position_system;
pub mod hex_hover_system;
pub mod map_click_handler;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub q: i32,
    pub r: i32,
}

const HEX_SIZE: f32 = (32.0 / 2.0) * 1.1;
const HEX_LAYOUT_SIZE_X: f32 = HEX_SIZE;
const HEX_LAYOUT_SIZE_Y: f32 = HEX_SIZE * 0.82;

pub struct Matrix {
    f0: f32,
    f1: f32,
    f2: f32,
    f3: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    b3: f32,
}

lazy_static::lazy_static! {
    pub static ref POINTY_HEX_MATRIX: Matrix = Matrix {
        f0: 3_f32.sqrt(),
        f1: 3_f32.sqrt() / 2.0,
        f2: 0.0,
        f3: 3.0 / 2.0,
        b0: 3_f32.sqrt() / 3.0,
        b1: -1.0 / 3.0,
        b2: 0.0,
        b3: 2.0 / 3.0,
    };
}

lazy_static::lazy_static! {
    pub static ref NEIGHBOR_DIRECTIONS: Vec<Coordinate> = vec![
        Coordinate { q: 1, r: 0 },
        Coordinate { q: 1, r: -1 },
        Coordinate { q: 0, r: -1 },
        Coordinate { q: -1, r: 0 },
        Coordinate { q: -1, r: 1 },
        Coordinate { q: 0, r: 1 },
    ];
}

// pub fn axial_distance(a: Coordinate, b: Coordinate) -> i32 {
//     ((a.q - b.q).abs() + (a.q + a.r - b.q - b.r).abs() + (a.r - b.r).abs()) / 2
// }

pub fn pointy_hex_to_pixel(coordinate: Coordinate) -> Vec2<f32> {
    let q = coordinate.q as f32;
    let r = coordinate.r as f32;
    let x = (POINTY_HEX_MATRIX.f0 * q + POINTY_HEX_MATRIX.f1 * r) * HEX_LAYOUT_SIZE_X;
    let y = (POINTY_HEX_MATRIX.f2 * q + POINTY_HEX_MATRIX.f3 * r) * HEX_LAYOUT_SIZE_Y;

    Vec2::new(x, y)
}

pub fn pixel_to_pointy_hex(x: f32, y: f32) -> Coordinate {
    let x = x / HEX_LAYOUT_SIZE_X;
    let y = y / HEX_LAYOUT_SIZE_Y;
    let q = POINTY_HEX_MATRIX.b0 * x + POINTY_HEX_MATRIX.b1 * y;
    let r = POINTY_HEX_MATRIX.b2 * x + POINTY_HEX_MATRIX.b3 * y;
    axial_round(q, r)
}

pub fn axial_round(fraq_q: f32, fraq_r: f32) -> Coordinate {
    let fraq_s = -fraq_q - fraq_r;
    let mut q = fraq_q.round();
    let mut r = fraq_r.round();
    let s = fraq_s.round();

    let q_diff = (q - fraq_q).abs();
    let r_diff = (r - fraq_r).abs();
    let s_diff = (s - fraq_s).abs();

    if q_diff > r_diff && q_diff > s_diff {
        q = -r - s;
    } else if r_diff > s_diff {
        r = -q - s;
    }

    Coordinate {
        q: q as i32,
        r: r as i32,
    }
}

pub enum MapShape {
    Hexagonal,
    Square,
}

pub fn create_grid(radius: i32, shape: MapShape) -> Vec<Coordinate> {
    let mut hexes: Vec<Coordinate> = vec![];

    match shape {
        MapShape::Hexagonal => {
            for q in -radius..radius {
                let r1: i32 = cmp::max(-radius, -q - radius);
                let r2: i32 = cmp::min(radius, -q + radius);

                for r in r1..r2 {
                    hexes.push(Coordinate { q, r });
                }
            }
        }
        MapShape::Square => {
            let top = ((-radius / 2) as f32).floor() as i32;
            let left = ((-radius / 2) as f32).floor() as i32;
            let bottom = ((radius / 2) as f32).floor() as i32;
            let right = ((radius / 2) as f32).floor() as i32;
            for r in top..bottom {
                let r_offset = ((r / 2) as f32).floor() as i32;
                for q in left - r_offset..right - r_offset {
                    hexes.push(Coordinate { q, r });
                }
            }
        }
    }

    hexes
}
