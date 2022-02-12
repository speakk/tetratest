use crate::features::map::NEIGHBOR_DIRECTIONS;
use pathfinding::prelude::bfs;

use crate::features::map::Coordinate;

impl Coordinate {
    fn successors(
        &self,
        //coordinates_to_hex: &CoordinateToHex,
        //hex_occupants: &HexOccupants,
    ) -> Vec<Coordinate> {
        //println!("Looking for successors");
        let Coordinate { q, r } = self;
        let nodes: Vec<Coordinate> = NEIGHBOR_DIRECTIONS
            .clone()
            .into_iter()
            .map(|dir| Coordinate {
                q: q + dir.q,
                r: r + dir.r,
            })
            // .filter(|coord| coordinates_to_hex.0.contains_key(coord))
            // .filter(|coord| {
            //     hex_occupants
            //         .0
            //         .get(coord)
            //         .filter(|occupants| !occupants.is_empty())
            //         .is_none()
            // })
            .collect();

        nodes
    }
}

pub fn get_path(
    from: Coordinate,
    to: Coordinate,
    // coordinates_to_hex: &CoordinateToHex,
    // hex_occupants: &HexOccupants,
) -> Option<Vec<Coordinate>> {
    bfs(
        &from,
        //|p| p.successors(coordinates_to_hex, hex_occupants),
        |p| p.successors(),
        |p| *p == to,
    )
}
