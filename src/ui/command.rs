use abstract_game_engine::core::simulator::Simulator;

use crate::core::{direction::Direction, rogue_action::RogueAction, rogue_simulator::RogueSimulator, rogue_state::RogueState};

use super::game_view_state::{GameViewState, OverlayViewState};

#[derive(Debug, PartialEq)]
pub enum Command {
	Continue,
    Multi(MultiCommand),
    Exit(ExitCommand),
}

#[derive(Debug, PartialEq)]
pub enum MultiCommand {
    Help,
    Inventory,
    Quit,
}

#[derive(Debug, PartialEq)]
pub enum ExitCommand {
	Win,
	Loss,
    Save,
	Quit,
    Error,
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
                MultiCommand::Quit => {
                    match char {
                        COMMAND_CHAR_YES => {
                            Command::Exit(ExitCommand::Quit)
                        },
                        _ => {
                            view_state.main_view_state.message = "".to_string();
                            Command::Continue
                        },
                    }
                },
                MultiCommand::Help | MultiCommand::Inventory => {
                    match char {
                        _ => {
                            hide_overlay(view_state)
                        },
                    }
                },
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
	// println!("{}", PRESS_ANY_KEY_TO_CONTINUE);
    // wait_for_any_character().unwrap();
    view_state.overlay_view_state = Some(OverlayViewState::Help);
    Command::Multi(MultiCommand::Help)
}

fn hide_overlay(view_state: &mut GameViewState) -> Command {
    view_state.overlay_view_state = None;
    Command::Continue
}

fn quit(view_state: &mut GameViewState) -> Command {
    view_state.main_view_state.message = "quit?".to_string();
	// unsafe { after = false; }
    // int oy, ox;
    // getyx(curscr, oy, ox);
    // screen.show_message("really quit?");
    /*if read_character().unwrap() == COMMAND_CHAR_YES {
        screen.clear();
		// mvprintw(LINES - 2, 0, "You quit with %d gold pieces", purse);
		// move(LINES - 1, 0);
		// refresh();
		// score(purse, 1, 0);
		CommandStatus::Quit
    } else {
		// move(0, 0);
		// clrtoeol();
		// status();
		// move(oy, ox);
		// refresh();
		// mpos = 0;
		// count = 0;
		CommandStatus::Continue
    }*/
    Command::Multi(MultiCommand::Quit)
}

/// Not action taken for illegal commands
fn illegal_command(view_state: &mut GameViewState, character: char) -> Command {
    // dbg!("illegal comand {}", character);
	// unsafe { repeat_command_count = 0; }
	// screen.show_message(&format!("illegal command '{}'", character));
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


pub static HELP_CHARS: [char; 37] = [
    COMMAND_CHAR_HELP,
    COMMAND_CHAR_IDENTIFY,
    COMMAND_CHAR_MOVE_LEFT,
    COMMAND_CHAR_MOVE_DOWN,
    COMMAND_CHAR_MOVE_UP,
    COMMAND_CHAR_MOVE_RIGHT,
    COMMAND_CHAR_MOVE_UP_LEFT,
    COMMAND_CHAR_MOVE_UP_RIGHT,
    COMMAND_CHAR_MOVE_DOWN_LEFT,
    COMMAND_CHAR_MOVE_DOWN_RIGHT,
    // "<SHIFT><dir>: run that way",
    COMMAND_CHAR_FIGHT,
    COMMAND_CHAR_THROW,
    // COMMAND_CHAR_MOVE, instead move never picks up, must always be separate action
    COMMAND_CHAR_ZAP,
    COMMAND_CHAR_IDENTIFY_TRAP,
    COMMAND_CHAR_SEARCH,
    COMMAND_CHAR_GO_DOWN,
    COMMAND_CHAR_GO_UP,
    COMMAND_CHAR_REST,
    COMMAND_CHAR_PICK_UP,
    COMMAND_CHAR_INVENTORY,
    COMMAND_CHAR_QUAFF,
    COMMAND_CHAR_READ,
    COMMAND_CHAR_EAT,
    COMMAND_CHAR_WIELD,
    COMMAND_CHAR_WEAR,
    COMMAND_CHAR_TAKE_OFF_ARMOR,
    COMMAND_CHAR_PUT_ON_RING,
    COMMAND_CHAR_REMOVE_RING,
    COMMAND_CHAR_DROP,
    COMMAND_CHAR_CALL,
    COMMAND_CHAR_REPEAT,
    COMMAND_CHAR_PRINT_WEAPON,
    COMMAND_CHAR_PRINT_ARMOR,
    COMMAND_CHAR_PRINT_RINGS,
    COMMAND_CHAR_RECALL,
    COMMAND_CHAR_SAVE,
    COMMAND_CHAR_QUIT,
];

pub static HELP_DESCS: [&str; 37] = [
    "prints help",
    "identify object",
    "left",
    "down",
    "up",
    "right",
    "up & left",
    "up & right",
    "down & left",
    "down & right",
    // "<SHIFT><dir>: run that way",
    "fight till death or near death",
    "throw something",
    // "move onto without picking up", move never picks up, must always be separate action
    "zap a wand in a direction",
    "identify trap type",
    "search for trap/secret door",
    "go down a staircase",
    "go up a staircase",
    "rest for a turn",
    "pick something up",
    "inventory",
    "quaff potion",
    "read scroll",
    "eat food",
    "wield a weapon",
    "wear armor",
    "take armor off",
    "put on ring",
    "remove ring",
    "drop object",
    "call object",
    "repeat last command",
    "print current weapon",
    "print current armor",
    "print current rings",
    "recall what's been discovered",
    "save game",
    "quit",
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
