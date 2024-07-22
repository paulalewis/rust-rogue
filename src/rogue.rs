use crate::core::{coord::Coord, creature::Creature, object::Object, room::Room, stats::{Attack, DmgStats, Stats}};

// This file contains global values for the game

//bool after;				/* True if we want after daemons */
pub static mut after: bool = false;
//bool again;				/* Repeating the last command */
// pub static again: bool = false;
//bool has_hit = FALSE;			/* Has a "hit" message pending in msg */
pub static has_hit: bool = false;
//bool inv_describe = TRUE;		/* Say which way items are being used */
pub static inv_describe: bool = true;
//bool move_on = FALSE;			/* Next move shouldn't pick up items */
// pub static move_on: bool = false;
//bool msg_esc = FALSE;			/* Check for ESC from msg's --More-- */
// pub static msg_esc: bool = false;
//bool running = FALSE;			/* True if player is running */
// pub static running: bool = false;
//bool to_death = FALSE;			/* Fighting is to the death! */
// pub static to_death: bool = false;

//char dir_ch;				/* Direction from last get_dir() call */
pub static dir_ch: char = '\0';
//char prbuf[2*MAXSTR];			/* buffer for sprintfs */
//char runch;				/* Direction player is running */
//char take;				/* Thing she is taking */
//int  orig_dsusp;			/* Original dsusp char */
//char l_last_comm = '\0';		/* Last last_comm */
//char l_last_dir = '\0';			/* Last last_dir */
//char last_comm = '\0';			/* Last command typed */
//char last_dir = '\0';			/* Last direction given */

//int n_objs;				/* # items listed in inventory() call */
pub static n_objs: usize = 0;
//int mpos = 0;				/* Where cursor is on top line */
// pub static mpos: usize = 0;

//int count = 0;				/* Number of times to repeat command */
// pub static mut repeat_command_count: usize = 0;
//int no_command = 0;			/* Number of turns asleep */
pub static no_command: usize = 0;
//int no_move = 0;			/* Number of turns held in place */
pub static no_move: usize = 0;
//int vf_hit = 0;				/* Number of time flytrap has hit */
pub static vf_hit: usize = 0;

// coord delta;				/* Change indicated to get_dir() */
pub static delta: Coord = Coord { y: 0, x: 0 };
// coord stairs;				/* Location of staircase */
pub static stairs: Coord = Coord { y: 0, x: 0 };

//THING *l_last_pick = NULL;		/* Last last_pick */
pub static l_last_pick: Option<Object> = None;
//THING *last_pick = NULL;		/* Last object picked in get_item() */
pub static last_pick: Option<Object> = None;

//#define CCHAR(x) ( (char) (x & A_CHARTEXT) )

//struct room	*roomin(coord *cp);

/*extern struct delayed_action {
    int d_type;
    void (*d_func)();
    int d_arg;
    int d_time;
} d_list[MAXDAEMONS];*/
pub struct DelayedAction {
    pub d_type: i32,
    pub func: fn(),
    pub arg: i32,
    pub time: i32,
}

//pub static d_list: [DelayedAction; MAXDAEMONS] = [DelayedAction { d_type: 0, d_func: fn(), d_arg: 0, d_time: 0 }; MAXDAEMONS];

/*typedef struct {
    char	*st_name;
    int		st_value;
} STONE;*/
pub struct Stone<'a> {
    pub name: &'a str,
    pub value: isize,
}