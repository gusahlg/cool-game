/// This file manages all the logic for the simple script editor!
use macroquad::prelude::*;
use super::theme::*;

const PADDING: f32 = 8.0;
const GUTTER_WIDTH: f32 = 40.0;
const LINE_HEIGHT_FACTOR: f32 = 1.4;
const RUN_BUTTON_HEIGHT: f32 = 32.0;
const RUN_BUTTON_MARGIN: f32 = 8.0;

const KEY_REPEAT_DELAY: f32 = 0.4;
const KEY_REPEAT_RATE: f32 = 0.035;

/// Tracks hold time for a key to support initial delay + repeat.
struct KeyRepeat {
    hold_time: f32,
    repeat_timer: f32,
}

impl KeyRepeat {
    fn new() -> Self {
        Self { hold_time: 0.0, repeat_timer: 0.0 }
    }

    /// Returns true on first press and then repeatedly after delay.
    fn update(&mut self, held: bool, dt: f32) -> bool {
        if !held {
            self.hold_time = 0.0;
            self.repeat_timer = 0.0;
            return false;
        }
        let was_zero = self.hold_time == 0.0;
        self.hold_time += dt;
        if was_zero {
            return true; // first frame pressed
        }
        if self.hold_time > KEY_REPEAT_DELAY {
            self.repeat_timer += dt;
            if self.repeat_timer >= KEY_REPEAT_RATE {
                self.repeat_timer -= KEY_REPEAT_RATE;
                return true;
            }
        }
        false
    }
}

pub struct CodeEditor {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
    desired_col: usize,
    scroll_row: usize,
    scroll_col: usize,
    font_size: f32,
    focused: bool,
    run_requested: bool,
    error_msg: Option<String>,
    file_path: Option<String>,
    key_backspace: KeyRepeat,
    key_delete: KeyRepeat,
    key_left: KeyRepeat,
    key_right: KeyRepeat,
    key_up: KeyRepeat,
    key_down: KeyRepeat,
}

struct EditorLayout {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    text_x: f32,
    line_h: f32,
    visible_lines: usize,
    chars_per_line: usize,
    char_w: f32,
    run_btn_x: f32,
    run_btn_y: f32,
    run_btn_w: f32,
    run_btn_h: f32,
}

fn compute_layout(font_size: f32) -> EditorLayout {
    let sw = screen_width();
    let sh = screen_height();

    // Game world is 160x160 (square), scaled to fill screen height, pinned right.
    // So it occupies the rightmost screen_height() pixels.
    let game_width_on_screen = sh;
    let editor_w = (sw - game_width_on_screen - PADDING * 2.0).max(200.0);

    let x = PADDING;
    let y = PADDING;
    let h = sh - PADDING * 2.0 - RUN_BUTTON_HEIGHT - RUN_BUTTON_MARGIN;

    let line_h = font_size * LINE_HEIGHT_FACTOR;
    let text_x = x + GUTTER_WIDTH;
    let text_w = editor_w - GUTTER_WIDTH;

    let char_w = measure_text("M", None, font_size as u16, 1.0).width;
    let visible_lines = ((h - PADDING * 2.0) / line_h).floor().max(1.0) as usize;
    let chars_per_line = ((text_w - PADDING) / char_w).floor().max(1.0) as usize;

    let run_btn_w = editor_w;
    let run_btn_h = RUN_BUTTON_HEIGHT;
    let run_btn_x = x;
    let run_btn_y = y + h + RUN_BUTTON_MARGIN;

    EditorLayout {
        x, y, w: editor_w, h,
        text_x, line_h, visible_lines, chars_per_line, char_w,
        run_btn_x, run_btn_y, run_btn_w, run_btn_h,
    }
}

