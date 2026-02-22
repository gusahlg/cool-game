use macroquad::prelude::*;

// This file describes the theme of the game, this includes colors and helper functions.

pub const BG_DARK: Color = Color::new(0.05, 0.05, 0.09, 1.0);
pub const BG_PANEL: Color = Color::new(0.08, 0.08, 0.15, 0.92);

pub const ACCENT_GREEN: Color = Color::new(0.0, 0.85, 0.40, 1.0);
pub const ACCENT_CYAN: Color = Color::new(0.0, 0.70, 0.85, 1.0);
pub const ACCENT_AMBER: Color = Color::new(0.95, 0.75, 0.10, 1.0);

pub const TEXT_PRIMARY: Color = Color::new(0.90, 0.90, 0.93, 1.0);
pub const TEXT_DIM: Color = Color::new(0.50, 0.50, 0.60, 1.0);

pub const BORDER: Color = Color::new(0.22, 0.22, 0.35, 1.0);

/// Draw text horizontally centered at the given y position.
pub fn draw_centered_text(text: &str, y: f32, font_size: f32, color: Color) {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    let x = (screen_width() - dims.width) / 2.0;
    draw_text(text, x, y, font_size, color);
}

/// Draw a panel (filled rectangle with border).
pub fn draw_panel(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, BG_PANEL);
    draw_rectangle_lines(x, y, w, h, 1.0, BORDER);
}

/// Draw decorative corner brackets on a rectangle.
pub fn draw_corner_accents(x: f32, y: f32, w: f32, h: f32, len: f32, color: Color) {
    let t = 2.0;
    // Top-left
    draw_line(x, y, x + len, y, t, color);
    draw_line(x, y, x, y + len, t, color);
    // Top-right
    draw_line(x + w, y, x + w - len, y, t, color);
    draw_line(x + w, y, x + w, y + len, t, color);
    // Bottom-left
    draw_line(x, y + h, x + len, y + h, t, color);
    draw_line(x, y + h, x, y + h - len, t, color);
    // Bottom-right
    draw_line(x + w, y + h, x + w - len, y + h, t, color);
    draw_line(x + w, y + h, x + w, y + h - len, t, color);
}

/// Returns a pulsing value (0.0 to 1.0) based on elapsed time.
pub fn pulse(speed: f64) -> f32 {
    ((get_time() * speed).sin() * 0.5 + 0.5) as f32
}

/// Create a color with modified alpha.
pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color::new(color.r, color.g, color.b, alpha)
}
