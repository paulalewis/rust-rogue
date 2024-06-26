use std::io::{stdin, Result};

use termion::input::TermRead;

use crate::core::rogue_simulator::RogueSimulator;

use super::{game_screen::GameScreen, game_view::GameView, game_controller::GameController};

/// Create and play a new game.
pub struct Game {
    view: GameView,
    controller: GameController,
}

impl Game {
    pub fn new(
        screen_wrapper: GameScreen,
    ) -> Game {
        Game { view: GameView::new(screen_wrapper), controller: GameController::new(RogueSimulator::new()) }
    }

    pub fn new_from_seed(
        screen_wrapper: GameScreen,
        seed: u64,
    ) -> Game {
        Game { view: GameView::new(screen_wrapper), controller: GameController::new(RogueSimulator::seed_from_u64(seed)) }
    }
    
    fn restore_game_from_file(filepath: String) -> RogueSimulator {
        todo!();
    }

    pub fn new_from_file(
        screen_wrapper: GameScreen,
        filepath: String,
    ) -> Game {
        Game { view: GameView::new(screen_wrapper), controller: GameController::new(Self::restore_game_from_file(filepath)) }
    }

    pub fn play(&mut self) -> Result<()> {
        let stdin = stdin();
        let keys = &mut stdin.keys();
        while !self.controller.end_game() {
            self.view.draw(&self.controller.view_state)?;
            let key = keys.next().unwrap()?;
            self.controller.process_key(key);
        }
        Ok(())
    }
}