use macroquad::prelude::*;
use super::theme::*;

/// Draws the level-complete / end screen.
///
/// Returns `true` when the player presses SPACE to continue.
pub fn draw_end_screen() -> bool {
    let sw = screen_width();
    let sh = screen_height();

    // Full-screen background
    draw_rectangle(0.0, 0.0, sw, sh, BG_DARK);

    // Central panel
    let panel_w = (sw * 0.50).min(460.0).max(280.0);
    let panel_h = (sh * 0.55).min(380.0).max(240.0);
    let panel_x = (sw - panel_w) / 2.0;
    let panel_y = (sh - panel_h) / 2.0;
    draw_panel(panel_x, panel_y, panel_w, panel_h);
    draw_corner_accents(panel_x, panel_y, panel_w, panel_h, 18.0, ACCENT_GREEN);

    // --- Banner placeholder ---
    // Replace with: draw_texture(&ui_assets.banner, banner_x, banner_y, WHITE);
    // Expected asset: assets/ui/banner.png (200x48)
    let banner_w = 200.0;
    let banner_h = 48.0;
    let banner_x = (sw - banner_w) / 2.0;
    let banner_y = panel_y + panel_h * 0.08;
    draw_rectangle(banner_x, banner_y, banner_w, banner_h, Color::new(0.06, 0.14, 0.08, 1.0));
    draw_rectangle_lines(banner_x, banner_y, banner_w, banner_h, 1.0, ACCENT_GREEN);

    // --- Heading ---
    let heading_y = banner_y + banner_h + 40.0;
    draw_centered_text("MISSION COMPLETE", heading_y, 38.0, ACCENT_GREEN);

    // Decorative line
    let line_w = 180.0;
    let line_x = (sw - line_w) / 2.0;
    draw_line(
        line_x, heading_y + 10.0,
        line_x + line_w, heading_y + 10.0,
        1.5, with_alpha(ACCENT_GREEN, 0.5),
    );

    // --- Flavor text ---
    draw_centered_text("McBertlington has escaped!", heading_y + 40.0, 18.0, TEXT_PRIMARY);
    draw_centered_text("The system has been breached.", heading_y + 62.0, 15.0, TEXT_DIM);

    // --- Prompt ---
    let alpha = 0.3 + pulse(2.5) * 0.7;
    let prompt_y = panel_y + panel_h * 0.82;
    draw_centered_text(
        "[ Press SPACE to Continue ]",
        prompt_y,
        20.0,
        with_alpha(ACCENT_AMBER, alpha),
    );

    is_key_pressed(KeyCode::Space)
}
