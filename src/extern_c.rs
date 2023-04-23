//#include <curses.h>
use crate::extern_h::*;
use crate::init::init_names;
use crate::rogue::*;

//bool after;				/* True if we want after daemons */
static after: bool = false;
//bool again;				/* Repeating the last command */
static again: bool = false;
//bool seenstairs;			/* Have seen the stairs (for lsd) */
static seenstairs: bool = false;
//bool amulet = FALSE;			/* He found the amulet */
static amulet: bool = false;
//bool door_stop = FALSE;			/* Stop running when we pass a door */
static door_stop: bool = false;
//bool fight_flush = FALSE;		/* True if toilet input */
static fight_flush: bool = false;
//bool firstmove = FALSE;			/* First move after setting door_stop */
static firstmove: bool = false;
//bool got_ltc = FALSE;			/* We have gotten the local tty chars */
static got_ltc: bool = false;
//bool has_hit = FALSE;			/* Has a "hit" message pending in msg */
static has_hit: bool = false;
//bool inv_describe = TRUE;		/* Say which way items are being used */
static inv_describe: bool = true;
//bool kamikaze = FALSE;			/* to_death really to DEATH */
static kamikaze: bool = false;
//bool lower_msg = FALSE;			/* Messages should start w/lower case */
static lower_msg: bool = false;
//bool move_on = FALSE;			/* Next move shouldn't pick up items */
static move_on: bool = false;
//bool msg_esc = FALSE;			/* Check for ESC from msg's --More-- */
static msg_esc: bool = false;
//bool passgo = FALSE;			/* Follow passages */
static passgo: bool = false;
//bool playing = TRUE;			/* True until he quits */
static playing: bool = true;
//bool q_comm = FALSE;			/* Are we executing a 'Q' command? */
static q_comm: bool = false;
//bool running = FALSE;			/* True if player is running */
static running: bool = false;
//bool save_msg = TRUE;			/* Remember last msg */
static save_msg: bool = true;
//bool stat_msg = FALSE;			/* Should status() print as a msg() */
static stat_msg: bool = false;
//bool to_death = FALSE;			/* Fighting is to the death! */
static to_death: bool = false;
//bool tombstone = TRUE;			/* Print out tombstone at end */
static tombstone: bool = true;
//bool pack_used[26] = {			/* Is the character used in the pack? */
//    FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE,
//    FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE,
//    FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE
//};
static pack_used: [bool; 26] = [false; 26];

//char dir_ch;				/* Direction from last get_dir() call */
static dir_ch: char = '\0';
//char file_name[MAXSTR];			/* Save file name */
//char huh[MAXSTR];			/* The last message printed */
//char *p_colors[MAXPOTIONS];		/* Colors of the potions */
pub static mut p_colors: [&str; MAXPOTIONS] = [""; MAXPOTIONS];
//char prbuf[2*MAXSTR];			/* buffer for sprintfs */
//char *r_stones[MAXRINGS];		/* Stone settings of the rings */
//char runch;				/* Direction player is running */
lazy_static! {
    //char *s_names[MAXSCROLLS];		/* Names of the scrolls */
    pub static ref s_names: Vec<String> = init_names();
}
//char take;				/* Thing she is taking */
//char whoami[MAXSTR];			/* Name of player */
pub static mut whoami: Option<String> = None;
//char *ws_made[MAXSTICKS];		/* What sticks are made of */
//char *ws_type[MAXSTICKS];		/* Is it a wand or a staff */
//int  orig_dsusp;			/* Original dsusp char */
//char fruit[MAXSTR]
pub const FRUIT: &str = "slime-mold";
//char home[MAXSTR] = { '\0' };		/* User's home directory */
//char *inv_t_name[] = {
//	"Overwrite",
//	"Slow",
//	"Clear"
//};
//char l_last_comm = '\0';		/* Last last_comm */
//char l_last_dir = '\0';			/* Last last_dir */
//char last_comm = '\0';			/* Last command typed */
//char last_dir = '\0';			/* Last direction given */
//char *tr_name[] = {			/* Names of the traps */
//	"a trapdoor",
//	"an arrow trap",
//	"a sleeping gas trap",
//	"a beartrap",
//	"a teleport trap",
//	"a poison dart trap",
//	"a rust trap",
//  "a mysterious trap"
//};

