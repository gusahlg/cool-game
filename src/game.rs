/// some cool rendering and shit
use crate::assets::Assets;
use crate::world::levels::Level;

pub fn gameloop(assets: &Assets, level: &Level) {
    for (y, row) in level.0.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            tile.draw(x as u8, y as u8, assets);
        }
    }
}
