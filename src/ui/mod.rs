/// Here is all the code for ui and the different screens.
///
/// # Screens
/// - `draw_start_screen()` – Title screen shown at game start
/// - `draw_end_screen()` – Completion screen shown when a level ends
/// - `draw_hud()` – In-game overlay (turn counter, unit info, controls)
///
/// # Texture Placeholders
/// The screens reference placeholder areas for the following assets
/// that should be placed in `assets/ui/`:
/// - `logo.png` (64x64) – Game logo for the start screen
/// - `banner.png` (200x48) – Victory banner for the end screen

mod theme;
mod start_screen;
mod end_screen;
mod game_over_screen;
mod hud;
mod interactives;
mod editor;

pub use start_screen::draw_start_screen;
pub use end_screen::draw_end_screen;
pub use game_over_screen::draw_game_over_screen;
pub use hud::*;
pub use interactives::*;
pub use editor::CodeEditor;
