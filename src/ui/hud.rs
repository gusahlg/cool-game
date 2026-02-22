/// This is the file for managing all in game ui stuff.
use macroquad::prelude::*;
use super::theme::*;
use super::interactives::*;

pub fn update_hud(buttons: &mut Vec<Button>/*, turn: u32, unit_count: u32, mouse_pos: [u8; 2]*/) {
    for btn in buttons.iter_mut() { 
        btn.update();
        btn.draw();
    }
}