//int n_objs;				/* # items listed in inventory() call */
static n_objs: usize = 0;
//int ntraps;				/* Number of traps on this level */
static ntraps: usize = 0;
//int hungry_state = 0;			/* How hungry is he */
static hungry_state: usize = 0;
//int inpack = 0;				/* Number of things in pack */
static inpack: usize = 0;
//int inv_type = 0;			/* Type of inventory to use */
static inv_type: usize = 0;
//int level = 1;				/* What level she is on */
static level: usize = 1;
//int max_hit;				/* Max damage done to her in to_death */
static max_hit: usize = 0;
//int max_level;				/* Deepest player has gone */
static max_level: usize = 0;
//int mpos = 0;				/* Where cursor is on top line */
static mpos: usize = 0;
//int no_food = 0;			/* Number of levels without food */
static no_food: usize = 0;
//int a_class[MAXARMORS] = {		/* Armor class for each armor type */
//	8,	/* LEATHER */
//	7,	/* RING_MAIL */
//	7,	/* STUDDED_LEATHER */
//	6,	/* SCALE_MAIL */
//	5,	/* CHAIN_MAIL */
//	4,	/* SPLINT_MAIL */
//	4,	/* BANDED_MAIL */
//	3,	/* PLATE_MAIL */
//};
static a_class: [usize; MAXARMORS] = [8, 7, 7, 6, 5, 4, 4, 3];

//int count = 0;				/* Number of times to repeat command */
static count: usize = 0;
//FILE *scoreboard = NULL;	/* File descriptor for score file */
//int food_left;				/* Amount of food in hero's stomach */
static food_left: usize = 0;
//int lastscore = -1;			/* Score before this turn */
static lastscore: isize = -1;
//int no_command = 0;			/* Number of turns asleep */
static no_command: usize = 0;
//int no_move = 0;			/* Number of turns held in place */
static no_move: usize = 0;
//int purse = 0;				/* How much gold he has */
static purse: usize = 0;
//int quiet = 0;				/* Number of quiet turns */
static quiet: usize = 0;
//int vf_hit = 0;				/* Number of time flytrap has hit */
static vf_hit: usize = 0;

