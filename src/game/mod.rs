use macroquad::prelude::*;

pub async fn game_loop() {
    let game_running: bool = true;
    while game_running {
        clear_background(BLACK);
        draw_rectangle(100.0, 100.0, 50.0, 50.0, RED);
        next_frame().await;
    }
}
