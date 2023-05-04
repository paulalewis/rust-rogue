use std::io::{Error, Read};

use crate::{core::{screen::{Screen, SCREEN_HEIGHT}, object::Object, stats::Stats}, rogue::HELP_ITEMS, constants::PRESS_SPACE_TO_CONTINUE};

use super::{input_handler::{read_character, wait_for_character}, console_screen::ConsoleScreen};

pub trait UserInterface {
    fn show_message(&mut self, msg: &str);
    fn hide_message(&mut self);

    fn show_help(&mut self);

    fn show_inventory(&mut self, items: &Vec<Object>);

    fn show_status(&mut self, level: usize, gold: usize, stats: Stats);

    fn show_map(&mut self);

    fn get_user_input(&mut self) -> Result<char, Error>;
    fn wait_for_user_input(&mut self);
}

pub struct ConsoleUserInterface {
    screen: ConsoleScreen,
    std_in: std::io::Stdin,
}

impl ConsoleUserInterface {
    pub fn new() -> ConsoleUserInterface {
        ConsoleUserInterface {
            screen: ConsoleScreen::new(),
            std_in: std::io::stdin(),
        }
    }

    fn show_press_space_to_continue(&mut self) {
        self.screen.move_cursor(SCREEN_HEIGHT - 1, 0);
        self.screen.write(PRESS_SPACE_TO_CONTINUE);
        self.wait_for_user_input();
    }
}

impl UserInterface for ConsoleUserInterface {
    fn show_message(&mut self, msg: &str) {
        self.screen.show_message(msg);
    }

    fn hide_message(&mut self) {
        self.screen.clear_message();
    }

    fn show_help(&mut self) {
        self.screen.clear();
	    for (i, help_item) in HELP_ITEMS.iter().enumerate() {
            self.screen.move_cursor(i + 1, 0);
            self.screen.write(help_item);
	    }
        self.show_press_space_to_continue();
    }

    fn show_inventory(&mut self, items: &Vec<Object>) {
        todo!();
        self.show_press_space_to_continue();
    }

    fn show_status(&mut self, level: usize, gold: usize, stats: Stats) {
        todo!();
    }

    fn show_map(&mut self) {
        todo!()
    }

    fn get_user_input(&mut self) -> Result<char, Error> {
        let mut buf = ['\n' as u8; 1];
        while buf[0] == '\n' as u8 || buf[0] == '\r' as u8 {
            self.std_in.read_exact(&mut buf)?;
        }
        return Ok(buf[0] as char);
    }

    fn wait_for_user_input(&mut self) {
        wait_for_character(' ').unwrap()
    }
}