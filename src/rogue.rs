use crate::{core::constants::{ISFLY, ISGREED, ISINVIS, ISMEAN, ISREGEN, MAXARMORS, MAXPOTIONS, MAXRINGS, MAXSCROLLS, MAXSTICKS, MAXWEAPONS, NUMTHINGS}, core::{coord::Coord, creature::Creature, object::Object, room::Room, stats::{Attack, DmgStats, Stats}}, utils::spread};

// This file contains global values for the game

//bool after;				/* True if we want after daemons */
pub static mut after: bool = false;
//bool again;				/* Repeating the last command */
pub static again: bool = false;
//bool has_hit = FALSE;			/* Has a "hit" message pending in msg */
pub static has_hit: bool = false;
//bool inv_describe = TRUE;		/* Say which way items are being used */
pub static inv_describe: bool = true;
//bool move_on = FALSE;			/* Next move shouldn't pick up items */
pub static move_on: bool = false;
//bool msg_esc = FALSE;			/* Check for ESC from msg's --More-- */
pub static msg_esc: bool = false;
//bool running = FALSE;			/* True if player is running */
pub static running: bool = false;
//bool to_death = FALSE;			/* Fighting is to the death! */
pub static to_death: bool = false;

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

//tr_name
pub static TRAP_NAMES: [&str; 8] = [
    "a trapdoor",
    "an arrow trap",
    "a sleeping gas trap",
    "a beartrap",
    "a teleport trap",
    "a poison dart trap",
    "a rust trap",
    "a mysterious trap"
];

//int n_objs;				/* # items listed in inventory() call */
pub static n_objs: usize = 0;
//int mpos = 0;				/* Where cursor is on top line */
pub static mpos: usize = 0;
//int a_class[MAXARMORS] = {
//	8,	/* LEATHER */
//	7,	/* RING_MAIL */
//	7,	/* STUDDED_LEATHER */
//	6,	/* SCALE_MAIL */
//	5,	/* CHAIN_MAIL */
//	4,	/* SPLINT_MAIL */
//	4,	/* BANDED_MAIL */
//	3,	/* PLATE_MAIL */
//};
pub static ARMOR_CLASS: [usize; MAXARMORS] = [8, 7, 7, 6, 5, 4, 4, 3];

//int count = 0;				/* Number of times to repeat command */
pub static mut repeat_command_count: usize = 0;
//int no_command = 0;			/* Number of turns asleep */
pub static no_command: usize = 0;
//int no_move = 0;			/* Number of turns held in place */
pub static no_move: usize = 0;
//int vf_hit = 0;				/* Number of time flytrap has hit */
pub static vf_hit: usize = 0;

// e_levels
pub static XP_LEVELS: [usize; 21] = [
    10,
    20,
    40,
    80,
    160,
    320,
    640,
    1300,
    2600,
    5200,
    13000,
    26000,
    50000,
    100000,
    200000,
    400000,
    800000,
    2000000,
    4000000,
    8000000,
    0,
];

// coord delta;				/* Change indicated to get_dir() */
pub static delta: Coord = Coord { y: 0, x: 0 };
// coord stairs;				/* Location of staircase */
pub static stairs: Coord = Coord { y: 0, x: 0 };

//THING *l_last_pick = NULL;		/* Last last_pick */
pub static l_last_pick: Option<Object> = None;
//THING *last_pick = NULL;		/* Last object picked in get_item() */
pub static last_pick: Option<Object> = None;

//#define INIT_STATS { 16, 0, 1, 10, 12, "1x4", 12 }
//struct stats max_stats = INIT_STATS;	/* The maximum for the player */
const MAX_INITIAL_STR: usize = 16;
pub static max_str: usize = MAX_INITIAL_STR;

//#define CCHAR(x) ( (char) (x & A_CHARTEXT) )

//#define BEARTIME	spread(3)
pub const BEARTIME: usize = 3;
//#define SLEEPTIME	spread(5)
pub const SLEEPTIME: usize = 5;
//#define HOLDTIME	spread(2)
pub const HOLDTIME: usize = 2;
//#define WANDERTIME	spread(70)
pub fn wander_time() -> usize { spread(70) }
//#define BEFORE		spread(1)
pub const BEFORE: usize = 1;
//#define AFTER		spread(2)
pub const AFTER: usize = 2;

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

pub const NSTONES: usize = 26;
//STONE stones[] = {
pub static stones: [Stone; NSTONES] = [
    Stone { name: "agate", value: 25 },
    Stone { name: "alexandrite", value: 40 },
    Stone { name: "amethyst", value: 50 },
    Stone { name: "carnelian", value: 40 },
    Stone { name: "diamond", value: 300 },
    Stone { name: "emerald", value: 300 },
    Stone { name: "germanium", value: 225 },
    Stone { name: "granite", value: 5 },
    Stone { name: "garnet", value: 50 },
    Stone { name: "jade", value: 150 },
    Stone { name: "kryptonite", value: 300 },
    Stone { name: "lapis lazuli", value: 50 },
    Stone { name: "moonstone", value: 50 },
    Stone { name: "obsidian", value: 15 },
    Stone { name: "onyx", value: 60 },
    Stone { name: "opal", value: 200 },
    Stone { name: "pearl", value: 220 },
    Stone { name: "peridot", value: 63 },
    Stone { name: "ruby", value: 350 },
    Stone { name: "sapphire", value: 285 },
    Stone { name: "stibotantalite", value: 200 },
    Stone { name: "tiger eye", value: 50 },
    Stone { name: "topaz", value: 60 },
    Stone { name: "turquoise", value: 70 },
    Stone { name: "taaffeite", value: 300 },
    Stone { name: "zircon", value: 80 },
];

/*
 * Contains defintions and functions for dealing with things like
 * potions and scrolls
 */
pub const NCOLORS: usize = 27;
/*char *rainbow[] = {};*/
pub static rainbow: [&str; NCOLORS] = [
    "amber",
    "aquamarine",
    "black",
    "blue",
    "brown",
    "clear",
    "crimson",
    "cyan",
    "ecru",
    "gold",
    "green",
    "grey",
    "magenta",
    "orange",
    "pink",
    "plaid",
    "purple",
    "red",
    "silver",
    "tan",
    "tangerine",
    "topaz",
    "turquoise",
    "vermilion",
    "violet",
    "white",
    "yellow",
];

pub const NWOOD: usize = 33;
//char *wood[] = {
pub static wood: [&str; NWOOD] = [
    "avocado wood",
    "balsa",
    "bamboo",
    "banyan",
    "birch",
    "cedar",
    "cherry",
    "cinnibar",
    "cypress",
    "dogwood",
    "driftwood",
    "ebony",
    "elm",
    "eucalyptus",
    "fall",
    "hemlock",
    "holly",
    "ironwood",
    "kukui wood",
    "mahogany",
    "manzanita",
    "maple",
    "oaken",
    "persimmon wood",
    "pecan",
    "pine",
    "poplar",
    "redwood",
    "rosewood",
    "spruce",
    "teak",
    "walnut",
    "zebrawood",
];

pub const NMETAL: usize = 22;
//char *metal[] = {
pub static metal: [&str; NMETAL] = [
    "aluminum",
    "beryllium",
    "bone",
    "brass",
    "bronze",
    "copper",
    "electrum",
    "gold",
    "iron",
    "lead",
    "magnesium",
    "mercury",
    "nickel",
    "pewter",
    "platinum",
    "steel",
    "silver",
    "silicon",
    "tin",
    "titanium",
    "tungsten",
    "zinc",
];