//int dnum;				/* Dungeon number */
static dnum: usize = 0;
//int seed;				/* Random number seed */
static seed: usize = 0;
//int e_levels[]
static e_levels: [usize; 21] = [
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

//coord delta;				/* Change indicated to get_dir() */
static delta: Coord = Coord { y: 0, x: 0 };
//coord oldpos;				/* Position before last look() call */
static oldpos: Coord = Coord { y: 0, x: 0 };
//coord stairs;				/* Location of staircase */
static stairs: Coord = Coord { y: 0, x: 0 };

//PLACE places[MAXLINES*MAXCOLS];		/* level map */
//static places: [Option<Place>; MAXLINES * MAXCOLS] = [None; MAXLINES * MAXCOLS];
static places: Vec<Place> = Vec::new();

//THING *cur_armor;			/* What he is wearing */
static cur_armor: Option<Thing> = None;
//THING *cur_ring[2];			/* Which rings are being worn */
static cur_ring: [Option<Thing>; 2] = [None, None];
//THING *cur_weapon;			/* Which weapon he is weilding */
static cur_weapon: Option<Thing> = None;
//THING *l_last_pick = NULL;		/* Last last_pick */
static l_last_pick: Option<Thing> = None;
//THING *last_pick = NULL;		/* Last object picked in get_item() */
static last_pick: Option<Thing> = None;
//THING *lvl_obj = NULL;			/* List of objects on this level */
static lvl_obj: Option<Thing> = None;
//THING *mlist = NULL;			/* List of monsters on the level */
static mlist: Option<Thing> = None;
//THING player;				/* His stats */
static player: Option<Thing> = None;

//WINDOW *hw = NULL;			/* used as a scratch window */

//#define INIT_STATS { 16, 0, 1, 10, 12, "1x4", 12 }
//struct stats max_stats = INIT_STATS;	/* The maximum for the player */
const MAX_INITIAL_STR: usize = 16;
static max_str: usize = MAX_INITIAL_STR;

//struct room *oldrp;			/* Roomin(&oldpos) */
static oldrp: Option<Room> = None;

//struct room rooms[MAXROOMS];		/* One for each room -- A level */
//static rooms: [Room; MAXROOMS] = [Room::new(); MAXROOMS];
static rooms: Vec<Room> = Vec::new();

//struct room passages[MAXPASS] =		/* One for each passage */
//{
//    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
//    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
//    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
//    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
//    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
//    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
//    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
//    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
//    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
//    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
//    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
//    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} }
//};
//static passages: [Room; MAXPASS] = [Room::new(); MAXPASS];
static passages: Vec<Room> = Vec::new();

lazy_static! {
    static ref monsters: [Monster; 26] = [
        Monster { name: String::from("aquator"), carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 20, lvl: 5, arm: 2, hpt: 1, dmg: String::from("0x0/0x0"), max_hp: 0 } },
        Monster { name: String::from("bat"), carry: 0, flags: ISFLY, stats: Stats { str: 10, exp: 1, lvl: 1, arm: 3, hpt: 1, dmg: String::from("1x2"), max_hp: 0 } },
        Monster { name: String::from("centaur"), carry: 15, flags: 0, stats: Stats { str: 10, exp: 17, lvl: 4, arm: 4, hpt: 1, dmg: String::from("1x2/1x5/1x5"), max_hp: 0 } },
        Monster { name: String::from("dragon"), carry: 100, flags: ISMEAN, stats: Stats { str: 10, exp: 5000, lvl: 10, arm: -1, hpt: 0, dmg: String::from("1x8/1x8/3x10"), max_hp: 0 } },
        Monster { name: String::from("emu"), carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 2, lvl: 1, arm: 7, hpt: 1, dmg: String::from("1x2"), max_hp: 0 } },
	    /* NOTE: the damage is %%% so that xstr won't merge this */
	    /* string with others, since it is written on in the program */
        Monster { name: String::from("venus flytrap"), carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 80, lvl: 8, arm: 3, hpt: 1, dmg: String::from("%%%x0"), max_hp: 0 } },
        Monster { name: String::from("griffin"), carry: 20, flags: ISMEAN | ISFLY | ISREGEN, stats: Stats { str: 10, exp: 2000, lvl: 13, arm: 2, hpt: 1, dmg: String::from("4x3/3x5"), max_hp: 0 } },
        Monster { name: String::from("hobgablin"), carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 3, lvl: 1, arm: 5, hpt: 1, dmg: String::from("1x8"), max_hp: 0 } },
        Monster { name: String::from("ice monster"), carry: 0, flags: 0, stats: Stats { str: 10, exp: 5, lvl: 1, arm: 9, hpt: 1, dmg: String::from("0x0"), max_hp: 0 } },
        Monster { name: String::from("jabberwock"), carry: 70, flags: 0, stats: Stats { str: 10, exp: 3000, lvl: 15, arm: 6, hpt: 1, dmg: String::from("2x12/2x4"), max_hp: 0 } },
        Monster { name: String::from("kestrel"), carry: 0, flags: ISMEAN | ISFLY, stats: Stats { str: 10, exp: 1, lvl: 1, arm: 7, hpt: 1, dmg: String::from("1x4"), max_hp: 0 } },
        Monster { name: String::from("leprechaun"), carry: 0, flags: 0, stats: Stats { str: 10, exp: 10, lvl: 3, arm: 8, hpt: 1, dmg: String::from("1x1"), max_hp: 0 } },
        Monster { name: String::from("medusa"), carry: 40, flags: ISMEAN, stats: Stats { str: 10, exp: 200, lvl: 8, arm: 2, hpt: 1, dmg: String::from("3x4/3x4/2x5"), max_hp: 0 } },
        Monster { name: String::from("nymph"), carry: 100, flags: 0, stats: Stats { str: 10, exp: 37, lvl: 3, arm: 9, hpt: 1, dmg: String::from("0x0"), max_hp: 0 } },
        Monster { name: String::from("orc"), carry: 15, flags: ISGREED, stats: Stats { str: 10, exp: 5, lvl: 1, arm: 6, hpt: 1, dmg: String::from("1x8"), max_hp: 0 } },
        Monster { name: String::from("phantom"), carry: 0, flags: ISINVIS, stats: Stats { str: 10, exp: 120, lvl: 8, arm: 3, hpt: 1, dmg: String::from("4x4"), max_hp: 0 } },
        Monster { name: String::from("quagga"), carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 15, lvl: 3, arm: 3, hpt: 1, dmg: String::from("1x5/1x5"), max_hp: 0 } },
        Monster { name: String::from("rattlesnake"), carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 9, lvl: 2, arm: 3, hpt: 1, dmg: String::from("1x6"), max_hp: 0 } },
        Monster { name: String::from("snake"), carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 2, lvl: 1, arm: 5, hpt: 1, dmg: String::from("1x3"), max_hp: 0 } },
        Monster { name: String::from("troll"), carry: 50, flags: ISREGEN | ISMEAN, stats: Stats { str: 10, exp: 120, lvl: 6, arm: 4, hpt: 1, dmg: String::from("1x8/1x8/2x6"), max_hp: 0 } },
        Monster { name: String::from("black unicorn"), carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 190, lvl: 7, arm: -2, hpt: 1, dmg: String::from("1x9/1x9/2x9"), max_hp: 0 } },
        Monster { name: String::from("vampire"), carry: 20, flags: ISREGEN | ISMEAN, stats: Stats { str: 10, exp: 350, lvl: 8, arm: 1, hpt: 1, dmg: String::from("1x10"), max_hp: 0 } },
        Monster { name: String::from("wraith"), carry: 0, flags: 0, stats: Stats { str: 10, exp: 55, lvl: 5, arm: 4, hpt: 1, dmg: String::from("1x6"), max_hp: 0 } },
        Monster { name: String::from("xeroc"), carry: 30, flags: 0, stats: Stats { str: 10, exp: 100, lvl: 7, arm: 7, hpt: 1, dmg: String::from("4x4"), max_hp: 0 } },
        Monster { name: String::from("yeti"), carry: 30, flags: 0, stats: Stats { str: 10, exp: 50, lvl: 4, arm: 6, hpt: 1, dmg: String::from("1x6/1x6"), max_hp: 0 } },
        Monster { name: String::from("zombie"), carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 6, lvl: 2, arm: 8, hpt: 1, dmg: String::from("1x8"), max_hp: 0 } },
    ];
}

