extern crate term_size;

use std::env;

use rust_rogue::command::command;
use rust_rogue::constants::{NUMCOLS, NUMLINES};
use rust_rogue::core::rogue_state::RogueState;
use rust_rogue::ui::console_screen;

enum InitGame {
    Init,
    FromSeed(u64),
    FromFile(String),
}

fn main() {
    check_terminal_size();
    let init_game = handle_args(&env::args().collect());
    let rogue_state = match init_game {
        InitGame::Init => init_rogue(0),
        InitGame::FromSeed(seed) => init_rogue(seed),
        InitGame::FromFile(filepath) => restore_game_from_file(&filepath),
    };
    play_rogue(rogue_state);
}

fn handle_args(args: &Vec<String>) -> InitGame {
    dbg!(&args);
    if args.len() == 3 {
        match args.get(1) {
            Some(arg) => {
                match arg.as_str() {
                    "-s" => {
                        let seed = args.get(2).expect("No seed provided").parse::<u64>().expect("Seed must be a number");
                        InitGame::FromSeed(seed)
                    },
                    "-r" => {
                        let filepath = args.get(2).expect("No file path provided");
                        InitGame::FromFile(filepath.to_string())
                    }
                    _ => InitGame::Init,
                }
            },
            None => InitGame::Init,
        }
    } else {
        InitGame::Init
    }
}

fn restore_game_from_file<'a, 'b>(filepath: &'a str) -> RogueState<'b> {
    dbg!(filepath);
    todo!("Restore not implemented");
}

fn check_terminal_size() {
    if let Some((width, height)) = term_size::dimensions() {
        dbg!(width, height, NUMCOLS, NUMLINES);
        if width < NUMCOLS || height < NUMLINES {
            panic!("Sorry, the screen must be at least {}x{}", NUMCOLS, NUMLINES);
        }
    } else {
        panic!("Unable to get term size");
    }
}

fn init_rogue<'a>(seed: u64) -> RogueState<'a> {
    let rogue_state = RogueState::new(seed);
    //new_level();			/* Draw current level */
    //Start up daemons and fuses
    //start_daemon(runners, 0, AFTER);
    //start_daemon(doctor, 0, AFTER);
    //fuse(swander, 0, WANDERTIME, AFTER);
    //start_daemon(stomach, 0, AFTER);
    rogue_state
}

fn play_rogue(rouge_state: RogueState) {
    let mut console_screen = console_screen::ConsoleScreen::new();
    while command(&mut console_screen, &rouge_state) {};
}