/// some cool rendering and shit
use crate::assets::Assets;
use crate::world::levels::Level;
use crate::units::UnitKind;
use crate::zoom::Viewport;
use crate::scripting;
use crate::game_state::*;
use crate::ui::*;

pub enum GameResult {
    Continue,
    Win,
    GameOver,
}

// Returns a GameResult to let the main loop know what state to transition to.
pub fn gameloop(assets: &Assets, level: &mut Level, viewport: &mut Viewport, state: &mut GameData, buttons: &mut Vec<Button>, editor: &mut CodeEditor) -> GameResult {
    // Run script from editor when RUN is clicked
    if editor.take_run_request() {
        let source = editor.content();
        match scripting::reload_and_run(&source) {
            Ok(()) => {
                editor.set_error(None);
                state.progress_turn(level);
                if check_king_escaped(level) {
                    return GameResult::Win;
                }
            }
            Err(e) => {
                editor.set_error(Some(e));
                if state.increment_and_check_blunder_count() {
                    return GameResult::GameOver;
                }
                state.progress_turn(level);
            }
        }
    }

    viewport.update();
    viewport.apply();
    for (y, row) in level.0.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            tile.draw(x as u8, y as u8, assets);
        }
    }
    Viewport::reset();

    editor.update();
    editor.draw();
    update_hud(buttons);
    GameResult::Continue
}

/// Check if the King has reached the right, top, or bottom edge of the map.
fn check_king_escaped(level: &Level) -> bool {
    let height = level.0.len();
    let width = if height > 0 { level.0[0].len() } else { return false };

    for (y, row) in level.0.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Some(ref unit) = tile.unit {
                if matches!(unit.kind, UnitKind::King) {
                    return x == width - 1 || y == 0 || y == height - 1;
                }
            }
        }
    }
    false
}

