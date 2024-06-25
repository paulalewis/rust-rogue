use std::io::Result;
use std::io::Write;

use super::game_view_state::GameViewState;
use super::game_view_state::MainViewState;
use super::game_view_state::OverlayViewState;
use super::game_screen::GameScreen;

/// Display game to user
pub struct GameView {
    screen: GameScreen,
}

impl GameView {
    pub fn new(screen: GameScreen) -> GameView {
        GameView { screen }
    }

    pub fn draw(&mut self, view_state: &GameViewState) -> Result<()> {
        write!(self.screen.stdout, "{}", termion::clear::All)?;
        match &view_state.overlay_view_state {
            Some(overlay_view_state) => {
                self.draw_overlay_view(&overlay_view_state)?;
            },
            None => {
                self.draw_main_view(&view_state.main_view_state)?;
            },
        };
        self.screen.stdout.flush()
    }

    pub fn clean_up(&mut self) -> Result<()> {
        write!(self.screen.stdout, "{}", termion::cursor::Show)?;
        self.screen.stdout.flush()
    }

    fn draw_main_view(&mut self, main_view_state: &MainViewState) -> Result<()> {
        self.draw_message(&main_view_state.message)
    }

    fn draw_message(&mut self, message: &String) -> Result<()> {
        write!(self.screen.stdout, "{}", termion::cursor::Goto(1, 1))?;
        // write!(self.stdout, "{}", termion::clear::CurrentLine).unwrap();
        write!(self.screen.stdout, "{}", message)
    }

    fn draw_overlay_view(&mut self, overlay_view_state: &OverlayViewState) -> Result<()> {
        match overlay_view_state {
            OverlayViewState::Help => {
                self.draw_help()
            },
            OverlayViewState::Inventory => {
                self.draw_inventory()
            },
            OverlayViewState::Win => {
                self.draw_win()
            },
            OverlayViewState::Loss => {
                self.draw_loss()
            },
            OverlayViewState::Quit => {
                self.draw_quit()
            },
        }
    }

    fn draw_help(&mut self) -> Result<()> {
        write!(self.screen.stdout, "{}", termion::cursor::Goto(1, 1))
    }

    fn draw_inventory(&mut self) -> Result<()> {
        write!(self.screen.stdout, "{}", termion::cursor::Goto(1, 1))
    }

    fn draw_win(&mut self) -> Result<()> {
        write!(self.screen.stdout, "{}", termion::cursor::Goto(1, 1))
    }

    fn draw_loss(&mut self) -> Result<()> {
        write!(self.screen.stdout, "{}", termion::cursor::Goto(1, 1))
    }

    fn draw_quit(&mut self) -> Result<()> {
        write!(self.screen.stdout, "{}", termion::clear::All)
    }
}