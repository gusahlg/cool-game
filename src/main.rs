mod game;
use game::{gameloop};
use macroquad::prelude::*;

#[macroquad::main("cool_game")]
async fn main() {
    loop {
        // Update screen state
        let dt = get_frame_time();
        clear_background(BLACK);
        gameloop();
        next_frame().await;
    }
}

