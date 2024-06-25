use abstract_game_engine::core::simulator::Simulator;
use termion::event::Key;

use crate::core::{rogue_simulator::RogueSimulator, rogue_state::RogueState};

use super::{command::{process_command, Command, MultiCommand}, game_view_state::{GameViewState, MainViewState}, screen::{SCREEN_HEIGHT, SCREEN_WIDTH}};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct ViewModel {
    simulator: RogueSimulator,
    current_state: RogueState,
    pub view_state: GameViewState,
    prev_command: Command,
}

impl ViewModel {
    pub fn new(mut simulator: RogueSimulator) -> ViewModel {
        let current_state = simulator.generate_initial_state();
        ViewModel {
            simulator,
            current_state,
            view_state: GameViewState {
                main_view_state: MainViewState {
                    message: format!("Welcome to Rust Rogue! Version {}", VERSION),
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
}