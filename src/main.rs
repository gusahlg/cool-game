mod game;
mod level; 
use crate::game::{game_loop};

#[macroquad::main("Teknik Game")]
async fn main() {
    game_loop().await;
}

