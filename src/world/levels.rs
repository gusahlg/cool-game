/// Level architecture is defined here!
/// So basically there is a few different kinds of terrain that can be used they also hold
/// information about the tile and different kinds of Tiles can do different things
use macroquad::prelude::*;
use crate::assets::Assets;
use crate::assets::TILE_SIZE;

// From the units directory, defines the Unit type
use crate::units::*;

pub struct Level ( pub Vec<Vec<Tile>> );

pub struct Tile {
    pub movement_cost: u8,

    // IMPORTANT NOTE:
    // Unit and Building structs are defined in a seperate directory under src called units.
    pub unit: Option<Unit>,

    //pub building: Option<Building>,

    // The type of the terrain, specified by the enum
    pub terrain_type: Terrain,

}

pub enum Terrain {
    // You can't see it
    Fog,

    // Can't do anything with it
    Void,

    // Higher movement cost
    Woods,

    // Super high cost
    Mountain,

    // Desert (not dessert lol), maybe has some cool implications but prob medium movement cost.
    Desert,

    // High cost and like desert maybe some special features later on (taking damage or something)
    Tundra,

    // Plains, low cost and just a kinda chill place
    Plains,
}
impl Tile {
    pub fn draw(&self, x: u8, y: u8, assets: &Assets) {
        let px = x as f32 * TILE_SIZE as f32;
        let py = y as f32 * TILE_SIZE as f32;

                
        match self.terrain_type {
            Terrain::Fog => {
                draw_texture(&assets.level.fog, px, py, WHITE);
            },

            Terrain::Void => {
                draw_texture(&assets.level.ocean, px, py, WHITE);
            },

            Terrain::Woods => {
                draw_texture(&assets.level.woods, px, py, WHITE);
            },

            Terrain::Mountain => {
                draw_texture(&assets.level.mountains, px, py, WHITE);
            },

            Terrain::Desert => {
                draw_texture(&assets.level.desert, px, py, WHITE);
            },

            Terrain::Tundra => {
                draw_texture(&assets.level.tundra, px, py, WHITE);
            },

            Terrain::Plains => {
                draw_texture(&assets.level.plains, px, py, WHITE);
            },
        }
            
        // Thing I need to do is to draw the units on top of the terrrain
        match &self.unit {
            Some(Unit { kind, .. }) => match kind {
                UnitKind::Fighter => {
                    draw_texture(&assets.units.fighter, px, py, WHITE);
                },
                UnitKind::Archer => {
                    draw_texture(&assets.units.archer, px, py, WHITE);
                },
                UnitKind::Catapult => {  
                    draw_texture(&assets.units.catapult, px, py, WHITE);
                },
                UnitKind::King => {
                    draw_texture(&assets.units.king, px, py, WHITE);
                }
            },
            None => {  }
        }
    }
}
