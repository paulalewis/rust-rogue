use std::{io::Error, rc::Rc};

use crate::{screen::{Screen, ConsoleScreen}, io::readchar, core::rogue_state::RogueState};

use super::rogue_view_model::{RogueViewModel, ViewCommand, ViewModelCommand};

pub struct RogueView<'a> {
    rogue_view_model: RogueViewModel<'a>,
    screen: Box<dyn Screen>,
    view_command_handler: Option<Box<dyn Fn(ViewCommand) -> Result<(), Error>>>,
    playing: bool,
}

impl<'a> RogueView<'a> {
    pub fn new(rogue_state: RogueState) -> RogueView {
        RogueView {
            rogue_view_model: RogueViewModel::new(rogue_state),
            screen: Box::new(ConsoleScreen::new()),
            view_command_handler: None,
            playing: true,
        }
    }

    pub fn init(&'a mut self) {
        self.rogue_view_model.add_callback(|command| {
            match command {
                ViewCommand::ShowMessage(msg) => {
                    println!("{}", msg);
                },
                ViewCommand::ShowHelp(msg) => {
                    println!("{}", msg);
                    self.screen.draw();
                },
                ViewCommand::Quit => self.playing = false,
            }
            Ok(())
        });
    }

    pub fn start(&mut self) {
        while self.playing {
            let input = self.next_character();
            match input {
                'a' => self.repeat(),
                '\u{001b}' => self.cancel(),
                'Q' => self.quit(),
                '?' => {
                    self.rogue_view_model.handle_command(ViewModelCommand::Help);
                },
                _ => self.illegal_command(input)
            }
            // self.screen.draw();
        }
    }

    fn next_character(&mut self) -> char {
        readchar().unwrap()
    }

    fn cancel(&mut self) {
        self.screen.clear_msg();
        self.screen.draw();
    }

    fn illegal_command(&mut self, command: char) {
        self.screen.msg(&format!("illegal command '{}'", command));
    }

    fn quit(&mut self) {
        self.screen.msg("really quit?(y/n)");
        self.screen.draw();
        let input = self.next_character();
        if input == 'y' {
            self.rogue_view_model.handle_command(ViewModelCommand::Quit);
        } else {
            self.screen.clear_msg();
            self.screen.draw();
        }
    }

    fn repeat(&mut self) {
        self.rogue_view_model.handle_command(ViewModelCommand::RepeatAction);
    }

    /*fn handle_view_command_2(&mut self, command: ViewCommand) -> Result<(), Error> {
        match command {
            ViewCommand::ShowMessage(msg) => {
                println!("{}", msg);
            },
            ViewCommand::ShowHelp(msg) => {
                println!("{}", msg);
                self.screen.draw();
            },
            ViewCommand::Quit => {
                println!("Quit");
            },
        }
        Ok(())
    }*/
}
