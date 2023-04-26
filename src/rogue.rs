pub const RELEASE: &str = "5.4.4";

// This file contains most all of the glabol constants and statics

//#define MAXSTR		1024	/* maximum length of strings */
pub const MAXSTR: usize = 1024;
//#define MAXLINES	32	/* maximum number of screen lines used */
pub const MAXLINES: usize = 32;
//#define MAXCOLS		80	/* maximum number of screen columns used */
pub const MAXCOLS: usize = 80;

//#define RN		(((seed = seed*11109+13849) >> 16) & 0xffff)

//bool after;				/* True if we want after daemons */
pub static mut after: bool = false;
//bool again;				/* Repeating the last command */
static again: bool = false;
//bool seenstairs;			/* Have seen the stairs (for lsd) */
static seenstairs: bool = false;
//bool amulet = FALSE;			/* He found the amulet */
static amulet: bool = false;
//bool door_stop = FALSE;			/* Stop running when we pass a door */
static door_stop: bool = false;
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
//bool running = FALSE;			/* True if player is running */
static running: bool = false;
//bool stat_msg = FALSE;			/* Should status() print as a msg() */
static stat_msg: bool = false;
//bool to_death = FALSE;			/* Fighting is to the death! */
static to_death: bool = false;
//bool pack_used[26] = {			/* Is the character used in the pack? */
//    FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE,
//    FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE,
//    FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE
//};
static pack_used: [bool; 26] = [false; 26];

//char dir_ch;				/* Direction from last get_dir() call */
static dir_ch: char = '\0';
//char *p_colors[MAXPOTIONS];		/* Colors of the potions */
pub static mut p_colors: [&str; MAXPOTIONS] = [""; MAXPOTIONS];
//char prbuf[2*MAXSTR];			/* buffer for sprintfs */
//char runch;				/* Direction player is running */
//char *r_stones[MAXRINGS];		/* Stone settings of the rings */
pub static mut r_stones: Option<Vec<String>> = None;
//char *s_names[MAXSCROLLS];		/* Names of the scrolls */
pub static mut s_names: Option<Vec<String>> = None;
//char take;				/* Thing she is taking */
//char whoami[MAXSTR]
pub const ADVENTURER_NAME: &str = "Rustacean";
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
pub static n_objs: usize = 0;
//int ntraps;				/* Number of traps on this level */
pub static ntraps: usize = 0;
//int hungry_state = 0;			/* How hungry is he */
pub static hungry_state: usize = 0;
//int inpack = 0;				/* Number of things in pack */
pub static inpack: usize = 0;
//int inv_type = 0;			/* Type of inventory to use */
pub static inv_type: usize = 0;
//int level = 1;				/* What level she is on */
pub static level: usize = 1;
//int max_hit;				/* Max damage done to her in to_death */
pub static max_hit: usize = 0;
//int max_level;				/* Deepest player has gone */
pub static max_level: usize = 0;
//int mpos = 0;				/* Where cursor is on top line */
pub static mpos: usize = 0;
//int no_food = 0;			/* Number of levels without food */
pub static no_food: usize = 0;
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
pub static a_class: [usize; MAXARMORS] = [8, 7, 7, 6, 5, 4, 4, 3];

//int count = 0;				/* Number of times to repeat command */
pub static mut repeat_command_count: usize = 0;
//FILE *scoreboard = NULL;	/* File descriptor for score file */
//int food_left;				/* Amount of food in hero's stomach */
pub static food_left: usize = 0;
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
pub static cur_armor: Option<Thing> = None;
//THING *cur_ring[2];			/* Which rings are being worn */
pub static cur_ring: [Option<Thing>; 2] = [None, None];
//THING *cur_weapon;			/* Which weapon he is weilding */
pub static cur_weapon: Option<Thing> = None;
//THING *l_last_pick = NULL;		/* Last last_pick */
pub static l_last_pick: Option<Thing> = None;
//THING *last_pick = NULL;		/* Last object picked in get_item() */
pub static last_pick: Option<Thing> = None;
//THING *lvl_obj = NULL;			/* List of objects on this level */
pub static lvl_obj: Option<Thing> = None;
//THING *mlist = NULL;			/* List of monsters on the level */
pub static mlist: Option<Thing> = None;
//THING player;				/* His stats */
pub static mut player: Option<Thing> = None;

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

