extern crate term_size;

use std::env;

use rust_rogue::command::command;
use rust_rogue::init::{init_player, init_scroll_names, init_potion_colors, init_ring_stones};
use rust_rogue::rogue::{NUMCOLS, NUMLINES, s_names, r_stones};

fn main() {
    handle_args(&env::args().collect());
    check_terminal_size();
    init_rogue();
    play_rogue();
}

fn handle_args(args: &Vec<String>) {
    dbg!(&args);
    if args.len() == 3 {
        match args.get(1) {
            Some(arg) => {
                if arg == "-r" {
                    let filepath = args.get(2).expect("No file path provided");
                    restore_game_from_file(filepath);
                }
            },
            None => (),
        }
    }
}

fn restore_game_from_file(filepath: &str) {
    dbg!(filepath);
    //if (!restore(argv[1], envp))	/* Note: restore will never return */
    //    my_exit(1);
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

fn init_rogue() {
    init_player();
    unsafe {
        s_names = Some(init_scroll_names());
    }
    init_potion_colors();
    unsafe {
        r_stones = Some(init_ring_stones());
    }
    //init_materials();			/* Set up materials of wands */
    //new_level();			/* Draw current level */
    //Start up daemons and fuses
    //start_daemon(runners, 0, AFTER);
    //start_daemon(doctor, 0, AFTER);
    //fuse(swander, 0, WANDERTIME, AFTER);
    //start_daemon(stomach, 0, AFTER);
    //oldpos = hero;
    //oldrp = roomin(&hero);
}

fn play_rogue() {
    while command() {};
}