//struct obj_info things[NUMTHINGS]
static things: [ObjInfo; NUMTHINGS] = [
    ObjInfo { name: String::new(), prob: 26, worth: 0, guess: String::new(), know: false },/* potion */
    ObjInfo { name: String::new(), prob: 62, worth: 0, guess: String::new(), know: false },/* scroll */
    ObjInfo { name: String::new(), prob: 78, worth: 0, guess: String::new(), know: false },/* food */
    ObjInfo { name: String::new(), prob: 85, worth: 0, guess: String::new(), know: false },/* weapon */
    ObjInfo { name: String::new(), prob: 92, worth: 0, guess: String::new(), know: false },/* armor */
    ObjInfo { name: String::new(), prob: 96, worth: 0, guess: String::new(), know: false },/* ring */
    ObjInfo { name: String::new(), prob: 100, worth: 0, guess: String::new(), know: false },/* stick */
];

//struct obj_info arm_info[MAXARMORS]
lazy_static! {
    static ref arm_info: [ObjInfo; MAXARMORS] = [
        ObjInfo { name: String::from("leather armor"), prob: 20, worth: 20, guess: String::new(), know: false },
        ObjInfo { name: String::from("ring mail"), prob: 35, worth: 25, guess: String::new(), know: false },
        ObjInfo { name: String::from("studded leather armor"), prob: 50, worth: 20, guess: String::new(), know: false },
        ObjInfo { name: String::from("scale mail"), prob: 63, worth: 30, guess: String::new(), know: false },
        ObjInfo { name: String::from("chain mail"), prob: 75, worth: 75, guess: String::new(), know: false },
        ObjInfo { name: String::from("splint mail"), prob: 85, worth: 80, guess: String::new(), know: false },
        ObjInfo { name: String::from("banded mail"), prob: 95, worth: 90, guess: String::new(), know: false },
        ObjInfo { name: String::from("plate mail"), prob: 100, worth: 150, guess: String::new(), know: false },
    ];
}

