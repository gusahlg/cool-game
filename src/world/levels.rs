/// Level architecture is defined here!
/// So basically there is a few different kinds of terrain that can be used they also hold
/// information about the tile and different kinds of Tiles can do different things
pub struct Level ( Vec<Vec<Tile>> );

pub struct Tile {
    pub movement_cost: u8,

    // IMPORTANT NOTE:
    // Unit and Building structs are defined in a seperate directory under src called entities.
    // They are accessable throught the prelude.

    //pub unit: Option<Unit>,
    //pub building: Option<Building>,

    // The type of the terrain, specified by the enum
    pub terrain_type: Terrain,
}

enum Terrain {
    // You can't see it
    Fog,
    
    // Can't do anything with it
    Ocean,

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
    pub fn draw(&self, x: u8, y: u8, assets: &Assets::Level) {
        match self.terrain_type {
            Fog => {
                // Draw a fog or someting
            },

            Ocean => {
                // Draw a ocean or someting
            },           

            Woods => {
                // Draw a woods or someting
            },

            Mountain => {
                // Draw a mountain or someting
            },

            Desert => {
                // Draw a desert or someting
            },

            Tundra => {
                // Draw a tundra or someting
            },

            Plains => {
                // Draw a plains or something
            },
        }
    }
}
