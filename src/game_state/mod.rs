/// This is the main file for managing game state. This includes what the game should be rendering.
/// Is it the game or selection screen or is ist the level completion screen? Game state also keeps
/// track of state related values such as score and in-game resources. To put it concisely: Game
/// state module handles things that are not directly coupled with the actual rendering of the
/// game, rather it keeps track of data in the background.
use crate::world::levels::Level;
use crate::game_api::bind_level;

pub enum State {
    Game,
    Start,
    Win,
    GameOver,
}

pub struct GameData {
    pub state: State,
    pub supplies: u8,
    pub action: u8,
    blunder_counter: u8,

    pub turn: u8,
}
impl GameData {
    pub fn new() -> Self {
        GameData { state: State::Start,
                   supplies: 0,
                   action: 5,
                   blunder_counter: 0,
                   turn: 1,
        }
    }
    pub fn progress_turn(&mut self, level: &mut Level) {
        self.turn += 1;
        bind_level(level);
    }
    pub fn reset(&mut self) {
        self.supplies = 0;
        self.action = 5;
        self.blunder_counter = 0;
        self.turn = 1;
        self.state = State::Game;
    }

    pub fn increment_and_check_blunder_count(&mut self) -> bool {
        self.blunder_counter += 1;
        if self.blunder_counter == 3 {
            return true;
        }
        false
    }
}
