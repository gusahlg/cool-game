mod game;
mod assets;
mod world;
mod units;
mod zoom;
mod game_api;

use game::gameloop;
use macroquad::prelude::*;
use assets::Assets;
use world::levels::{Level, Tile, Terrain};
use units::{Unit};
use zoom::Viewport;

#[macroquad::main("escape.exe")]
async fn main() {
    let assets = Assets::load().await;

    let mut level = Level(Vec::new());
    for y in 0..10 {
        let mut row: Vec<Tile> = Vec::new();
        for x in 0..10 {
            row.push(
                Tile { movement_cost: 5, terrain_type: Terrain::Plains, unit: None }
            );
        }
        level.0.push(row);
    }

    let mut viewport = Viewport::new(160.0, 160.0);

    loop {
        clear_background(BLACK);
        gameloop(&assets, &mut level, &mut viewport);
        next_frame().await;
    }
}
