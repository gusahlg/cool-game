// Everything about the level
pub struct Level {
    ocean: Texture2D,
    tundra: Texture2D,
    mountains: Texture2D,
    woods: Texture2D,
    desert: Texture2D,
    fog: Texture2D,
    plains: Texture2D,
}
impl Level {
    pub async fn load() -> Self {
            let ocean = load_texture("assets/textures/tiles/ocean.png").await.expect("Failed to load player.png");
            ocean.set_filter(FilterMode::Nearest); 

            let tundra = load_texture("assets/textures/tiles/tundra.png").await.expect("Failed to load player.png");
            tundra.set_filter(FilterMode::Nearest); 

            let mountains = load_texture("assets/textures/tiles/mountains.png").await.expect("Failed to load player.png");
            mountains.set_filter(FilterMode::Nearest); 

            let woods = load_texture("assets/textures/tiles/woods.png").await.expect("Failed to load player.png");
            woods.set_filter(FilterMode::Nearest); 

            let desert = load_texture("assets/textures/tiles/desert.png").await.expect("Failed to load player.png");
            desert.set_filter(FilterMode::Nearest); 

            let fog = load_texture("assets/textures/tiles/fog.png").await.expect("Failed to load player.png");
            fog.set_filter(FilterMode::Nearest); 

            let plains = load_texture("assets/textures/tiles/plains.png").await.expect("Failed to load player.png");
            plains.set_filter(FilterMode::Nearest); 

            Self {
                ocean: ocean,
                tundra: tundra,
                mountains: mountains,
                woods: woods,
                desert: desert,
                fog: fog,
                plains: plains,
            }
    }
}

// Some units for the game
pub struct Units {
    // I am keeping it minimal so that it is within scope plus I named it fighter to not have same
    // name is civ 6 warrior btw. My fingers are clumsy now lol

    fighter: Texture2D, 
    archer: Texture2D, 
    catapult: Texture2D, 
}
impl Units {
    pub async fn load() -> Self {
            let fighter = load_texture("assets/textures/units/fighter.png").await.expect("Failed to load player.png");
            fighter.set_filter(FilterMode::Nearest); 

            let archer = load_texture("assets/textures/units/archer.png").await.expect("Failed to load player.png");
            archer.set_filter(FilterMode::Nearest); 

            let catapult = load_texture("assets/textures/units/catapult.png").await.expect("Failed to load player.png");
            catapult.set_filter(FilterMode::Nearest); 

            Self {
                fighter: fighter,
                archer: archer,
                catapult: catapult,
            }
    }
}

// Buildings for the game
pub struct Buildings {
    // like the city center in civ
    // Nothing is cemented when it comes to builings, we got to keep it minimal so we can't have a
    // massive plan for these, they may be entirely aesthetics for now as implementing features
    // will be quite hard.
    town_hall: Texture2D,
    farm: Texture2D,
    village: Texture2D,
}
impl Buildings {
    pub async fn load() -> Self {
            let town_hall = load_texture("assets/textures/buildings/town_hall.png").await.expect("Failed to load player.png");
            town_hall.set_filter(FilterMode::Nearest); 

            let farm = load_texture("assets/textures/buildings/farm.png").await.expect("Failed to load player.png");
            farm.set_filter(FilterMode::Nearest); 

            let village = load_texture("assets/textures/buildings/village.png").await.expect("Failed to load player.png");
            village.set_filter(FilterMode::Nearest); 

            Self {
                town_hall: town_hall,
                farm: farm,
                village: village,
            }
    }
}