pub const NUMBER_OF_MONSTERS: usize = 26;
lazy_static! {
    pub static ref monsters: [Monster; NUMBER_OF_MONSTERS] = [
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
    pub static ref ring_info: [ObjInfo; MAXRINGS] = [
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

pub static HELP_ITEMS: [&str; 42] = [
    "?            prints help",
    "/            identify object",
    "h            left",
    "j            down",
    "k            up",
    "l            right",
    "y            up & left",
    "u            up & right",
    "b            down & left",
    "n            down & right",
    "<shift><dir> run that way",
    "f<dir>       fight till death or near death",
    "t<dir>       throw something",
    "m<dir>       move onto without picking up",
    "z<dir>       zap a wand in a direction",
    "^<dir>       identify trap type",
    "s            search for trap/secret door",
    ">            go down a staircase",
    "<            go up a staircase",
    ".            rest for a turn",
    ",            pick something up",
    "i            inventory",
    "q            quaff potion",
    "r            read scroll",
    "e            eat food",
    "w            wield a weapon",
    "W            wear armor",
    "T            take armor off",
    "P            put on ring",
    "R            remove ring",
    "d            drop object",
    "c            call object",
    "a            repeat last command",
    ")            print current weapon",
    "]            print current armor",
    "=            print current rings",
    "D            recall what's been discovered",
    "S            save game",
    "Q            quit",
    "F<dir>       fight till either of you dies",
    "v            print version",
    "ESC          cancel command",
];

//#define CCHAR(x) ( (char) (x & A_CHARTEXT) )

//#define MAXROOMS
pub const MAXROOMS: usize = 9;
//#define MAXTHINGS
pub const MAXTHINGS: usize = 9;
//#define MAXOBJ
pub const MAXOBJ: usize = 9;
//#define MAXPACK
pub const MAXPACK: usize = 23;
//#define MAXTRAPS 10
pub const MAXTRAPS: usize = 10;
//#define AMULETLEVEL 26
pub const AMULETLEVEL: usize = 26;
//#define NUMTHINGS /* number of types of things */
pub const NUMTHINGS: usize = 7;
//#define MAXPASS /* upper limit on number of passages */
pub const MAXPASS: usize = 13;
//#define NUMLINES
pub const NUMLINES: usize = 24;
//#define NUMCOLS
pub const NUMCOLS: usize = 80;
//#define STATLINE
pub const STATLINE: usize = NUMLINES - 1;
//#define BORE_LEVEL
pub const BORE_LEVEL: usize = 50;

/*
 * return values for get functions
 */
//#define	NORM	0	/* normal exit */
const NORM: usize = 0;
//#define	QUIT	1	/* quit option setting */
const QUIT: usize = 1;
//#define	MINUS	2	/* back up one option */
const MINUS: usize = 2;

/*
 * things that appear on the screens
 */
//#define PASSAGE		'#'
pub const PASSAGE: char = '#';
//#define DOOR		'+'
pub const DOOR: char = '+';
//#define FLOOR		'.'
pub const FLOOR: char = '.';
//#define PLAYER		'@'
pub const PLAYER: char = '@';
//#define TRAP		'^'
pub const TRAP: char = '^';
//#define STAIRS		'%'
pub const STAIRS: char = '%';
//#define GOLD		'*'
pub const GOLD: char = '*';
//#define POTION		'!'
pub const POTION: char = '!';
//#define SCROLL		'?'
pub const SCROLL: char = '?';
//#define MAGIC		'$'
pub const MAGIC: char = '$';
//#define FOOD		':'
pub const FOOD: char = ':';
//#define WEAPON		')'
pub const WEAPON: char = ')';
//#define ARMOR		']'
pub const ARMOR: char = ']';
//#define AMULET		','
pub const AMULET: char = ',';
//#define RING		'='
pub const RING: char = '=';
//#define STICK		'/'
pub const STICK: char = '/';
//#define CALLABLE	-1
pub const CALLABLE: isize = -1;
//#define R_OR_S		-2
pub const R_OR_S: isize = -2;

/*
 * Various constants
 */
//#define BEARTIME	spread(3)
//#define SLEEPTIME	spread(5)
//#define HOLDTIME	spread(2)
//#define WANDERTIME	spread(70)
//#define BEFORE		spread(1)
//#define AFTER		spread(2)
//#define HEALTIME	30
pub const HEALTIME: usize = 30;
//#define HUHDURATION	20
pub const HUHDURATION: usize = 20;
//#define SEEDURATION	850
pub const SEEDURATION: usize = 850;
//#define HUNGERTIME	1300
pub const HUNGERTIME: usize = 1300;
//#define MORETIME	150
pub const MORETIME: usize = 150;
//#define STOMACHSIZE	2000
pub const STOMACHSIZE: usize = 2000;
//#define STARVETIME	850
pub const STARVETIME: usize = 850;
//#define ESCAPE		27
pub const ESCAPE: char = '\u{001b}';
//#define LEFT		0
pub const LEFT: usize = 0;
//#define RIGHT		1
pub const RIGHT: usize = 1;
//#define BOLT_LENGTH	6
pub const BOLT_LENGTH: usize = 6;
//#define LAMPDIST	3
pub const LAMPDIST: usize = 3;

/*
 * Save against things
 */
//#define VS_POISON	00
pub const VS_POISON: usize = 0;
//#define VS_PARALYZATION	00
pub const VS_PARALYZATION: usize = 0;
//#define VS_DEATH	00
pub const VS_DEATH: usize = 0;
//#define VS_BREATH	02
pub const VS_BREATH: usize = 2;
//#define VS_MAGIC	03
pub const VS_MAGIC: usize = 3;

/*
 * Various flag bits
 */
/* flags for rooms */
//#define ISDARK	0000001		/* room is dark */
pub const ISDARK: usize = 1;
//#define ISGONE	0000002		/* room is gone (a corridor) */
pub const ISGONE: usize = 2;
//#define ISMAZE	0000004		/* room is gone (a corridor) */
pub const ISMAZE: usize = 4;

/* flags for objects */
//#define ISCURSED 000001		/* object is cursed */
pub const ISCURSED: usize = 1;
//#define ISKNOW	0000002		/* player knows details about the object */
pub const ISKNOW: usize = 2;
//#define ISMISL	0000004		/* object is a missile type */
pub const ISMISL: usize = 4;
//#define ISMANY	0000010		/* object comes in groups */
pub const ISMANY: usize = 10;
/*	ISFOUND 0000020		...is used for both objects and creatures */
//#define	ISPROT	0000040		/* armor is permanently protected */
pub const ISPROT: usize = 40;

/* flags for creatures */
//#define CANHUH	0000001		/* creature can confuse */
pub const CANHUH: usize = 1;
//#define CANSEE	0000002		/* creature can see invisible creatures */
pub const CANSEE: usize = 2;
//#define ISBLIND	0000004		/* creature is blind */
pub const ISBLIND: usize = 4;
//#define ISCANC	0000010		/* creature has special qualities cancelled */
pub const ISCANC: usize = 10;
//#define ISLEVIT	0000010		/* hero is levitating */
pub const ISLEVIT: usize = 10;
//#define ISFOUND	0000020		/* creature has been seen (used for objects) */
pub const ISFOUND: usize = 20;
//#define ISGREED	0000040		/* creature runs to protect gold */
pub const ISGREED: usize = 40;
//#define ISHASTE	0000100		/* creature has been hastened */
pub const ISHASTE: usize = 100;
//#define ISTARGET 000200		/* creature is the target of an 'f' command */
pub const ISTARGET: usize = 200;
//#define ISHELD	0000400		/* creature has been held */
pub const ISHELD: usize = 400;
//#define ISHUH	0001000		/* creature is confused */
pub const ISHUH: usize = 1000;
//#define ISINVIS	0002000		/* creature is invisible */
pub const ISINVIS: usize = 2000;
//#define ISMEAN	0004000		/* creature can wake when player enters room */
pub const ISMEAN: usize = 4000;
//#define ISHALU	0004000		/* hero is on acid trip */
pub const ISHALU: usize = 4000;
//#define ISREGEN	0010000		/* creature can regenerate */
pub const ISREGEN: usize = 10000;
//#define ISRUN	0020000		/* creature is running at the player */
pub const ISRUN: usize = 20000;
//#define SEEMONST 040000		/* hero can detect unseen monsters */
pub const SEEMONST: usize = 40000;
//#define ISFLY	0040000		/* creature can fly */
pub const ISFLY: usize = 400000;
//#define ISSLOW	0100000		/* creature has been slowed */
pub const ISSLOW: usize = 1000000;

/*
 * Flags for level map
 */
//#define F_PASS		0x80		/* is a passageway */
pub const F_PASS: usize = 0x80;
//#define F_SEEN		0x40		/* have seen this spot before */
pub const F_SEEN: usize = 0x40;
//#define F_DROPPED	0x20		/* object was dropped here */
pub const F_DROPPED: usize = 0x20;
//#define F_LOCKED	0x20		/* door is locked */
pub const F_LOCKED: usize = 0x20;
//#define F_REAL		0x10		/* what you see is what you get */
pub const F_REAL: usize = 0x10;
//#define F_PNUM		0x0f		/* passage number mask */
pub const F_PNUM: usize = 0x0f;
//#define F_TMASK		0x07		/* trap number mask */
pub const F_TMASK: usize = 0x07;

/*
 * Trap types
 */
//#define T_DOOR	00
pub const T_DOOR: usize = 0;
//#define T_ARROW	01
pub const T_ARROW: usize = 1;
//#define T_SLEEP	02
pub const T_SLEEP: usize = 2;
//#define T_BEAR	03
pub const T_BEAR: usize = 3;
//#define T_TELEP	04
pub const T_TELEP: usize = 4;
//#define T_DART	05
pub const T_DART: usize = 5;
//#define T_RUST	06
pub const T_RUST: usize = 6;
//#define T_MYST  07
pub const T_MYST: usize = 7;
//#define NTRAPS	8
pub const NTRAPS: usize = 8;

/*
 * Potion types
 */
//#define P_CONFUSE	0
pub const P_CONFUSE: usize = 0;
//#define P_LSD		1
pub const P_LSD: usize = 1;
//#define P_POISON	2
pub const P_POISON: usize = 2;
//#define P_STRENGTH	3
pub const P_STRENGTH: usize = 3;
//#define P_SEEINVIS	4
pub const P_SEEINVIS: usize = 4;
//#define P_HEALING	5
pub const P_HEALING: usize = 5;
//#define P_MFIND		6
pub const P_MFIND: usize = 6;
//#define	P_TFIND 	7
pub const P_TFIND: usize = 7;
//#define	P_RAISE		8
pub const P_RAISE: usize = 8;
//#define P_XHEAL		9
pub const P_XHEAL: usize = 9;
//#define P_HASTE		10
pub const P_HASTE: usize = 10;
//#define P_RESTORE	11
pub const P_RESTORE: usize = 11;
//#define P_BLIND		12
pub const P_BLIND: usize = 12;
//#define P_LEVIT		13
pub const P_LEVIT: usize = 13;
//#define MAXPOTIONS	14
pub const MAXPOTIONS: usize = 14;

/*
 * Scroll types
 */
//#define S_CONFUSE	0
pub const S_CONFUSE: usize = 0;
//#define S_MAP		1
pub const S_MAP: usize = 1;
//#define S_HOLD		2
pub const S_HOLD: usize = 2;
//#define S_SLEEP		3
pub const S_SLEEP: usize = 3;
//#define S_ARMOR		4
pub const S_ARMOR: usize = 4;
//#define S_ID_POTION	5
pub const S_ID_POTION: usize = 5;
//#define S_ID_SCROLL	6
pub const S_ID_SCROLL: usize = 6;
//#define S_ID_WEAPON	7
pub const S_ID_WEAPON: usize = 7;
//#define S_ID_ARMOR	8
pub const S_ID_ARMOR: usize = 8;
//#define S_ID_R_OR_S	9
pub const S_ID_R_OR_S: usize = 9;
//#define S_SCARE		10
pub const S_SCARE: usize = 10;
//#define S_FDET		11
pub const S_FDET: usize = 11;
//#define S_TELEP		12
pub const S_TELEP: usize = 12;
//#define S_ENCH		13
pub const S_ENCH: usize = 13;
//#define S_CREATE	14
pub const S_CREATE: usize = 14;
//#define S_REMOVE	15
pub const S_REMOVE: usize = 15;
//#define S_AGGR		16
pub const S_AGGR: usize = 16;
//#define S_PROTECT	17
pub const S_PROTECT: usize = 17;
//#define MAXSCROLLS	18
pub const MAXSCROLLS: usize = 18;

/*
 * Weapon types
 */
//#define MACE		0
pub const MACE: usize = 0;
//#define SWORD		1
pub const SWORD: usize = 1;
//#define BOW		2
pub const BOW: usize = 2;
//#define ARROW		3
pub const ARROW: usize = 3;
//#define DAGGER		4
pub const DAGGER: usize = 4;
//#define TWOSWORD	5
pub const TWOSWORD: usize = 5;
//#define DART		6
pub const DART: usize = 6;
//#define SHIRAKEN	7
pub const SHIRAKEN: usize = 7;
//#define SPEAR		8
pub const SPEAR: usize = 8;
//#define FLAME		9	/* fake entry for dragon breath (ick) */
pub const FLAME: usize = 9;
//#define MAXWEAPONS	9	/* this should equal FLAME */
pub const MAXWEAPONS: usize = 9;

/*
 * Armor types
 */
//#define LEATHER		0
pub const LEATHER: usize = 0;
//#define RING_MAIL	1
pub const RING_MAIL: usize = 1;
//#define STUDDED_LEATHER	2
pub const STUDDED_LEATHER: usize = 2;
//#define SCALE_MAIL	3
pub const SCALE_MAIL: usize = 3;
//#define CHAIN_MAIL	4
pub const CHAIN_MAIL: usize = 4;
//#define SPLINT_MAIL	5
pub const SPLINT_MAIL: usize = 5;
//#define BANDED_MAIL	6
pub const BANDED_MAIL: usize = 6;
//#define PLATE_MAIL	7
pub const PLATE_MAIL: usize = 7;
//#define MAXARMORS	8
pub const MAXARMORS: usize = 8;

/*
 * Ring types
 */
//#define R_PROTECT	0
pub const R_PROTECT: usize = 0;
//#define R_ADDSTR	1
pub const R_ADDSTR: usize = 1;
//#define R_SUSTSTR	2
pub const R_SUSTSTR: usize = 2;
//#define R_SEARCH	3
pub const R_SEARCH: usize = 3;
//#define R_SEEINVIS	4
pub const R_SEEINVIS: usize = 4;
//#define R_NOP		5
pub const R_NOP: usize = 5;
//#define R_AGGR		6
pub const R_AGGR: usize = 6;
//#define R_ADDHIT	7
pub const R_ADDHIT: usize = 7;
//#define R_ADDDAM	8
pub const R_ADDDAM: usize = 8;
//#define R_REGEN		9
pub const R_REGEN: usize = 9;
//#define R_DIGEST	10
pub const R_DIGEST: usize = 10;
//#define R_TELEPORT	11
pub const R_TELEPORT: usize = 11;
//#define R_STEALTH	12
pub const R_STEALTH: usize = 12;
//#define R_SUSTARM	13
pub const R_SUSTARM: usize = 13;
//#define MAXRINGS	14
pub const MAXRINGS: usize = 14;

/*
 * Rod/Wand/Staff types
 */
//#define WS_LIGHT	0
pub const WS_LIGHT: usize = 0;
//#define WS_INVIS	1
pub const WS_INVIS: usize = 1;
//#define WS_ELECT	2
pub const WS_ELECT: usize = 2;
//#define WS_FIRE		3
pub const WS_FIRE: usize = 3;
//#define WS_COLD		4
pub const WS_COLD: usize = 4;
//#define WS_POLYMORPH	5
pub const WS_POLYMORPH: usize = 5;
//#define WS_MISSILE	6
pub const WS_MISSILE: usize = 6;
//#define WS_HASTE_M	7
pub const WS_HASTE_M: usize = 7;
//#define WS_SLOW_M	8
pub const WS_SLOW_M: usize = 8;
//#define WS_DRAIN	9
pub const WS_DRAIN: usize = 9;
//#define WS_NOP		10
pub const WS_NOP: usize = 10;
//#define WS_TELAWAY	11
pub const WS_TELAWAY: usize = 11;
//#define WS_TELTO	12
pub const WS_TELTO: usize = 12;
//#define WS_CANCEL	13
pub const WS_CANCEL: usize = 13;
//#define MAXSTICKS	14
pub const MAXSTICKS: usize = 14;

/*
 * Coordinate data type
 */
/*typedef struct {
    int x;
    int y;
} coord;*/
#[derive(Copy, Clone, PartialEq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

//typedef unsigned int str_t;
pub type str_t = u32;

/*
 * Stuff about objects
 */
/*struct obj_info {
    char *oi_name;
    int oi_prob;
    int oi_worth;
    char *oi_guess;
    bool oi_know;
};*/
pub struct ObjInfo {
    pub name: String,
    pub prob: i32,
    pub worth: i32,
    pub guess: String,
    pub know: bool,
}

/*
 * Room structure
 */
/*struct room {
    coord r_pos;			/* Upper left corner */
    coord r_max;			/* Size of room */
    coord r_gold;			/* Where the gold is */
    int r_goldval;			/* How much the gold is worth */
    short r_flags;			/* info about the room */
    int r_nexits;			/* Number of exits */
    coord r_exit[12];			/* Where the exits are */
};*/
#[derive(Copy, Clone, PartialEq)]
pub struct Room {
    pub pos: Coord,
    pub max: Coord,
    pub gold: Coord,
    pub goldval: i32,
    pub flags: usize,
    pub nexits: i32,
    pub exit: [Coord; 12],
}

impl Room {
    pub fn new() -> Self {
        Room {
            pos: Coord { x: 0, y: 0 },
            max: Coord { x: 0, y: 0 },
            gold: Coord { x: 0, y: 0 },
            goldval: 0,
            flags: 0,
            nexits: 0,
            exit: [Coord { x: 0, y: 0 }; 12],
        }
    }
}

/*
 * Structure describing a fighting being
 */
/*struct stats {
    str_t s_str;			/* Strength */
    int s_exp;				/* Experience */
    int s_lvl;				/* level of mastery */
    int s_arm;				/* Armor class */
    int s_hpt;			/* Hit points */
    char s_dmg[13];			/* String describing damage done */
    int  s_maxhp;			/* Max hit points */
};*/
#[derive(Clone, PartialEq)]
pub struct Stats {
    pub str: str_t,
    pub exp: i32,
    pub lvl: usize,
    pub arm: i32,
    pub hpt: i32,
    pub dmg: String,
    pub max_hp: usize,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            str: 0,
            exp: 0,
            lvl: 0,
            arm: 0,
            hpt: 0,
            dmg: String::new(),
            max_hp: 0,
        }
    }
}

