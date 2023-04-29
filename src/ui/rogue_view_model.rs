use std::io::Error;

use crate::core::rogue_action::RogueAction;
use crate::rogue_state::RogueState;

pub struct RogueViewModel {
    rogue_state: RogueState<'static>,
    view_command_handler: Box<dyn ViewCommandHandler>,
    previous_action: Option<RogueAction>,
}

pub trait ViewCommandHandler {
    fn update_view(&mut self, command: ViewCommand) -> Result<(), Error>;
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
	// Unknown(char),
}

impl RogueViewModel {
    pub fn new(rogue_state: RogueState, view_command_handler: Box<dyn ViewCommandHandler>) -> RogueViewModel {
        RogueViewModel {
            rogue_state,
            view_command_handler,
            previous_action: None,
        }
    }

    pub fn handle_command(&mut self, command: ViewModelCommand) -> Result<(), Error> {
        match command {
            ViewModelCommand::Help => self.handle_help(),
            ViewModelCommand::Quit => self.handle_quit(),
            ViewModelCommand::RepeatAction => self.handle_repeat_action(),
            ViewModelCommand::SaveGame => self.handle_save_game(),
        }
    }

    fn handle_help(&mut self) -> Result<(), Error> {
        self.view_command_handler.update_view(ViewCommand::ShowHelp("Help".to_string()))
    }

    fn handle_quit(&mut self) -> Result<(), Error> {
        self.view_command_handler.update_view(ViewCommand::Quit)
    }
    
    fn handle_repeat_action(&mut self) -> Result<(), Error> {
        if let Some(action) = self.previous_action {
            self.rogue_state.take_action(action);
        }
        Ok(())
    }

    fn handle_save_game(&mut self) -> Result<(), Error> {
        todo!();
    }
}