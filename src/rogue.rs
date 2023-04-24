use crate::extern_c::*;
use crate::extern_h::*;

//#undef lines 

//#define CCHAR(x) ( (char) (x & A_CHARTEXT) )
/*
 * Maximum number of different things
 */
//#define MAXROOMS	9
pub const MAXROOMS: usize = 9;
//#define MAXTHINGS	9
pub const MAXTHINGS: usize = 9;
//#define MAXOBJ		9
pub const MAXOBJ: usize = 9;
//#define MAXPACK		23
pub const MAXPACK: usize = 23;
//#define MAXTRAPS	10
pub const MAXTRAPS: usize = 10;
//#define AMULETLEVEL	26
pub const AMULETLEVEL: usize = 26;
//#define	NUMTHINGS	7	/* number of types of things */
pub const NUMTHINGS: usize = 7;
//#define MAXPASS		13	/* upper limit on number of passages */
pub const MAXPASS: usize = 13;
//#define	NUMLINES	24
pub const NUMLINES: usize = 24;
//#define	NUMCOLS		80
pub const NUMCOLS: usize = 80;
//#define STATLINE		(NUMLINES - 1)
pub const STATLINE: usize = NUMLINES - 1;
//#define BORE_LEVEL	50
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
 * All the fun defines
 */
//#define when		break;case
//#define otherwise	break;default
//#define until(expr)	while(!(expr))
//#define next(ptr)	(*ptr).l_next
//#define prev(ptr)	(*ptr).l_prev
//#define winat(y,x)	(moat(y,x) != NULL ? moat(y,x)->t_disguise : chat(y,x))
//#define ce(a,b)		((a).x == (b).x && (a).y == (b).y)
//#define hero		player.t_pos
//#define pstats		player.t_stats
//#define pack		player.t_pack
//#define proom		player.t_room
//#define max_hp		player.t_stats.s_maxhp
//#define attach(a,b)	_attach(&a,b)
//#define detach(a,b)	_detach(&a,b)
//#define free_list(a)	_free_list(&a)
//#undef max
//#define max(a,b)	((a) > (b) ? (a) : (b))
//#define on(thing,flag)	((bool)(((thing).t_flags & (flag)) != 0))
//#define GOLDCALC	(rnd(50 + 10 * level) + 2)
//#define ISRING(h,r)	(cur_ring[h] != NULL && cur_ring[h]->o_which == r)
pub fn is_ring(h: usize, r: usize) -> bool {
    match cur_ring[h] {
        Some(ref ring) => {
            match ring {
                Thing::Object { next, prev, r#type, pos, text, launch, packch, damage, hurldmg, count, which, hplus, dplus, arm, flags, group, label } => {
                    *which == r
                },
                _ => false
            }
        },
        None => false
    }
}
//#define ISWEARING(r)	(ISRING(LEFT, r) || ISRING(RIGHT, r))
//#define ISMULT(type) 	(type == POTION || type == SCROLL || type == FOOD)
//#define INDEX(y,x)	(&places[((x) << 5) + (y)])
//#define chat(y,x)	(places[((x) << 5) + (y)].p_ch)
//#define flat(y,x)	(places[((x) << 5) + (y)].p_flags)
//#define moat(y,x)	(places[((x) << 5) + (y)].p_monst)
//#define unc(cp)		(cp).y, (cp).x

/*
 * things that appear on the screens
 */
//#define PASSAGE		'#'
const PASSAGE: char = '#';
//#define DOOR		'+'
const DOOR: char = '+';
//#define FLOOR		'.'
const FLOOR: char = '.';
//#define PLAYER		'@'
const PLAYER: char = '@';
//#define TRAP		'^'
const TRAP: char = '^';
//#define STAIRS		'%'
const STAIRS: char = '%';
//#define GOLD		'*'
const GOLD: char = '*';
//#define POTION		'!'
const POTION: char = '!';
//#define SCROLL		'?'
const SCROLL: char = '?';
//#define MAGIC		'$'
const MAGIC: char = '$';
//#define FOOD		':'
const FOOD: char = ':';
//#define WEAPON		')'
const WEAPON: char = ')';
//#define ARMOR		']'
const ARMOR: char = ']';
//#define AMULET		','
const AMULET: char = ',';
//#define RING		'='
const RING: char = '=';
//#define STICK		'/'
const STICK: char = '/';
//#define CALLABLE	-1
const CALLABLE: isize = -1;
//#define R_OR_S		-2
const R_OR_S: isize = -2;

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
pub const ESCAPE: char = '\x1b';
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
 * Now we define the structures and types
 */
/*
 * Help list
 */
/*struct h_list {
    char h_ch;
    char *h_desc;
    bool h_print;
};*/
pub struct HList {
    pub ch: char,
    pub desc: String,
    pub print: bool,
}

/*
 * Coordinate data type
 */
/*typedef struct {
    int x;
    int y;
} coord;*/
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Clone)]
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
#[derive(Clone)]
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
    pub carry: i32,
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
