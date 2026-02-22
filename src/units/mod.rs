/// This is the main file for defining unit behaviour.
/// So there will be a Unit struct with a UnitKind enum for different types of units. Dynamic
/// dispatch is not needed since this is only a lower level api for the scripting. 

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

pub enum UnitKind {
    Archer,
    Fighter,
    Catapult,
    King,
}   

pub struct Unit {
    pub hp: u8,
    max_hp: u8,
    dmg: u8,
    pub position: [u8; 2],

    // Determines what type of unit it is
    pub kind: UnitKind,
}
impl Unit {
    pub fn travel(&mut self, dir: Direction) {
        // Sorry for the disgusting freakish code
        match dir {
            Direction::Up => { self.position[1] = self.position[1].saturating_sub(1); },
            Direction::Down => { self.position[1] += 1; },
            Direction::Left => { self.position[0] = self.position[0].saturating_sub(1); },
            Direction::Right => { self.position[0] += 1; },
            Direction::UpRight => { self.position[1] = self.position[1].saturating_sub(1); self.position[0] += 1; },
            Direction::UpLeft => { self.position[1] = self.position[1].saturating_sub(1); self.position[0] = self.position[0].saturating_sub(1); },
            Direction::DownRight => { self.position[1] += 1; self.position[0] += 1; },
            Direction::DownLeft => { self.position[1] += 1; self.position[0] = self.position[0].saturating_sub(1); },
        }
    }
    pub fn is_dead(&self) -> bool {
        if self.hp == 0 {
            return true;
        }
        else {return false;}
    }
    pub fn archer(pos: [u8; 2]) -> Self {
        Self { hp: 5,
               max_hp: 5,
               dmg: 3,
               position: pos,
               kind: UnitKind::Archer,
        }
    }
    pub fn fighter(pos: [u8; 2]) -> Self {
        Self { hp: 7,
               max_hp: 7,
               dmg: 4,
               position: pos,
               kind: UnitKind::Fighter,
        }
    }
    pub fn catapult(pos: [u8; 2]) -> Self {
        Self { hp: 6,
               max_hp: 6,
               dmg: 8,
               position: pos,
               kind: UnitKind::Catapult,
        }
    }
    pub fn king(pos: [u8; 2]) -> Self {
        Self { hp: 6,
               max_hp: 6,
               dmg: 8,
               position: pos,
               kind: UnitKind::King,
        }
    }
}