//struct obj_info pot_info[MAXPOTIONS]
lazy_static! {
    static ref pot_info: [ObjInfo; MAXPOTIONS] = [
        ObjInfo { name: String::from("confusion"), prob: 7, worth: 5, guess: String::new(), know: false },
        ObjInfo { name: String::from("hallucination"), prob: 15, worth: 5, guess: String::new(), know: false },
        ObjInfo { name: String::from("poison"), prob: 23, worth: 5, guess: String::new(), know: false },
        ObjInfo { name: String::from("gain strength"), prob: 36, worth: 150, guess: String::new(), know: false },
        ObjInfo { name: String::from("see invisible"), prob: 39, worth: 100, guess: String::new(), know: false },
        ObjInfo { name: String::from("healing"), prob: 52, worth: 130, guess: String::new(), know: false },
        ObjInfo { name: String::from("monster detection"), prob: 58, worth: 130, guess: String::new(), know: false },
        ObjInfo { name: String::from("magic detection"), prob: 64, worth: 105, guess: String::new(), know: false },
        ObjInfo { name: String::from("raise level"), prob: 66, worth: 250, guess: String::new(), know: false },
        ObjInfo { name: String::from("extra healing"), prob: 71, worth: 200, guess: String::new(), know: false },
        ObjInfo { name: String::from("haste self"), prob: 76, worth: 190, guess: String::new(), know: false },
        ObjInfo { name: String::from("restore strength"), prob: 89, worth: 130, guess: String::new(), know: false },
        ObjInfo { name: String::from("blindness"), prob: 94, worth: 5, guess: String::new(), know: false },
        ObjInfo { name: String::from("levitation"), prob: 100, worth: 75, guess: String::new(), know: false },
    ];
}

//struct obj_info ring_info[MAXRINGS]
lazy_static! {
    static ref ring_info: [ObjInfo; MAXRINGS] = [
        ObjInfo { name: String::from("protection"), prob: 9, worth: 400, guess: String::new(), know: false },
        ObjInfo { name: String::from("add strength"), prob: 18, worth: 400, guess: String::new(), know: false },
        ObjInfo { name: String::from("sustain strength"), prob: 23, worth: 280, guess: String::new(), know: false },
        ObjInfo { name: String::from("searching"), prob: 33, worth: 420, guess: String::new(), know: false },
        ObjInfo { name: String::from("see invisible"), prob: 43, worth: 310, guess: String::new(), know: false },
        ObjInfo { name: String::from("adornment"), prob: 44, worth: 10, guess: String::new(), know: false },
        ObjInfo { name: String::from("aggravate monster"), prob: 54, worth: 10, guess: String::new(), know: false },
        ObjInfo { name: String::from("dexterity"), prob: 62, worth: 440, guess: String::new(), know: false },
        ObjInfo { name: String::from("increase damage"), prob: 70, worth: 400, guess: String::new(), know: false },
        ObjInfo { name: String::from("regeneration"), prob: 74, worth: 460, guess: String::new(), know: false },
        ObjInfo { name: String::from("slow digestion"), prob: 83, worth: 240, guess: String::new(), know: false },
        ObjInfo { name: String::from("teleportation"), prob: 88, worth: 30, guess: String::new(), know: false },
        ObjInfo { name: String::from("stealth"), prob: 95, worth: 470, guess: String::new(), know: false },
        ObjInfo { name: String::from("maintain armor"), prob: 100, worth: 380, guess: String::new(), know: false },
    ];
}

//struct obj_info scr_info[MAXSCROLLS]
lazy_static! {
    static ref scr_info: [ObjInfo; MAXSCROLLS] = [
        ObjInfo { name: String::from("monster confusion"), prob: 7, worth: 140, guess: String::new(), know: false },
        ObjInfo { name: String::from("magic mapping"), prob: 11, worth: 150, guess: String::new(), know: false },
        ObjInfo { name: String::from("hold monster"), prob: 13, worth: 180, guess: String::new(), know: false },
        ObjInfo { name: String::from("sleep"), prob: 16, worth: 5, guess: String::new(), know: false },
        ObjInfo { name: String::from("enchant armor"), prob: 23, worth: 160, guess: String::new(), know: false },
        ObjInfo { name: String::from("identify potion"), prob: 33, worth: 80, guess: String::new(), know: false },
        ObjInfo { name: String::from("identify scroll"), prob: 43, worth: 80, guess: String::new(), know: false },
        ObjInfo { name: String::from("identify weapon"), prob: 49, worth: 80, guess: String::new(), know: false },
        ObjInfo { name: String::from("identify armor"), prob: 56, worth: 100, guess: String::new(), know: false },
        ObjInfo { name: String::from("identify ring, wand or staff"), prob: 66, worth: 115, guess: String::new(), know: false },
        ObjInfo { name: String::from("scare monster"), prob: 69, worth: 200, guess: String::new(), know: false },
        ObjInfo { name: String::from("food detection"), prob: 71, worth: 60, guess: String::new(), know: false },
        ObjInfo { name: String::from("teleportation"), prob: 76, worth: 165, guess: String::new(), know: false },
        ObjInfo { name: String::from("enchant weapon"), prob: 84, worth: 150, guess: String::new(), know: false },
        ObjInfo { name: String::from("create monster"), prob: 88, worth: 75, guess: String::new(), know: false },
        ObjInfo { name: String::from("remove curse"), prob: 95, worth: 105, guess: String::new(), know: false },
        ObjInfo { name: String::from("aggravate monsters"), prob: 98, worth: 20, guess: String::new(), know: false },
        ObjInfo { name: String::from("protect armor"), prob: 100, worth: 250, guess: String::new(), know: false },
    ];
}

