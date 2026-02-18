mod game;
mod assets;
mod world;
mod units;

use game::gameloop;
use macroquad::prelude::*;
use assets::Assets;
use world::levels::{Level, Tile, Terrain};
use units::{Unit};

#[macroquad::main("cool_game")]
async fn main() {
    // NEW PLAN. ADD IN positions for units when initializing the levels by iterating over x and y
    // pos.
    let assets = Assets::load().await;

    let mut level = Level(Vec::new());
    for y in 0..10 {
        let mut row: Vec<Tile> = Vec::new();
        for x in 0..10 {
            row.push(
                Tile { movement_cost: 5, terrain_type: Terrain::Fog, unit: Some(Unit::archer([x,y])) }
            );
        }
        level.0.push(row);
    }

    loop {
        clear_background(BLACK);
        gameloop(&assets, &level);
        next_frame().await;
    }
}
