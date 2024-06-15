use abstract_game_engine::core::simulator::Simulator;

use crate::{constants::PRESS_SPACE_TO_CONTINUE, core::{direction::Direction, rogue_action::RogueAction, rogue_simulator::RogueSimulator, rogue_state::RogueState, screen::Screen}, ui::input_handler::wait_for_character};

use super::input_handler::read_character;

#[derive(Debug, PartialEq)]
pub enum CommandStatus {
	Continue,
    StateChange(RogueState),
	Win,
	Loss,
    Save,
	Quit,
    Error,
}

pub fn process_command(
	screen: &mut dyn Screen,
    simulator: &mut RogueSimulator,
	state: &RogueState,
) -> CommandStatus {
	let mut prev_char: Option<char> = None;
	match read_character() {
		Ok(command_char) => {
			match command_char {
                COMMAND_CHAR_HELP => help(screen),
                COMMAND_CHAR_MOVE_LEFT => rogue_action(RogueAction::Move(Direction::Left), screen, simulator, state),
                COMMAND_CHAR_MOVE_DOWN => rogue_action(RogueAction::Move(Direction::Down), screen, simulator, state),
                COMMAND_CHAR_MOVE_UP => rogue_action(RogueAction::Move(Direction::Up), screen, simulator, state),
                COMMAND_CHAR_MOVE_RIGHT => rogue_action(RogueAction::Move(Direction::Right), screen, simulator, state),
                COMMAND_CHAR_MOVE_UP_LEFT => rogue_action(RogueAction::Move(Direction::UpLeft), screen, simulator, state),
                COMMAND_CHAR_MOVE_UP_RIGHT => rogue_action(RogueAction::Move(Direction::UpRight), screen, simulator, state),
                COMMAND_CHAR_MOVE_DOWN_LEFT => rogue_action(RogueAction::Move(Direction::DownLeft), screen, simulator, state),
                COMMAND_CHAR_MOVE_DOWN_RIGHT => rogue_action(RogueAction::Move(Direction::DownRight), screen, simulator, state),
				COMMAND_CHAR_QUIT => quit(screen),
				_ => illegal_command(screen, command_char),
			}
		} 
		Err(_) => CommandStatus::Error,
	}
}

fn rogue_action(
    action: RogueAction,
	screen: &mut dyn Screen,
    simulator: &mut RogueSimulator,
	state: &RogueState,
) -> CommandStatus {
    let actions = vec![Some(action)];
    let new_state = simulator.state_transition(state, &actions);
    CommandStatus::StateChange(new_state)
}

fn help(screen: &mut dyn Screen) -> CommandStatus {
	// unsafe { after = false; }
	// for help_item in &HELP_ITEMS {
	// 	println!("{}", help_item);
	// }
	println!("{}", PRESS_SPACE_TO_CONTINUE);
	wait_for_character(' ').unwrap();
	//refresh();
	CommandStatus::Continue
}

fn quit(screen: &mut dyn Screen) -> CommandStatus {
	// unsafe { after = false; }
    // int oy, ox;
    // getyx(curscr, oy, ox);
    screen.show_message("really quit?");
    if read_character().unwrap() == COMMAND_CHAR_YES {
		// clear();
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
    }
}

fn illegal_command(screen: &mut dyn Screen, character: char) -> CommandStatus {
	// unsafe { repeat_command_count = 0; }
	screen.show_message(&format!("illegal command '{}'", character));
	CommandStatus::Continue
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
	screen: &mut dyn Screen,
) -> Option<Direction> {
	screen.show_message(MSG_ASK_DIRECTION);
	match read_character() {
		Ok(ch) => {
			match ch {
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
		Err(_) => None,
	}
}