//struct obj_info weap_info[MAXWEAPONS + 1]
lazy_static! {
    static ref weap_info: [ObjInfo; MAXWEAPONS + 1] = [
        ObjInfo { name: String::from("mace"), prob: 11, worth: 8, guess: String::new(), know: false },
        ObjInfo { name: String::from("long sword"), prob: 22, worth: 15, guess: String::new(), know: false },
        ObjInfo { name: String::from("short bow"), prob: 34, worth: 15, guess: String::new(), know: false },
        ObjInfo { name: String::from("arrow"), prob: 46, worth: 1, guess: String::new(), know: false },
        ObjInfo { name: String::from("dagger"), prob: 54, worth: 3, guess: String::new(), know: false },
        ObjInfo { name: String::from("two handed sword"), prob: 64, worth: 75, guess: String::new(), know: false },
        ObjInfo { name: String::from("dart"), prob: 76, worth: 2, guess: String::new(), know: false },
        ObjInfo { name: String::from("shuriken"), prob: 88, worth: 5, guess: String::new(), know: false },
        ObjInfo { name: String::from("spear"), prob: 100, worth: 5, guess: String::new(), know: false },
        //{ NULL, 0 }, /* DO NOT REMOVE: fake entry for dragon's breath
        ObjInfo { name: String::new(), prob: 0, worth: 0, guess: String::new(), know: false },
    ];
}

//struct obj_info ws_info[MAXSTICKS]
lazy_static! {
    static ref ws_info: [ObjInfo; MAXSTICKS] = [
        ObjInfo { name: String::from("light"), prob: 12, worth: 250, guess: String::new(), know: false },
        ObjInfo { name: String::from("invisibility"), prob: 18, worth: 5, guess: String::new(), know: false },
        ObjInfo { name: String::from("lightning"), prob: 21, worth: 330, guess: String::new(), know: false },
        ObjInfo { name: String::from("fire"), prob: 24, worth: 330, guess: String::new(), know: false },
        ObjInfo { name: String::from("cold"), prob: 27, worth: 330, guess: String::new(), know: false },
        ObjInfo { name: String::from("polymorph"), prob: 42, worth: 310, guess: String::new(), know: false },
        ObjInfo { name: String::from("magic missile"), prob: 52, worth: 170, guess: String::new(), know: false },
        ObjInfo { name: String::from("haste monster"), prob: 62, worth: 5, guess: String::new(), know: false },
        ObjInfo { name: String::from("slow monster"), prob: 73, worth: 350, guess: String::new(), know: false },
        ObjInfo { name: String::from("drain life"), prob: 82, worth: 300, guess: String::new(), know: false },
        ObjInfo { name: String::from("nothing"), prob: 83, worth: 5, guess: String::new(), know: false },
        ObjInfo { name: String::from("teleport away"), prob: 89, worth: 340, guess: String::new(), know: false },
        ObjInfo { name: String::from("teleport to"), prob: 95, worth: 50, guess: String::new(), know: false },
        ObjInfo { name: String::from("cancellation"), prob: 100, worth: 280, guess: String::new(), know: false },
    ];
}

