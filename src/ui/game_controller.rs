use abstract_game_engine::core::simulator::Simulator;
use termion::event::Key;

use crate::core::{dungeon::{MAP_HEIGHT, MAP_WIDTH}, rogue_simulator::RogueSimulator, rogue_state::RogueState, status::Status};

use super::{command::{process_command, Command}, game_view_state::{GameViewState, MainViewState}};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct GameController {
    simulator: RogueSimulator,
    current_state: RogueState,
    pub view_state: GameViewState,
    pub prev_command: Command,
}

impl GameController {
    pub fn new(mut simulator: RogueSimulator) -> GameController {
        let current_state = simulator.generate_initial_state();
        let status = Status::from_rogue_state(&current_state);
        GameController {
            simulator,
            current_state,
            view_state: GameViewState {
                main_view_state: MainViewState {
                    message: format!("Welcome to Rust Rogue version {}. Press '?' for help.", VERSION),
                    map: [['0'; MAP_WIDTH]; MAP_HEIGHT],
                    status,
                },
                overlay_view_state: None,
            },
            prev_command: Command::Continue,
        }
    }

    pub fn process_key(&mut self, key: Key) {
        match key {
            Key::Char(char) => {
                self.prev_command = process_command(
                    &mut self.view_state,
                    &mut self.simulator,
                    &mut self.current_state,
                    &self.prev_command,
                    char,
                )
            },
            _ => {}
        }
    }

    pub fn end_game(&self) -> bool {
        self.prev_command == Command::Exit
    }
}