use std::ops::Mul;

use abstract_game_engine::core::simulator::Simulator;

use crate::core::{direction::Direction, rogue_action::RogueAction, rogue_simulator::RogueSimulator, rogue_state::RogueState};

use super::game_view_state::{GameViewState, OverlayViewState};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
	Continue,
    Multi(MultiCommand),
    Exit,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MultiCommand {
    Help,
    Inventory,
    Quit,
    ConfirmQuit,
    Win,
    Loss,
    Save,
}

pub fn process_command(
    view_state: &mut GameViewState,
    simulator: &mut RogueSimulator,
	state: &mut RogueState,
    prev_command: &Command,
    char: char,
) -> Command {
    match prev_command{
        Command::Multi(multi_command) => {
            match multi_command {
                MultiCommand::ConfirmQuit => {
                    match char {
                        COMMAND_CHAR_YES => {
                            view_state.overlay_view_state = Some(OverlayViewState::Quit { score: state.calculate_score() });
                            Command::Multi(MultiCommand::Quit)
                        },
                        _ => {
                            view_state.main_view_state.message = "".to_string();
                            Command::Continue
                        },
                    }
                },
                MultiCommand::Quit => {
                    Command::Exit
                },
                MultiCommand::Help | MultiCommand::Inventory => {
                    match char {
                        _ => {
                            hide_overlay(view_state)
                        },
                    }
                },
                MultiCommand::Win => todo!(),
                MultiCommand::Loss => todo!(),
                MultiCommand::Save => todo!(),
            }
        },
        _ => handle_new_command(view_state, simulator, state, char),
    }
} 

fn handle_new_command(
    view_state: &mut GameViewState,
    simulator: &mut RogueSimulator,
	state: &mut RogueState,
    char: char,
) -> Command {
    match char {
        COMMAND_CHAR_HELP => show_help(view_state),
        COMMAND_CHAR_MOVE_LEFT => rogue_action(RogueAction::Move(Direction::Left), simulator, state),
        COMMAND_CHAR_MOVE_DOWN => rogue_action(RogueAction::Move(Direction::Down), simulator, state),
        COMMAND_CHAR_MOVE_UP => rogue_action(RogueAction::Move(Direction::Up), simulator, state),
        COMMAND_CHAR_MOVE_RIGHT => rogue_action(RogueAction::Move(Direction::Right), simulator, state),
        COMMAND_CHAR_MOVE_UP_LEFT => rogue_action(RogueAction::Move(Direction::UpLeft), simulator, state),
        COMMAND_CHAR_MOVE_UP_RIGHT => rogue_action(RogueAction::Move(Direction::UpRight), simulator, state),
        COMMAND_CHAR_MOVE_DOWN_LEFT => rogue_action(RogueAction::Move(Direction::DownLeft), simulator, state),
        COMMAND_CHAR_MOVE_DOWN_RIGHT => rogue_action(RogueAction::Move(Direction::DownRight), simulator, state),
        COMMAND_CHAR_QUIT => quit(view_state),
        _ => illegal_command(view_state, char),
    }
}


fn rogue_action(
    action: RogueAction,
    simulator: &mut RogueSimulator,
	state: &mut RogueState,
) -> Command {
    let actions = vec![Some(action)];
    *state = simulator.state_transition(state, &actions);
    Command::Continue
}

fn show_help(view_state: &mut GameViewState) -> Command {
    view_state.overlay_view_state = Some(OverlayViewState::Help);
    Command::Multi(MultiCommand::Help)
}

fn hide_overlay(view_state: &mut GameViewState) -> Command {
    view_state.overlay_view_state = None;
    Command::Continue
}

fn quit(view_state: &mut GameViewState) -> Command {
    view_state.main_view_state.message = "quit? (y/n)".to_string();
    Command::Multi(MultiCommand::ConfirmQuit)
}

/// No action taken for illegal commands
fn illegal_command(view_state: &mut GameViewState, character: char) -> Command {
    view_state.main_view_state.message = format!("illegal command '{}'", character);
	Command::Continue
}

