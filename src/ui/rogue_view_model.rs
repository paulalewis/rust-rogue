use std::io::Error;
use std::rc::{Rc, Weak};

use crate::core::rogue_action::RogueAction;
use crate::core::rogue_state::RogueState;

pub struct RogueViewModel<'a> {
    rogue_state: RogueState<'a>,
    view_command_callback: Option<Box<dyn FnMut(ViewCommand) -> Result<(), Error> + 'a>>,
    previous_action: Option<RogueAction>,
}

pub enum ViewCommand {
    ShowMessage(String),
    ShowHelp(String),
    Quit,
}

pub enum ViewModelCommand {
	// Cancel,
	// Continue,
	Help,
	Quit,
    RepeatAction,
	SaveGame,
}

impl<'a> RogueViewModel<'a> {
    pub fn new(rogue_state: RogueState) -> RogueViewModel {
        RogueViewModel {
            rogue_state,
            view_command_callback: None, 
            previous_action: None,
        }
    }

    pub fn add_callback(&mut self, callback: impl FnMut(ViewCommand) -> Result<(), Error> + 'a) {
        self.view_command_callback = Some(Box::new(callback));
    }

    pub fn handle_command(&mut self, command: ViewModelCommand) {
        match command {
            ViewModelCommand::Help => self.handle_help(),
            ViewModelCommand::Quit => self.handle_quit(),
            ViewModelCommand::RepeatAction => self.handle_repeat_action(),
            ViewModelCommand::SaveGame => self.handle_save_game(),
        }
    }

    fn handle_help(&mut self) {
        (self.view_command_callback.as_mut().unwrap())(ViewCommand::ShowHelp("Help".to_string()));
    }

    fn handle_quit(&mut self) {
        (self.view_command_callback.as_mut().unwrap())(ViewCommand::Quit);
    }
    
    fn handle_repeat_action(&mut self) {
        if self.previous_action.is_some() {
            let action = self.previous_action.as_ref().unwrap();
            // self.rogue_state.player.take_action(action);
        }
    }

    fn handle_save_game(&mut self) {
        todo!();
    }
}