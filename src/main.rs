use std::env;
use std::process;

use rust_rogue::rogue::*;
use rust_rogue::extern_c::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let home_path = env::home_dir().unwrap();

    let save_path = home_path.as_path().join(DEFAULT_SAVE_FILE_NAME);
/*
    char *env;
    int lowtime;

    //get home and options from environment
    strncpy(home, md_gethomedir(), MAXSTR);

    strcpy(file_name, home);
    strcat(file_name, "rogue.save");

    if ((env = getenv("ROGUEOPTS")) != NULL)
	parse_opts(env);
    lowtime = (int) time(NULL);
	dnum = lowtime + md_getpid();
    seed = dnum;

    open_score();
*/
    match args.get(1) {
        Some(arg) => {
            if arg == "-s" {
                show_scores();
            } else if arg == "-r" {
                dbg!("restore");
            } else if arg == "-n" {
                set_player_name(&args);
            }
        },
        None => {},
    }
    
/*
    if (argc == 2)
	if (!restore(argv[1], envp))	/* Note: restore will never return */
	    my_exit(1);

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
    process::exit(0);
}

fn show_scores() {
    dbg!("show_scores()");
    // score(0, -1, 0, true);
    process::exit(0);
}

fn set_player_name(args: &Vec<String>) {
    dbg!("set_player_name({})", args);
    unsafe {
        whoami = Some(if args.len() > 2 { args[2].to_owned() } else { String::from("Rustacean") });
    }
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
    process::exit(0);
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