const COMMAND_CHAR_HELP: char = '?';
const COMMAND_CHAR_IDENTIFY: char = '/';
const COMMAND_CHAR_MOVE_LEFT: char = 'h';
const COMMAND_CHAR_MOVE_DOWN: char = 'j';
const COMMAND_CHAR_MOVE_UP: char = 'k';
const COMMAND_CHAR_MOVE_RIGHT: char = 'l';
const COMMAND_CHAR_MOVE_UP_LEFT: char = 'y';
const COMMAND_CHAR_MOVE_UP_RIGHT: char = 'u';
const COMMAND_CHAR_MOVE_DOWN_LEFT: char = 'b';
const COMMAND_CHAR_MOVE_DOWN_RIGHT: char = 'n';
const COMMAND_CHAR_FIGHT: char = 'f';
const COMMAND_CHAR_THROW: char = 't';
// const COMMAND_CHAR_MOVE: char = 'm';
const COMMAND_CHAR_ZAP: char = 'z';
const COMMAND_CHAR_IDENTIFY_TRAP: char = '^';
const COMMAND_CHAR_SEARCH: char = 's';
const COMMAND_CHAR_GO_DOWN: char = '>';
const COMMAND_CHAR_GO_UP: char = '<';
const COMMAND_CHAR_REST: char = '.';
const COMMAND_CHAR_PICK_UP: char = ',';
const COMMAND_CHAR_INVENTORY: char = 'i';
const COMMAND_CHAR_QUAFF: char = 'q';
const COMMAND_CHAR_READ: char = 'r';
const COMMAND_CHAR_EAT: char = 'e';
const COMMAND_CHAR_WIELD: char = 'w';
const COMMAND_CHAR_WEAR: char = 'W';
const COMMAND_CHAR_TAKE_OFF_ARMOR: char = 'T';
const COMMAND_CHAR_PUT_ON_RING: char = 'P';
const COMMAND_CHAR_REMOVE_RING: char = 'R';
const COMMAND_CHAR_DROP: char = 'd';
const COMMAND_CHAR_CALL: char = 'c';
const COMMAND_CHAR_REPEAT: char = 'a';
const COMMAND_CHAR_PRINT_WEAPON: char = ')';
const COMMAND_CHAR_PRINT_ARMOR: char = ']';
const COMMAND_CHAR_PRINT_RINGS: char = '=';
const COMMAND_CHAR_RECALL: char = 'D';
const COMMAND_CHAR_SAVE: char = 'S';
const COMMAND_CHAR_QUIT: char = 'Q';
// const COMMAND_CHAR_CANCEL: char = '\u{1b}'; ESC not needed, any key other than correct key will cancel
const COMMAND_CHAR_YES: char = 'y';

pub static HELP_ITEMS: [(char, &str); 31] = [
    (COMMAND_CHAR_IDENTIFY, "identify object"),
    (COMMAND_CHAR_MOVE_LEFT, "left"),
    (COMMAND_CHAR_MOVE_DOWN, "down"),
    (COMMAND_CHAR_MOVE_UP, "up"),
    (COMMAND_CHAR_MOVE_RIGHT, "right"),
    (COMMAND_CHAR_MOVE_UP_LEFT, "up & left"),
    (COMMAND_CHAR_MOVE_UP_RIGHT, "up & right"),
    (COMMAND_CHAR_MOVE_DOWN_LEFT, "down & left"),
    (COMMAND_CHAR_MOVE_DOWN_RIGHT, "down & right"),
    // "<SHIFT><dir>: run that way",
    // (COMMAND_CHAR_FIGHT, "fight to the death"),
    (COMMAND_CHAR_THROW, "throw something"),
    // COMMAND_CHAR_MOVE, instead move never picks up, must always be separate action
    (COMMAND_CHAR_ZAP, "zap a wand in a direction"),
    (COMMAND_CHAR_IDENTIFY_TRAP, "identify trap type"),
    (COMMAND_CHAR_SEARCH, "search for trap/secret door"),
    (COMMAND_CHAR_GO_DOWN, "go down a staircase"),
    (COMMAND_CHAR_GO_UP, "go up a staircase"),
    (COMMAND_CHAR_REST, "rest for a turn"),
    (COMMAND_CHAR_PICK_UP, "pick something up"),
    (COMMAND_CHAR_INVENTORY, "inventory"),
    (COMMAND_CHAR_QUAFF, "quaff potion"),
    (COMMAND_CHAR_READ, "read scroll"),
    (COMMAND_CHAR_EAT, "eat food"),
    (COMMAND_CHAR_WIELD, "wield a weapon"),
    (COMMAND_CHAR_WEAR, "wear armor"),
    (COMMAND_CHAR_TAKE_OFF_ARMOR, "take armor off"),
    (COMMAND_CHAR_PUT_ON_RING, "put on ring"),
    (COMMAND_CHAR_REMOVE_RING, "remove ring"),
    (COMMAND_CHAR_DROP, "drop object"),
    // (COMMAND_CHAR_CALL, "call object"),
    // (COMMAND_CHAR_REPEAT, "repeat last command"),
    (COMMAND_CHAR_PRINT_WEAPON, "print current weapon"),
    (COMMAND_CHAR_PRINT_ARMOR, "print current armor"),
    (COMMAND_CHAR_PRINT_RINGS, "print current rings"),
    // (COMMAND_CHAR_RECALL, "recall what's been discovered"),
    // (COMMAND_CHAR_SAVE, "save game"),
    (COMMAND_CHAR_QUIT, "quit"),
];

const MSG_ASK_DIRECTION: &str = "which direction?";
pub fn get_direction(
    view_state: &mut GameViewState,
    direction_char: char,
) -> Option<Direction> {
    view_state.main_view_state.message = MSG_ASK_DIRECTION.to_string();
    match direction_char {
        COMMAND_CHAR_MOVE_LEFT => Some(Direction::Left),
        COMMAND_CHAR_MOVE_DOWN => Some(Direction::Down),
        COMMAND_CHAR_MOVE_UP => Some(Direction::Up),
        COMMAND_CHAR_MOVE_RIGHT => Some(Direction::Right),
        COMMAND_CHAR_MOVE_UP_LEFT => Some(Direction::UpLeft),
        COMMAND_CHAR_MOVE_UP_RIGHT => Some(Direction::UpRight),
        COMMAND_CHAR_MOVE_DOWN_LEFT => Some(Direction::DownLeft),
        COMMAND_CHAR_MOVE_DOWN_RIGHT => Some(Direction::DownRight),
        _ => None,
    }
}
