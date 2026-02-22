mod game;
mod assets;
mod world;
mod units;
mod zoom;
mod game_api;
mod game_state;
mod scripting;
mod ui;

use game::{gameloop, GameResult};
use macroquad::prelude::*;
use assets::Assets;
use world::generator::generate_level;
use zoom::Viewport;
use game_state::*;
use game_api::bind_level;
use ui::*;
use macroquad::audio::*;

#[macroquad::main("escape.exe")]
async fn main() {
    // Some super mega cool music for la game
    let music = load_sound("assets/music.ogg").await.unwrap();

    play_sound(
        &music,
        PlaySoundParams {
            looped: true,
            volume: 0.5,
        },
    );

    let assets = Assets::load().await;

    let mut level = generate_level(75, 75);

    let mut viewport = Viewport::new(75.0 * 16.0, 75.0 * 16.0);
    let mut game_data = GameData::new();

    // Initialize scripting engine
    bind_level(&mut level);
    let mut engine = ahl_lang::AhlEngine::new();
    scripting::setup_engine(&mut engine, "assets/scripting/turn.ahl");
    scripting::bind_engine(&mut engine);

    let mut editor = CodeEditor::new();
    editor.load_from_file("assets/script.ahl");

    // Here all buttons are defined
    let mut buttons: Vec<Button> = vec!(
        Button::new(
/*position*/[0, 0],
/*width*/   0,
/*height*/  0,
        ),
    );

    loop {
        clear_background(BLACK);
        match game_data.state {
            State::Game => {
                match gameloop(&assets, &mut level, &mut viewport, &mut game_data, &mut buttons, &mut editor) {
                    GameResult::Win => game_data.state = State::Win,
                    GameResult::GameOver => game_data.state = State::GameOver,
                    GameResult::Continue => {}
                }
            }
            State::Start => { if draw_start_screen() { game_data.state = State::Game; } }
            State::Win => {
                if draw_end_screen() {
                    level = generate_level(75, 75);
                    viewport = Viewport::new(75.0 * 16.0, 75.0 * 16.0);
                    bind_level(&mut level);
                    game_data.reset();
                }
            }
            State::GameOver => {
                if draw_game_over_screen() {
                    level = generate_level(75, 75);
                    viewport = Viewport::new(75.0 * 16.0, 75.0 * 16.0);
                    bind_level(&mut level);
                    game_data.reset();
                }
            }
        }
        next_frame().await;
    }
}
