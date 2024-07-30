use std::io::Result;
use std::io::Write;

use termion::cursor::Goto;

use crate::core::status::Status;
use crate::ui::command::HELP_ITEMS;
use crate::ui::constants::PRESS_ANY_KEY_TO_CONTINUE;
use crate::ui::game_screen::SCREEN_HEIGHT;
use crate::ui::utils::break_string;

use super::game_screen::SCREEN_WIDTH;
use super::game_view_state::GameViewState;
use super::game_view_state::MainViewState;
use super::game_view_state::OverlayViewState;
use super::game_screen::GameScreen;

/// Display game to terminal
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

    fn draw_main_view(&mut self, main_view_state: &MainViewState) -> Result<()> {
        self.draw_message(&main_view_state.message)?;
        self.draw_status(&main_view_state.status)
    }

    fn draw_message(&mut self, message: &String) -> Result<()> {
        let (line1, line2) = break_string(message, 80);
        write!(self.screen.stdout, "{}{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine, line1)?;
        write!(self.screen.stdout, "{}{}{}", termion::cursor::Goto(2, 1), termion::clear::CurrentLine, line2)
    }

    fn draw_status(&mut self, status: &Status) -> Result<()> {
        write!(self.screen.stdout, "{}", termion::cursor::Goto(1, SCREEN_HEIGHT as u16))?;
        write!(self.screen.stdout, "Level: {} Gold: {} Health: {}/{} Strength: {}/{} Armor: {} Experience: {}/{}", status.level, status.gold, status.health.0, status.health.1, status.strength.0, status.strength.1, status.armor, status.experience.0, status.experience.1)
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
            OverlayViewState::Quit { score } => self.draw_quit(*score),
        }
    }

    fn draw_help(&mut self) -> Result<()> {
        write!(self.screen.stdout, "{}", termion::clear::All)?;
        for (i, item) in HELP_ITEMS.iter().enumerate() {
            let character = item.0;
            let desc = item.1;
            write!(self.screen.stdout, "{}{} {}", Self::calc_help_location(i, HELP_ITEMS.len()), character, desc)?;
        }
        write!(self.screen.stdout, "{}{}", termion::cursor::Goto(1, SCREEN_HEIGHT as u16), PRESS_ANY_KEY_TO_CONTINUE)
    }

    fn calc_help_location(index: usize, n_items: usize) -> Goto {
        let items_height = n_items / 2 + n_items % 2;
        if index < items_height {
            Goto(1, (index + 1) as u16)
        } else {
            Goto((SCREEN_WIDTH / 2 + 1) as u16, ((index % items_height) + 1) as u16)
        }
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

    fn draw_quit(&mut self, score: usize) -> Result<()> {
        write!(self.screen.stdout, "{}", termion::clear::All)?;
        write!(self.screen.stdout, "{}You quit with {} gold pieces", termion::cursor::Goto(1, (SCREEN_HEIGHT - 2) as u16), score)?;
        write!(self.screen.stdout, "{}{}", termion::cursor::Goto(1, (SCREEN_HEIGHT - 1) as u16), PRESS_ANY_KEY_TO_CONTINUE)
    }
}