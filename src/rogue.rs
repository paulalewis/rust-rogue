use crate::{utils::spread, constants::{NUMTHINGS, ISMEAN, ISFLY, ISREGEN, ISGREED, ISINVIS, MAXPOTIONS, MAXARMORS, MAXRINGS, MAXSCROLLS, MAXWEAPONS, MAXSTICKS}, core::{coord::Coord, object::Object, creature::Creature, room::Room, stats::Stats}};

// This file contains global values for the game

//bool after;				/* True if we want after daemons */
pub static mut after: bool = false;
//bool again;				/* Repeating the last command */
pub static again: bool = false;
//bool seenstairs;			/* Have seen the stairs (for lsd) */
pub static seenstairs: bool = false;
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
pub static passages: Vec<Room> = Vec::new();

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
pub static things: [ObjInfo; NUMTHINGS] = [
    ObjInfo { name: String::new(), prob: 26, worth: 0, guess: String::new(), know: false },/* potion */
    ObjInfo { name: String::new(), prob: 62, worth: 0, guess: String::new(), know: false },/* scroll */
    ObjInfo { name: String::new(), prob: 78, worth: 0, guess: String::new(), know: false },/* food */
    ObjInfo { name: String::new(), prob: 85, worth: 0, guess: String::new(), know: false },/* weapon */
    ObjInfo { name: String::new(), prob: 92, worth: 0, guess: String::new(), know: false },/* armor */
    ObjInfo { name: String::new(), prob: 96, worth: 0, guess: String::new(), know: false },/* ring */
    ObjInfo { name: String::new(), prob: 100, worth: 0, guess: String::new(), know: false },/* stick */
];

lazy_static! {
    //struct obj_info arm_info[MAXARMORS]
    pub static ref ARMOR_INFO: [ObjInfo; MAXARMORS] = [
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

lazy_static! {
    //struct obj_info pot_info[MAXPOTIONS]
    pub static ref POTION_INFO: [ObjInfo; MAXPOTIONS] = [
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

lazy_static! {
    //struct obj_info ring_info[MAXRINGS]
    pub static ref RING_INFO: [ObjInfo; MAXRINGS] = [
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

lazy_static! {
    //struct obj_info scr_info[MAXSCROLLS]
    pub static ref SCROLL_INFO: [ObjInfo; MAXSCROLLS] = [
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

lazy_static! {
    //struct obj_info weap_info[MAXWEAPONS + 1]
    pub static ref WEAPON_INFO: [ObjInfo; MAXWEAPONS] = [
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
        //ObjInfo { name: String::new(), prob: 0, worth: 0, guess: String::new(), know: false },
    ];
}

lazy_static! {
    //struct obj_info ws_info[MAXSTICKS]
    pub static ref STICK_INFO: [ObjInfo; MAXSTICKS] = [
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
    pub prob: usize,
    pub worth: usize,
    pub guess: String,
    pub know: bool,
}

// describe a place on the level map
/*typedef struct {
    char p_ch;
    char p_flags;
    THING *p_monst;
} PLACE;*/
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Place {
    pub ch: char,
    pub flags: usize,
    pub monst: Option<Creature>,
}

impl Place {
    pub fn new() -> Self {
        Place {
            ch: '\0',
            flags: 0,
            monst: None,
        }
    }
}

// Array containing information on all the various types of monsters
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
