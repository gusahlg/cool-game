/// Simple level generator with pure noise terrain and a start area.
use macroquad::rand::gen_range;
use super::levels::*;
use crate::units::Unit;

fn random_terrain() -> (Terrain, u8) {
    let roll = gen_range(0, 100);
    match roll {
        0..45  => (Terrain::Plains, 1),
        45..65 => (Terrain::Woods, 2),
        65..80 => (Terrain::Mountain, 4),
        80..90 => (Terrain::Desert, 3),
        90..97 => (Terrain::Tundra, 3),
        _      => (Terrain::Void, 255),
    }
}

pub fn generate_level(width: usize, height: usize) -> Level {
    let mut grid = Vec::with_capacity(height);

    for y in 0..height {
        let mut row = Vec::with_capacity(width);
        for x in 0..width {
            let (terrain, cost) = // Start area: 8x8 plains in the bottom-left corner
                if x < 8 && y >= height - 8 {
                    (Terrain::Plains, 1)
                } else {
                    random_terrain()
                };

            let unit = match (x, y) {
                (3, _) if y == height - 3 => Some(Unit::king([x as u8, y as u8])),
                (1, _) if y == height - 2 => Some(Unit::fighter([x as u8, y as u8])),
                (5, _) if y == height - 2 => Some(Unit::archer([x as u8, y as u8])),
                _ => None,
            };

            row.push(Tile {
                movement_cost: cost,
                terrain_type: terrain,
                unit,
            });
        }
        grid.push(row);
    }

    Level(grid)
}
