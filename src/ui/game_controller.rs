use abstract_game_engine::core::simulator::Simulator;
use termion::event::Key;

use crate::core::{rogue_simulator::RogueSimulator, rogue_state::RogueState};

use super::{command::{process_command, Command, ExitCommand}, game_screen::{SCREEN_HEIGHT, SCREEN_WIDTH}, game_view_state::{GameViewState, MainViewState}};

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
        GameController {
            simulator,
            current_state,
            view_state: GameViewState {
                main_view_state: MainViewState {
                    message: format!("Welcome to Rust Rogue version {}. Press '?' for help.", VERSION),
                    map: [[' '; SCREEN_WIDTH]; SCREEN_HEIGHT - 2],
                    status: "".to_string(),
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
        self.prev_command == Command::Exit(ExitCommand::Quit)
    }
}