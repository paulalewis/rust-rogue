extern crate term_size;

use std::{env, fs};

use rust_rogue::{constants::{NUMCOLS, NUMLINES}, ui::game::Game};

fn main() {
    check_tty();
    check_terminal_size();
    let mut game = init_game();
    game.play();
}

enum InitGame {
    Init,
    FromSeed(u64),
    FromFile(String),
}

fn init_game() -> Game {
    match handle_args(&env::args().collect()) {
        InitGame::Init => Game::new(),
        InitGame::FromSeed(seed) => Game::new_from_seed(seed),
        InitGame::FromFile(filepath) => Game::new_from_file(filepath),
    }
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

fn check_tty() {
    if termion::is_tty(&fs::File::create("/dev/stdout").unwrap()) {
        dbg!("This is a TTY!");
    } else {
        panic!("This is not a TTY :(");
    }
}