/*
 * Structure for monsters and player
 */
/*union thing {
    struct {
	union thing *_l_next, *_l_prev;	/* Next pointer in link */
	coord _t_pos;			/* Position */
	bool _t_turn;			/* If slowed, is it a turn to move */
	char _t_type;			/* What it is */
	char _t_disguise;		/* What mimic looks like */
	char _t_oldch;			/* Character that was where it was */
	coord *_t_dest;			/* Where it is running to */
	short _t_flags;			/* State word */
	struct stats _t_stats;		/* Physical description */
	struct room *_t_room;		/* Current room for thing */
	union thing *_t_pack;		/* What the thing is carrying */
        int _t_reserved;
    } _t;
    struct {
	union thing *_l_next, *_l_prev;	/* Next pointer in link */
	int _o_type;			/* What kind of object it is */
	coord _o_pos;			/* Where it lives on the screen */
	char *_o_text;			/* What it says if you read it */
	int  _o_launch;			/* What you need to launch it */
	char _o_packch;			/* What character it is in the pack */
	char _o_damage[8];		/* Damage if used like sword */
	char _o_hurldmg[8];		/* Damage if thrown */
	int _o_count;			/* count for plural objects */
	int _o_which;			/* Which object of a type it is */
	int _o_hplus;			/* Plusses to hit */
	int _o_dplus;			/* Plusses to damage */
	int _o_arm;			/* Armor protection */
	int _o_flags;			/* information about objects */
	int _o_group;			/* group number for this object */
	char *_o_label;			/* Label for object */
    } _o;
};*/
#[derive(Clone, PartialEq)]
pub enum Thing {
    Creature {
        next: Box<Option<Thing>>,
        prev: Box<Option<Thing>>,
        pos: Coord,
        turn: bool,
        r#type: char,
        disguise: char,
        oldch: char,
        dest: Option<Coord>,
        flags: usize,
        stats: Stats,
        room: Box<Room>,
        pack: Box<Option<(Thing, /* reserved */i32)>>,
    },
    Object {
        next: Box<Option<Thing>>,
        prev: Box<Option<Thing>>,
        r#type: i32,
        pos: Coord,
        text: String,
        launch: i32,
        packch: char,
        damage: String,
        hurldmg: String,
        count: i32,
        which: usize,
        hplus: i32,
        dplus: i32,
        arm: isize,
        flags: usize,
        group: i32,
        label: String,
    },
}

