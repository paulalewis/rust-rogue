use crate::{utils::spread, rogue_state::RogueState, constants::{NUMTHINGS, ISMEAN, ISFLY, ISREGEN, ISGREED, ISINVIS, MAXPOTIONS, MAXARMORS, MAXRINGS, MAXSCROLLS, MAXWEAPONS, MAXSTICKS}};

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
//bool kamikaze = FALSE;			/* to_death really to DEATH */
pub static kamikaze: bool = false;
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
//char *ws_made[MAXSTICKS];		/* What sticks are made of */
//char *ws_type[MAXSTICKS];		/* Is it a wand or a staff */
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
//int max_hit;				/* Max damage done to her in to_death */
pub static max_hit: usize = 0;
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
// coord oldpos;				/* Position before last look() call */
pub static oldpos: Coord = Coord { y: 0, x: 0 };
// coord stairs;				/* Location of staircase */
pub static stairs: Coord = Coord { y: 0, x: 0 };

//PLACE places[MAXLINES*MAXCOLS];		/* level map */
//static places: [Option<Place>; MAXLINES * MAXCOLS] = [None; MAXLINES * MAXCOLS];
pub static places: Vec<Place> = Vec::new();

//THING *l_last_pick = NULL;		/* Last last_pick */
pub static l_last_pick: Option<Thing> = None;
//THING *last_pick = NULL;		/* Last object picked in get_item() */
pub static last_pick: Option<Thing> = None;
//THING *lvl_obj = NULL;			/* List of objects on this level */
pub static lvl_obj: Option<Thing> = None;
//THING *mlist = NULL;			/* List of monsters on the level */
pub static mlist: Option<Thing> = None;

//#define INIT_STATS { 16, 0, 1, 10, 12, "1x4", 12 }
//struct stats max_stats = INIT_STATS;	/* The maximum for the player */
const MAX_INITIAL_STR: usize = 16;
pub static max_str: usize = MAX_INITIAL_STR;

//struct room *oldrp;			/* Roomin(&oldpos) */
pub static oldrp: Option<Room> = None;

//struct room rooms[MAXROOMS];		/* One for each room -- A level */
//static rooms: [Room; MAXROOMS] = [Room::new(); MAXROOMS];
pub static rooms: Vec<Room> = Vec::new();

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

//struct obj_info arm_info[MAXARMORS]
lazy_static! {
    pub static ref arm_info: [ObjInfo; MAXARMORS] = [
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
    pub static ref pot_info: [ObjInfo; MAXPOTIONS] = [
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
    pub static ref scr_info: [ObjInfo; MAXSCROLLS] = [
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
    pub static ref weap_info: [ObjInfo; MAXWEAPONS + 1] = [
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
    pub static ref ws_info: [ObjInfo; MAXSTICKS] = [
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


// coord
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

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
#[derive(Debug, Copy, Clone, PartialEq)]
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

/*struct stats {
    uint s_str;			/* Strength */
    int s_exp;				/* Experience */
    int s_lvl;				/* level of mastery */
    int s_arm;				/* Armor class */
    int s_hpt;			/* Hit points */
    char s_dmg[13];			/* String describing damage done */
    int  s_maxhp;			/* Max hit points */
};*/
#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub str: usize,
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
//typedef union thing THING;
#[derive(Debug, Clone, PartialEq)]
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

// describe a place on the level map
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
