use macroquad::prelude::*;
/// This file add in interactive ui elements such as buttons

pub struct Button {
    pos: [u8; 2],
    state: State,
    last_state: State,
    width: u8,
    height: u8,
}
impl Button {
    pub fn draw(&self) {
        match self.state {
            State::Normal => {
                draw_rectangle(
                    self.pos[0] as f32,
                    self.pos[1] as f32,
                    self.width as f32,
                    self.height as f32,
                    Color::new(0.87, 0.10, 0.10, 1.0),
                );
            }
            State::Pressed => {
                // do integer math safely (avoid underflow)
                let x = self.pos[0].saturating_sub(5);
                let y = self.pos[1].saturating_sub(5);
                let w = self.width.saturating_sub(5);
                let h = self.height.saturating_sub(5);

                draw_rectangle(
                    x as f32,
                    y as f32,
                    w as f32,
                    h as f32,
                    Color::new(0.89, 0.10, 0.12, 0.85),
                );
            }
        }
    }
    pub fn is_clicked(&mut self) -> bool {
        // Checking if it is a click and resetting last_state
        if self.state == State::Normal && self.last_state == State::Pressed {
            self.last_state = State::Normal;
            return true;
        }

        false
    }
    pub fn update(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        // Collision detection
        let coll: bool;
        if mouse_x >= self.pos[0] as f32 && mouse_x <= self.pos[0] as f32 + self.width as f32 &&
           mouse_y >= self.pos[1] as f32 && mouse_y <= self.pos[1] as f32 + self.height as f32 {
               coll = true;
        } else { coll = false; }

        // Make new last state
        self.last_state = self.state.clone();

        // Do checks to determine new state
        if coll && is_mouse_button_pressed(MouseButton::Left) {
            self.state = State::Pressed;
        } else { self.state = State::Normal; }
    }

    pub fn new(pos: [u8; 2], width: u8, height: u8) -> Button {
        Button{pos: pos, state: State::Normal, last_state: State::Normal, width: width, height: height}
    }

    pub fn resize(&mut self, x: u8, y: u8, width: u8, height: u8) {
        self.pos[0] = x; self.pos[1] = y; self.width = width; self.height = height;
    }
}
#[derive(PartialEq, Eq, Clone)]
enum State {
    Normal,
    Pressed,
}