impl Thing {
    pub fn new_creature() -> Self {
        Thing::Creature {
            next: Box::new(None),
            prev: Box::new(None),
            pos: Coord { x: 0, y: 0 },
            turn: false,
            r#type: '\0',
            disguise: '\0',
            oldch: '\0',
            dest: None,
            flags: 0,
            stats: Stats::new(),
            room: Box::new(Room::new()),
            pack: Box::new(None),
        }
    }
}


//typedef union thing THING;
pub type THING = Thing;

//#define l_next		_t._l_next
//#define l_prev		_t._l_prev
//#define t_pos		_t._t_pos
//#define t_turn		_t._t_turn
//#define t_type		_t._t_type
//#define t_disguise	_t._t_disguise
//#define t_oldch		_t._t_oldch
//#define t_dest		_t._t_dest
//#define t_flags		_t._t_flags
//#define t_stats		_t._t_stats
//#define t_pack		_t._t_pack
//#define t_room		_t._t_room
//#define t_reserved  _t._t_reserved
//#define o_type		_o._o_type
//#define o_pos		_o._o_pos
//#define o_text		_o._o_text
//#define o_launch	_o._o_launch
//#define o_packch	_o._o_packch
//#define o_damage	_o._o_damage
//#define o_hurldmg	_o._o_hurldmg
//#define o_count		_o._o_count
//#define o_which		_o._o_which
//#define o_hplus		_o._o_hplus
//#define o_dplus		_o._o_dplus
//#define o_arm		_o._o_arm
//#define o_charges	o_arm
//#define o_goldval	o_arm
//#define o_flags		_o._o_flags
//#define o_group		_o._o_group
//#define o_label		_o._o_label

