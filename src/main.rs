extern crate term_size;

use std::{env, fs, panic};

use rust_rogue::{core::constants::{NUMCOLS, NUMLINES}, ui::{game::Game, game_screen::GameScreen}};

fn main() {
    check_tty();
    check_terminal_size();
    let prev = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        println!("Custom panic hook called");
        // println!("{}", info);
        prev(info);
    }));
    init_game().play().unwrap();
}

enum InitGame {
    Init,
    FromSeed(u64),
    FromFile(String),
}

fn init_game() -> Game {
    let screen = GameScreen::new();
    match handle_args(&env::args().collect()) {
        InitGame::Init => Game::new(screen),
        InitGame::FromSeed(seed) => Game::new_from_seed(screen, seed),
        InitGame::FromFile(filepath) => Game::new_from_file(screen, filepath),
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
        dbg!(width, height);
        if width < NUMCOLS || height < NUMLINES {
            panic!("The screen must be at least {}x{}", NUMCOLS, NUMLINES);
        }
    } else {
        panic!("Unable to get term size");
    }
}

fn check_tty() {
    if !termion::is_tty(&fs::File::create("/dev/stdout").unwrap()) {
        panic!("Terminal is not a TTY");
    }
}