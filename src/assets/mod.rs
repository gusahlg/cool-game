use macroquad::prelude::*;
mod definitions;
use definitions::*;

mod constants;
use constants::*;

pub struct Assets {
    level: Level,
    units: Units,
    buildings: Buildings,
}

impl Assets {
    pub async fn load() -> Self {

        // Assemble level, units and buildings sprites
        let lvl = Level {
            ocean: Texture2D::new(TILE_SIZE, TILE_SIZE),
            tundra: Texture2D::new(TILE_SIZE, TILE_SIZE),
            mountains: Texture2D::new(TILE_SIZE, TILE_SIZE),
            woods: Texture2D::new(TILE_SIZE, TILE_SIZE),
            desert: Texture2D::new(TILE_SIZE, TILE_SIZE),
            fog: Texture2D::new(TILE_SIZE, TILE_SIZE),
            plains: Texture2D::new(TILE_SIZE, TILE_SIZE),

        }
        let units = Units {
            fighter: Texture2D::new(TILE_SIZE, TILE_SIZE), 
            archer: Texture2D::new(TILE_SIZE, TILE_SIZE), 
            catapult: Texture2D::new(TILE_SIZE, TILE_SIZE), 

        }
        let buildings = Buildings {
            town_hall: Texture2D::new(TILE_SIZE, TILE_SIZE),
            farm: Texture2D::new(TILE_SIZE, TILE_SIZE),
            village: Texture2D::new(TILE_SIZE, TILE_SIZE),
        }

        // Load all sprites for different things
        lvl.load();
        units.load();
        buildings.load();

        Self {
            level: lvl,
            units: units,
            buildings: buildings,
        }
    }
}


