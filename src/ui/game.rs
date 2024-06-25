use std::io::{stdin, Result};

use termion::input::TermRead;

use crate::core::rogue_simulator::RogueSimulator;

use super::{command::{Command, ExitCommand}, game_screen::GameScreen, game_view::GameView, view_model::ViewModel};

/// Create and play a new game.
pub struct Game {
    game_view: GameView,
    view_model: ViewModel,
}

impl Game {
    pub fn new(
        screen_wrapper: GameScreen,
    ) -> Game {
        Game { game_view: GameView::new(screen_wrapper), view_model: ViewModel::new(RogueSimulator::new()) }
    }

    pub fn new_from_seed(
        screen_wrapper: GameScreen,
        seed: u64,
    ) -> Game {
        Game { game_view: GameView::new(screen_wrapper), view_model: ViewModel::new(RogueSimulator::seed_from_u64(seed)) }
    }
    
    fn restore_game_from_file(filepath: String) -> RogueSimulator {
        todo!();
    }

    pub fn new_from_file(
        screen_wrapper: GameScreen,
        filepath: String,
    ) -> Game {
        Game { game_view: GameView::new(screen_wrapper), view_model: ViewModel::new(Self::restore_game_from_file(filepath)) }
    }

    pub fn play(&mut self) -> Result<()> {
        let stdin = stdin();
        self.game_view.draw(&self.view_model.view_state)?;
        for key in stdin.keys() {
            self.view_model.process_key(key?);
            self.game_view.draw(&self.view_model.view_state)?;
            if self.view_model.prev_command == Command::Exit(ExitCommand::Quit) {
                break;
            }
        }
        self.game_view.clean_up()
    }
    
}