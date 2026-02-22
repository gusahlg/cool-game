use macroquad::prelude::*;
use super::theme::*;

/// Draws the title/start screen.
///
/// Returns `true` when the player presses SPACE to begin.
pub fn draw_start_screen() -> bool {
    let sw = screen_width();
    let sh = screen_height();

    // Full-screen dark background
    draw_rectangle(0.0, 0.0, sw, sh, BG_DARK);

    // Subtle scanline overlay
    let scanline_color = Color::new(0.0, 0.0, 0.0, 0.08);
    let mut ly = 0.0;
    while ly < sh {
        draw_line(0.0, ly, sw, ly, 1.0, scanline_color);
        ly += 4.0;
    }

    // Central panel
    let panel_w = (sw * 0.55).min(500.0).max(300.0);
    let panel_h = (sh * 0.65).min(450.0).max(280.0);
    let panel_x = (sw - panel_w) / 2.0;
    let panel_y = (sh - panel_h) / 2.0;
    draw_panel(panel_x, panel_y, panel_w, panel_h);
    draw_corner_accents(panel_x, panel_y, panel_w, panel_h, 18.0, ACCENT_CYAN);

    // --- Logo placeholder ---
    // Replace with: draw_texture(&ui_assets.logo, logo_x, logo_y, WHITE);
    // Expected asset: assets/ui/logo.png (64x64)
    let logo_size = 64.0;
    let logo_x = (sw - logo_size) / 2.0;
    let logo_y = panel_y + panel_h * 0.06;
    draw_rectangle(logo_x, logo_y, logo_size, logo_size, Color::new(0.08, 0.12, 0.18, 1.0));
    draw_rectangle_lines(logo_x, logo_y, logo_size, logo_size, 1.0, ACCENT_CYAN);
    draw_text(">_", logo_x + 14.0, logo_y + 42.0, 32.0, ACCENT_GREEN);

    // --- Title ---
    let title_y = logo_y + logo_size + 45.0;
    draw_centered_text("escape.exe", title_y, 46.0, ACCENT_GREEN);

    // Underline
    let ul_w = 200.0;
    let ul_x = (sw - ul_w) / 2.0;
    draw_line(ul_x, title_y + 8.0, ul_x + ul_w, title_y + 8.0, 1.5, with_alpha(ACCENT_GREEN, 0.5));

    // --- Subtitle ---
    draw_centered_text("A Programming Strategy Game", title_y + 35.0, 18.0, TEXT_DIM);

    // --- Animated prompt ---
    let alpha = 0.3 + pulse(2.5) * 0.7;
    let prompt_y = panel_y + panel_h * 0.80;
    draw_centered_text(
        "[ Press SPACE to Start ]",
        prompt_y,
        22.0,
        with_alpha(ACCENT_AMBER, alpha),
    );

    // --- Version ---
    draw_centered_text("v0.1.0", panel_y + panel_h - 18.0, 13.0, TEXT_DIM);

    is_key_pressed(KeyCode::Space)
}
