use macroquad::prelude::*;
use super::theme::*;

/// Draws the game over screen.
///
/// Returns `true` when the player presses SPACE to restart.
pub fn draw_game_over_screen() -> bool {
    let sw = screen_width();
    let sh = screen_height();

    draw_rectangle(0.0, 0.0, sw, sh, BG_DARK);

    let panel_w = (sw * 0.50).min(460.0).max(280.0);
    let panel_h = (sh * 0.55).min(380.0).max(240.0);
    let panel_x = (sw - panel_w) / 2.0;
    let panel_y = (sh - panel_h) / 2.0;
    draw_panel(panel_x, panel_y, panel_w, panel_h);
    draw_corner_accents(panel_x, panel_y, panel_w, panel_h, 18.0, ACCENT_AMBER);

    let heading_y = panel_y + panel_h * 0.25;
    draw_centered_text("SYSTEM FAILURE", heading_y, 38.0, ACCENT_AMBER);

    let line_w = 180.0;
    let line_x = (sw - line_w) / 2.0;
    draw_line(
        line_x, heading_y + 10.0,
        line_x + line_w, heading_y + 10.0,
        1.5, with_alpha(ACCENT_AMBER, 0.5),
    );

    draw_centered_text("Too many blunders.", heading_y + 40.0, 18.0, TEXT_PRIMARY);
    draw_centered_text("McBertlington could not escape.", heading_y + 62.0, 15.0, TEXT_DIM);

    let alpha = 0.3 + pulse(2.5) * 0.7;
    let prompt_y = panel_y + panel_h * 0.82;
    draw_centered_text(
        "[ Press SPACE to Try Again ]",
        prompt_y,
        20.0,
        with_alpha(ACCENT_AMBER, alpha),
    );

    is_key_pressed(KeyCode::Space)
}