//struct h_list helpstr[]
lazy_static! {
    static ref helpstr: [HList; 65] = [
        HList { ch: '?', desc: String::from("prints help"), print: true },
        HList { ch: '/', desc: String::from("identify object"), print: true },
        HList { ch: 'h', desc: String::from("left"), print: true },
        HList { ch: 'j', desc: String::from("down"), print: true },
        HList { ch: 'k', desc: String::from("up"), print: true },
        HList { ch: 'l', desc: String::from("right"), print: true },
        HList { ch: 'y', desc: String::from("up & left"), print: true },
        HList { ch: 'u', desc: String::from("up & right"), print: true },
        HList { ch: 'b', desc: String::from("down & left"), print: true },
        HList { ch: 'n', desc: String::from("down & right"), print: true },
        HList { ch: 'H', desc: String::from("run left"), print: false },
        HList { ch: 'J', desc: String::from("run down"), print: false },
        HList { ch: 'K', desc: String::from("run up"), print: false },
        HList { ch: 'L', desc: String::from("run right"), print: false },
        HList { ch: 'Y', desc: String::from("run up & left"), print: false },
        HList { ch: 'U', desc: String::from("run up & right"), print: false },
        HList { ch: 'B', desc: String::from("run down & left"), print: false },
        HList { ch: 'N', desc: String::from("run down & right"), print: false },
        HList { ch: CTRL('H'), desc: String::from("run left until adjacent"), print: false },
        HList { ch: CTRL('J'), desc: String::from("run down until adjacent"), print: false },
        HList { ch: CTRL('K'), desc: String::from("run up until adjacent"), print: false },
        HList { ch: CTRL('L'), desc: String::from("run right until adjacent"), print: false },
        HList { ch: CTRL('Y'), desc: String::from("run up & left until adjacent"), print: false },
        HList { ch: CTRL('U'), desc: String::from("run up & right until adjacent"), print: false },
        HList { ch: CTRL('B'), desc: String::from("run down & left until adjacent"), print: false },
        HList { ch: CTRL('N'), desc: String::from("run down & right until adjacent"), print: false },
        HList { ch: '\0', desc: String::from("<SHIFT><dir>: run that way"), print: true },
        HList { ch: '\0', desc: String::from("<CTRL><dir>: run till adjacent"), print: true },
        HList { ch: 'f', desc: String::from("<dir> fight till death or near death"), print: true },
        HList { ch: 't', desc: String::from("<dir> throw something"), print: true },
        HList { ch: 'm', desc: String::from("<dir> move onto without picking up"), print: true },
        HList { ch: 'z', desc: String::from("<dir> zap a wand in a direction"), print: true },
        HList { ch: '^', desc: String::from("<dir> identify trap type"), print: true },
        HList { ch: 's', desc: String::from("search for trap/secret door"), print: true },
        HList { ch: '>', desc: String::from("go down a staircase"), print: true },
        HList { ch: '<', desc: String::from("go up a staircase"), print: true },
        HList { ch: '.', desc: String::from("rest for a turn"), print: true },
        HList { ch: ',', desc: String::from("pick something up"), print: true },
        HList { ch: 'i', desc: String::from("inventory"), print: true },
        HList { ch: 'I', desc: String::from("inventory single item"), print: true },
        HList { ch: 'q', desc: String::from("quaff potion"), print: true },
        HList { ch: 'r', desc: String::from("read scroll"), print: true },
        HList { ch: 'e', desc: String::from("eat food"), print: true },
        HList { ch: 'w', desc: String::from("wield a weapon"), print: true },
        HList { ch: 'W', desc: String::from("wear armor"), print: true },
        HList { ch: 'T', desc: String::from("take armor off"), print: true },
        HList { ch: 'P', desc: String::from("put on ring"), print: true },
        HList { ch: 'R', desc: String::from("remove ring"), print: true },
        HList { ch: 'd', desc: String::from("drop object"), print: true },
        HList { ch: 'c', desc: String::from("call object"), print: true },
        HList { ch: 'a', desc: String::from("repeat last command"), print: true },
        HList { ch: ')', desc: String::from("print current weapon"), print: true },
        HList { ch: ']', desc: String::from("print current armor"), print: true },
        HList { ch: '=', desc: String::from("print current rings"), print: true },
        HList { ch: '@', desc: String::from("print current stats"), print: true },
        HList { ch: 'D', desc: String::from("recall what's been discovered"), print: true },
        HList { ch: 'o', desc: String::from("examine/set options"), print: true },
        HList { ch: CTRL('R'), desc: String::from("redraw screen"), print: true },
        HList { ch: CTRL('P'), desc: String::from("repeat last message"), print: true },
        HList { ch: ESCAPE, desc: String::from("cancel command"), print: true },
        HList { ch: 'S', desc: String::from("save game"), print: true },
        HList { ch: 'Q', desc: String::from("quit"), print: true },
        HList { ch: 'F', desc: String::from("<dir> fight till either of you dies"), print: true },
        HList { ch: 'v', desc: String::from("print version number"), print: true },
        HList { ch: '\0', desc: String::new(), print: false },
    ];
}