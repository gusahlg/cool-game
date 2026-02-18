/// This is the api for the game that the scripting language can use to interact with the game!
/// The main interactions are moving units and executing unit actions. This file also handles more
/// complex logic and checks whilst the rest of the code is low level abstractions, the core
/// gameplay logic is here!
mod bindings;
use crate::units::*;
use crate::world::levels::*;
use Direction;
pub use bindings::LEVEL_PTR;
pub use bindings::bind_level;

pub fn get_units() -> Vec<&'static mut Unit> {
  unsafe {
      (*LEVEL_PTR).0.iter_mut()
          .flat_map(|row| row.iter_mut())
          .filter_map(|tile| tile.unit.as_mut())
          .collect()
  }
}
pub fn move_unit(unit: &mut Unit, dir: Direction) {
    if unit.position[0] == 0 {
        match dir {
            Direction::Left => return,
            Direction::UpLeft => return,
            Direction::DownLeft => return,
            _ => {}
        }
    }
    else if unit.position[1] == 0 {
        match dir {
            Direction::Up => return,
            Direction::UpRight => return,
            Direction::UpLeft => return,
            _ => {}
        }
    }
    unit.travel(dir);
}