impl CodeEditor {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_row: 0,
            cursor_col: 0,
            desired_col: 0,
            scroll_row: 0,
            scroll_col: 0,
            font_size: 16.0,
            focused: true,
            run_requested: false,
            error_msg: None,
            file_path: None,
            key_backspace: KeyRepeat::new(),
            key_delete: KeyRepeat::new(),
            key_left: KeyRepeat::new(),
            key_right: KeyRepeat::new(),
            key_up: KeyRepeat::new(),
            key_down: KeyRepeat::new(),
        }
    }

    pub fn load_from_file(&mut self, path: &str) {
        if let Ok(content) = std::fs::read_to_string(path) {
            self.lines = content.lines().map(String::from).collect();
            if self.lines.is_empty() {
                self.lines.push(String::new());
            }
        }
        self.file_path = Some(path.to_string());
    }

    fn save(&self) {
        if let Some(ref path) = self.file_path {
            let _ = std::fs::write(path, self.content());
        }
    }

    pub fn content(&self) -> String {
        self.lines.join("\n")
    }

    pub fn take_run_request(&mut self) -> bool {
        let r = self.run_requested;
        self.run_requested = false;
        r
    }

    pub fn set_error(&mut self, msg: Option<String>) {
        self.error_msg = msg;
    }

    pub fn update(&mut self) {
        let layout = compute_layout(self.font_size);

        // Focus handling + cursor placement via mouse click
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            if mx >= layout.x && mx <= layout.x + layout.w
                && my >= layout.y && my <= layout.y + layout.h
            {
                self.focused = true;
                self.set_cursor_from_click(mx, my, &layout);
            } else if mx >= layout.run_btn_x && mx <= layout.run_btn_x + layout.run_btn_w
                && my >= layout.run_btn_y && my <= layout.run_btn_y + layout.run_btn_h
            {
                self.run_requested = true;
            } else {
                self.focused = false;
            }
        }

        if !self.focused { return; }

        let mut dirty = false;

        // Character input
        while let Some(ch) = get_char_pressed() {
            if ch == '\r' || ch == '\n' {
                let tail = self.lines[self.cursor_row].split_off(self.cursor_col);
                self.cursor_row += 1;
                self.lines.insert(self.cursor_row, tail);
                self.cursor_col = 0;
                self.desired_col = 0;
                dirty = true;
            } else if ch == '\t' {
                self.lines[self.cursor_row].insert_str(self.cursor_col, "    ");
                self.cursor_col += 4;
                self.desired_col = self.cursor_col;
                dirty = true;
            } else if ch as u32 >= 32 {
                self.lines[self.cursor_row].insert(self.cursor_col, ch);
                self.cursor_col += 1;
                self.desired_col = self.cursor_col;
                dirty = true;
            }
        }

        let dt = get_frame_time();

        // Backspace
        if self.key_backspace.update(is_key_down(KeyCode::Backspace), dt) {
            if self.cursor_col > 0 {
                self.cursor_col -= 1;
                self.lines[self.cursor_row].remove(self.cursor_col);
            } else if self.cursor_row > 0 {
                let removed = self.lines.remove(self.cursor_row);
                self.cursor_row -= 1;
                self.cursor_col = self.lines[self.cursor_row].len();
                self.lines[self.cursor_row].push_str(&removed);
            }
            self.desired_col = self.cursor_col;
            dirty = true;
        }

        // Delete
        if self.key_delete.update(is_key_down(KeyCode::Delete), dt) {
            let line_len = self.lines[self.cursor_row].len();
            if self.cursor_col < line_len {
                self.lines[self.cursor_row].remove(self.cursor_col);
                dirty = true;
            } else if self.cursor_row + 1 < self.lines.len() {
                let next = self.lines.remove(self.cursor_row + 1);
                self.lines[self.cursor_row].push_str(&next);
                dirty = true;
            }
        }

        // Arrow keys
        if self.key_left.update(is_key_down(KeyCode::Left), dt) {
            if self.cursor_col > 0 {
                self.cursor_col -= 1;
            } else if self.cursor_row > 0 {
                self.cursor_row -= 1;
                self.cursor_col = self.lines[self.cursor_row].len();
            }
            self.desired_col = self.cursor_col;
        }

        if self.key_right.update(is_key_down(KeyCode::Right), dt) {
            let line_len = self.lines[self.cursor_row].len();
            if self.cursor_col < line_len {
                self.cursor_col += 1;
            } else if self.cursor_row + 1 < self.lines.len() {
                self.cursor_row += 1;
                self.cursor_col = 0;
            }
            self.desired_col = self.cursor_col;
        }

        if self.key_up.update(is_key_down(KeyCode::Up), dt) && self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.cursor_col = self.desired_col.min(self.lines[self.cursor_row].len());
        }

        if self.key_down.update(is_key_down(KeyCode::Down), dt) && self.cursor_row + 1 < self.lines.len() {
            self.cursor_row += 1;
            self.cursor_col = self.desired_col.min(self.lines[self.cursor_row].len());
        }

        if is_key_pressed(KeyCode::Home) {
            self.cursor_col = 0;
            self.desired_col = 0;
        }

        if is_key_pressed(KeyCode::End) {
            self.cursor_col = self.lines[self.cursor_row].len();
            self.desired_col = self.cursor_col;
        }

        self.ensure_cursor_visible(&layout);

        if dirty {
            self.save();
        }
    }

    pub fn draw(&self) {
        let layout = compute_layout(self.font_size);

        // Background panel
        draw_panel(layout.x, layout.y, layout.w, layout.h);
        draw_corner_accents(layout.x, layout.y, layout.w, layout.h, 12.0, ACCENT_CYAN);

        // Gutter background
        draw_rectangle(layout.x, layout.y, GUTTER_WIDTH, layout.h,
            Color::new(0.04, 0.04, 0.07, 0.95));
        draw_line(
            layout.x + GUTTER_WIDTH, layout.y + PADDING,
            layout.x + GUTTER_WIDTH, layout.y + layout.h - PADDING,
            1.0, BORDER,
        );

        // Draw visible lines
        let end_row = (self.scroll_row + layout.visible_lines).min(self.lines.len());
        for i in self.scroll_row..end_row {
            let screen_line = (i - self.scroll_row) as f32;
            let text_y = layout.y + PADDING + screen_line * layout.line_h + self.font_size;

            // Current line highlight (drawn before text)
            if i == self.cursor_row {
                draw_rectangle(
                    layout.text_x, layout.y + PADDING + screen_line * layout.line_h,
                    layout.w - GUTTER_WIDTH, layout.line_h,
                    Color::new(0.12, 0.12, 0.20, 0.3),
                );
            }

            // Line number
            let line_num = format!("{:>3}", i + 1);
            let num_color = if i == self.cursor_row { ACCENT_CYAN } else { TEXT_DIM };
            draw_text(&line_num, layout.x + 4.0, text_y, self.font_size, num_color);

            // Line content (with horizontal scroll)
            let line = &self.lines[i];
            let visible_start = self.scroll_col.min(line.len());
            let visible_end = (self.scroll_col + layout.chars_per_line).min(line.len());
            if visible_start < visible_end {
                let visible_text = &line[visible_start..visible_end];
                draw_text(visible_text, layout.text_x + PADDING, text_y, self.font_size, TEXT_PRIMARY);
            }
        }

        // Blinking cursor, it's opacity is changed to resemble it blinking
        if self.focused
            && self.cursor_row >= self.scroll_row
            && self.cursor_row < self.scroll_row + layout.visible_lines
        {
            let blink = if (get_time() * 5.0).sin() > 0.0 { 1.0 } else { 0.0 };
            let screen_line = (self.cursor_row - self.scroll_row) as f32;
            let cursor_x = layout.text_x + PADDING
                + (self.cursor_col as f32 - self.scroll_col as f32) * layout.char_w;
            let cursor_y = layout.y + PADDING + screen_line * layout.line_h;
            draw_rectangle(cursor_x, cursor_y, 2.0, layout.line_h, with_alpha(ACCENT_GREEN, blink));
        }

        // Run button
        let (mx, my) = mouse_position();
        let hovering = mx >= layout.run_btn_x && mx <= layout.run_btn_x + layout.run_btn_w
            && my >= layout.run_btn_y && my <= layout.run_btn_y + layout.run_btn_h;

        let btn_color = if hovering {
            Color::new(0.0, 0.50, 0.25, 0.95)
        } else {
            Color::new(0.0, 0.35, 0.18, 0.90)
        };
        draw_rectangle(layout.run_btn_x, layout.run_btn_y, layout.run_btn_w, layout.run_btn_h, btn_color);
        draw_rectangle_lines(layout.run_btn_x, layout.run_btn_y, layout.run_btn_w, layout.run_btn_h, 1.0, ACCENT_GREEN);

        let btn_text = "[ RUN ]";
        let dims = measure_text(btn_text, None, 18, 1.0);
        let btn_text_x = layout.run_btn_x + (layout.run_btn_w - dims.width) / 2.0;
        let btn_text_y = layout.run_btn_y + (layout.run_btn_h + dims.height) / 2.0;
        draw_text(btn_text, btn_text_x, btn_text_y, 18.0, ACCENT_GREEN);

        // Error message
        if let Some(ref err) = self.error_msg {
            let err_y = layout.run_btn_y + layout.run_btn_h + 4.0;
            draw_text(err, layout.x + PADDING, err_y + self.font_size, 14.0, ACCENT_AMBER);
        }

        // Scroll indicators
        if self.scroll_row > 0 {
            draw_text("^", layout.x + layout.w - 16.0, layout.y + 14.0, 14.0, TEXT_DIM);
        }
        if end_row < self.lines.len() {
            draw_text("v", layout.x + layout.w - 16.0, layout.y + layout.h - 4.0, 14.0, TEXT_DIM);
        }
    }

    fn set_cursor_from_click(&mut self, mx: f32, my: f32, layout: &EditorLayout) {
        let relative_y = my - layout.y - PADDING;
        let clicked_row = (relative_y / layout.line_h).floor() as usize + self.scroll_row;
        self.cursor_row = clicked_row.min(self.lines.len() - 1);

        let relative_x = mx - layout.text_x - PADDING;
        let clicked_col = ((relative_x / layout.char_w).round() as usize)
            .saturating_add(self.scroll_col);
        self.cursor_col = clicked_col.min(self.lines[self.cursor_row].len());
        self.desired_col = self.cursor_col;
    }

    fn ensure_cursor_visible(&mut self, layout: &EditorLayout) {
        if self.cursor_row < self.scroll_row {
            self.scroll_row = self.cursor_row;
        }
        if self.cursor_row >= self.scroll_row + layout.visible_lines {
            self.scroll_row = self.cursor_row - layout.visible_lines + 1;
        }
        if self.cursor_col < self.scroll_col {
            self.scroll_col = self.cursor_col;
        }
        if self.cursor_col >= self.scroll_col + layout.chars_per_line {
            self.scroll_col = self.cursor_col - layout.chars_per_line + 1;
        }
    }
}