/*
 * describe a place on the level map
 */
/*typedef struct {
    char p_ch;
    char p_flags;
    THING *p_monst;
} PLACE;*/
#[derive(Clone)]
pub struct Place {
    pub ch: char,
    pub flags: char,
    pub monst: Option<Thing>,
}

impl Place {
    pub fn new() -> Self {
        Place {
            ch: '\0',
            flags: '\0',
            monst: None,
        }
    }
}

/*
 * Array containing information on all the various types of monsters
 */
/*struct monster {
    char *m_name;			/* What to call the monster */
    int m_carry;			/* Probability of carrying something */
    short m_flags;			/* things about the monster */
    struct stats m_stats;		/* Initial stats */
};*/
pub struct Monster {
    pub name: String,
    pub carry: usize,
    pub flags: usize,
    pub stats: Stats,
}

//struct room	*roomin(coord *cp);

//#define MAXDAEMONS 20
const MAXDAEMONS: usize = 20;

/*extern struct delayed_action {
    int d_type;
    void (*d_func)();
    int d_arg;
    int d_time;
} d_list[MAXDAEMONS];*/
pub struct DelayedAction {
    pub r#type: i32,
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
    pub value: i32,
}

pub const PRESS_SPACE_TO_CONTINUE: &str = "--Press space to continue--";