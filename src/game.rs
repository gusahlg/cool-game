/// some cool rendering and shit
use crate::assets::Assets;
use crate::world::levels::Level;
use crate::zoom::Viewport;

// To extract data for the game api
use crate::game_api::bind_level;

pub fn gameloop(assets: &Assets, level: &mut Level, viewport: &mut Viewport) {
    bind_level(level);
    viewport.update();
    viewport.apply();
    for (y, row) in level.0.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            tile.draw(x as u8, y as u8, assets);
        }
    }
    Viewport::reset();
}
