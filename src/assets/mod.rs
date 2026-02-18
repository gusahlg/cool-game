pub mod definitions;
use definitions::*;

mod constants;
pub use constants::*;

pub struct Assets {
    pub level: Level,
    pub units: Units,
    pub buildings: Buildings,
}

impl Assets {
    pub async fn load() -> Self {

        // Assemble level, units and buildings sprites
        let lvl = Level::load().await;
        let units = Units::load().await;
        let buildings = Buildings::load().await;

        Self {
            level: lvl,
            units: units,
            buildings: buildings,
        }
    }
}


