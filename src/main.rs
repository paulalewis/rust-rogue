extern crate term_size;

use std::env;

use rust_rogue::command::command;
use rust_rogue::rogue::*;

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
/*
    initscr();				/* Start up cursor package */
    init_player();			/* Set up initial player stats */
    // init_names();			/* Set up names of scrolls */
    init_colors();			/* Set up colors of potions */
    init_stones();			/* Set up stone settings of rings */
    init_materials();			/* Set up materials of wands */
    setup();

    /*
     * Set up windows
     */
    hw = newwin(LINES, COLS, 0, 0);
    idlok(stdscr, TRUE);
    idlok(hw, TRUE);
    new_level();			/* Draw current level */
    */
    // Start up daemons and fuses
    //start_daemon(runners, 0, AFTER);
    //start_daemon(doctor, 0, AFTER);
    //fuse(swander, 0, WANDERTIME, AFTER);
    //start_daemon(stomach, 0, AFTER);
    // oldpos = hero;
    // oldrp = roomin(&hero);
}

//playit
fn play_rogue() {
    while command() {};
}

/*
// Have player make certain, then exit.
void quit(int sig) {
    int oy, ox;

    /*
     * Reset the signal in case we got here via an interrupt
     */
    if (!q_comm)
	mpos = 0;
    getyx(curscr, oy, ox);
    msg("really quit?");
    if (readchar() == 'y')
    {
	signal(SIGINT, leave);
	clear();
	mvprintw(LINES - 2, 0, "You quit with %d gold pieces", purse);
	move(LINES - 1, 0);
	refresh();
	score(purse, 1, 0);
	exit(0);
    }
    else
    {
	move(0, 0);
	clrtoeol();
	status();
	move(oy, ox);
	refresh();
	mpos = 0;
	count = 0;
	to_death = FALSE;
    }
}
*/