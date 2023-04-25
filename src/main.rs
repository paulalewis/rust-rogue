use std::env;
use std::process;

use rust_rogue::rogue::*;

fn main() {
    let args: Vec<String> = env::args().collect();
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
    
/*
    initscr();				/* Start up cursor package */
    init_probs();			/* Set up prob tables for objects */
    init_player();			/* Set up initial player stats */
    // init_names();			/* Set up names of scrolls */
    init_colors();			/* Set up colors of potions */
    init_stones();			/* Set up stone settings of rings */
    init_materials();			/* Set up materials of wands */
    setup();

    /*
     * The screen must be at least NUMLINES x NUMCOLS
     */
    if (LINES < NUMLINES || COLS < NUMCOLS)
    {
	printf("\nSorry, the screen must be at least %dx%d\n", NUMLINES, NUMCOLS);
	endwin();
	my_exit(1);
    }

    /*
     * Set up windows
     */
    hw = newwin(LINES, COLS, 0, 0);
    idlok(stdscr, TRUE);
    idlok(hw, TRUE);
    new_level();			/* Draw current level */
    /*
     * Start up daemons and fuses
     */
    start_daemon(runners, 0, AFTER);
    start_daemon(doctor, 0, AFTER);
    fuse(swander, 0, WANDERTIME, AFTER);
    start_daemon(stomach, 0, AFTER);
    */
    playit();
}

fn restore_game_from_file(filepath: &str) {
    dbg!(filepath);
    //if (!restore(argv[1], envp))	/* Note: restore will never return */
    //    my_exit(1);
}

fn playit() {
    // char *opts;

    // parse environment declaration of options
    // if ((opts = getenv("ROGUEOPTS")) != NULL)
	// parse_opts(opts);

    // oldpos = hero;
    // oldrp = roomin(&hero);
    // while (playing)
	// command(); /* Command execution */
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