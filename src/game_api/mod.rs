/// This is the api for the game that the scripting language can use to interact with the game!
/// The main interactions are moving units and executing unit actions. This file also handles more
/// complex logic and checks whilst the rest of the code is low level abstractions, the core
/// gameplay logic is here!
mod bindings;
use crate::units::*;
use crate::world::levels::*;
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
pub fn spawn_unit(unit: Unit, x: u8, y: u8) {
    unsafe {
        let level = &mut *LEVEL_PTR;
        let tile = &mut level.0[y as usize][x as usize];
        tile.unit = Some(unit);
    }
}

pub fn get_terrain(x: usize, y: usize) -> Result<String, String> {
    unsafe {
        let level = &*LEVEL_PTR;
        let row = level.0.get(y).ok_or_else(|| format!("y={} out of bounds", y))?;
        let tile = row.get(x).ok_or_else(|| format!("x={} out of bounds", x))?;
        let name = match tile.terrain_type {
            Terrain::Fog => "fog",
            Terrain::Void => "void",
            Terrain::Woods => "woods",
            Terrain::Mountain => "mountain",
            Terrain::Desert => "desert",
            Terrain::Tundra => "tundra",
            Terrain::Plains => "plains",
        };
        Ok(name.to_string())
    }
}

/// Move a unit by index. Takes it out of its current tile and places it in the new one.
pub fn move_unit(index: usize, dir: Direction) {
    unsafe {
        let level = &mut *LEVEL_PTR;
        let height = level.0.len();
        let width = if height > 0 { level.0[0].len() } else { return };

        // Find the unit's current tile by index
        let mut count = 0usize;
        let mut old_x = 0usize;
        let mut old_y = 0usize;
        let mut found = false;
        for (y, row) in level.0.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.unit.is_some() {
                    if count == index {
                        old_x = x;
                        old_y = y;
                        found = true;
                        break;
                    }
                    count += 1;
                }
            }
            if found { break; }
        }
        if !found { return; }

        // Compute new position with boundary clamping
        let (dx, dy): (i32, i32) = match dir {
            Direction::Up        => ( 0, -1),
            Direction::Down      => ( 0,  1),
            Direction::Left      => (-1,  0),
            Direction::Right     => ( 1,  0),
            Direction::UpLeft    => (-1, -1),
            Direction::UpRight   => ( 1, -1),
            Direction::DownLeft  => (-1,  1),
            Direction::DownRight => ( 1,  1),
        };

        let new_x = (old_x as i32 + dx).clamp(0, width as i32 - 1) as usize;
        let new_y = (old_y as i32 + dy).clamp(0, height as i32 - 1) as usize;

        // Don't move if destination is the same or already occupied
        if (new_x == old_x && new_y == old_y) || level.0[new_y][new_x].unit.is_some() {
            return;
        }

        // Move the unit between tiles and update its position field
        let mut unit = level.0[old_y][old_x].unit.take().unwrap();
        unit.position = [new_x as u8, new_y as u8];
        level.0[new_y][new_x].unit = Some(unit);
    }
}
