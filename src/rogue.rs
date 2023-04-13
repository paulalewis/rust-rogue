/*
 * Rogue definitions and variable declarations
 *
 * @(#)rogue.h	5.42 (Berkeley) 08/06/83
 *
 * Rogue: Exploring the Dungeons of Doom
 * Copyright (C) 1980-1983, 1985, 1999 Michael Toy, Ken Arnold and Glenn Wichman
 * All rights reserved.
 *
 * See the file LICENSE.TXT for full copyright and licensing information.
 */

//#include "extern.h"

//#undef lines 

//#define NOOP(x) (x += 0)
//#define CCHAR(x) ( (char) (x & A_CHARTEXT) )
/*
 * Maximum number of different things
 */
//#define MAXROOMS	9
const MAXROOMS: usize = 9;
//#define MAXTHINGS	9
const MAXTHINGS: usize = 9;
//#define MAXOBJ		9
const MAXOBJ: usize = 9;
//#define MAXPACK		23
const MAXPACK: usize = 23;
//#define MAXTRAPS	10
const MAXTRAPS: usize = 10;
//#define AMULETLEVEL	26
const AMULETLEVEL: usize = 26;
//#define	NUMTHINGS	7	/* number of types of things */
const NUMTHINGS: usize = 7;
//#define MAXPASS		13	/* upper limit on number of passages */
const MAXPASS: usize = 13;
//#define	NUMLINES	24
const NUMLINES: usize = 24;
//#define	NUMCOLS		80
const NUMCOLS: usize = 80;
//#define STATLINE		(NUMLINES - 1)
const STATLINE: usize = NUMLINES - 1;
//#define BORE_LEVEL	50
const BORE_LEVEL: usize = 50;

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
 * inventory types
 */
//#define	INV_OVER	0
const INV_OVER: usize = 0;
//#define	INV_SLOW	1
const INV_SLOW: usize = 1;
//#define	INV_CLEAR	2
const INV_CLEAR: usize = 2;

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
//#define ISWEARING(r)	(ISRING(LEFT, r) || ISRING(RIGHT, r))
//#define ISMULT(type) 	(type == POTION || type == SCROLL || type == FOOD)
//#define INDEX(y,x)	(&places[((x) << 5) + (y)])
//#define chat(y,x)	(places[((x) << 5) + (y)].p_ch)
//#define flat(y,x)	(places[((x) << 5) + (y)].p_flags)
//#define moat(y,x)	(places[((x) << 5) + (y)].p_monst)
//#define unc(cp)		(cp).y, (cp).x
//#ifdef MASTER
//#define debug		if (wizard) msg
//#endif

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
const HEALTIME: usize = 30;
//#define HUHDURATION	20
const HUHDURATION: usize = 20;
//#define SEEDURATION	850
const SEEDURATION: usize = 850;
//#define HUNGERTIME	1300
const HUNGERTIME: usize = 1300;
//#define MORETIME	150
const MORETIME: usize = 150;
//#define STOMACHSIZE	2000
const STOMACHSIZE: usize = 2000;
//#define STARVETIME	850
const STARVETIME: usize = 850;
//#define ESCAPE		27
const ESCAPE: char = '\x1b';
//#define LEFT		0
const LEFT: usize = 0;
//#define RIGHT		1
const RIGHT: usize = 1;
//#define BOLT_LENGTH	6
const BOLT_LENGTH: usize = 6;
//#define LAMPDIST	3
const LAMPDIST: usize = 3;

/*
 * Save against things
 */
//#define VS_POISON	00
const VS_POISON: usize = 0;
//#define VS_PARALYZATION	00
const VS_PARALYZATION: usize = 0;
//#define VS_DEATH	00
const VS_DEATH: usize = 0;
//#define VS_BREATH	02
const VS_BREATH: usize = 2;
//#define VS_MAGIC	03
const VS_MAGIC: usize = 3;

/*
 * Various flag bits
 */
/* flags for rooms */
//#define ISDARK	0000001		/* room is dark */
const ISDARK: usize = 1;
//#define ISGONE	0000002		/* room is gone (a corridor) */
const ISGONE: usize = 2;
//#define ISMAZE	0000004		/* room is gone (a corridor) */
const ISMAZE: usize = 4;

/* flags for objects */
//#define ISCURSED 000001		/* object is cursed */
const ISCURSED: usize = 1;
//#define ISKNOW	0000002		/* player knows details about the object */
const ISKNOW: usize = 2;
//#define ISMISL	0000004		/* object is a missile type */
const ISMISL: usize = 4;
//#define ISMANY	0000010		/* object comes in groups */
const ISMANY: usize = 10;
/*	ISFOUND 0000020		...is used for both objects and creatures */
//#define	ISPROT	0000040		/* armor is permanently protected */
const ISPROT: usize = 40;

/* flags for creatures */
//#define CANHUH	0000001		/* creature can confuse */
const CANHUH: usize = 1;
//#define CANSEE	0000002		/* creature can see invisible creatures */
const CANSEE: usize = 2;
//#define ISBLIND	0000004		/* creature is blind */
const ISBLIND: usize = 4;
//#define ISCANC	0000010		/* creature has special qualities cancelled */
const ISCANC: usize = 10;
//#define ISLEVIT	0000010		/* hero is levitating */
const ISLEVIT: usize = 10;
//#define ISFOUND	0000020		/* creature has been seen (used for objects) */
const ISFOUND: usize = 20;
//#define ISGREED	0000040		/* creature runs to protect gold */
const ISGREED: usize = 40;
//#define ISHASTE	0000100		/* creature has been hastened */
const ISHASTE: usize = 100;
//#define ISTARGET 000200		/* creature is the target of an 'f' command */
const ISTARGET: usize = 200;
//#define ISHELD	0000400		/* creature has been held */
const ISHELD: usize = 400;
//#define ISHUH	0001000		/* creature is confused */
const ISHUH: usize = 1000;
//#define ISINVIS	0002000		/* creature is invisible */
const ISINVIS: usize = 2000;
//#define ISMEAN	0004000		/* creature can wake when player enters room */
const ISMEAN: usize = 4000;
//#define ISHALU	0004000		/* hero is on acid trip */
const ISHALU: usize = 4000;
//#define ISREGEN	0010000		/* creature can regenerate */
const ISREGEN: usize = 10000;
//#define ISRUN	0020000		/* creature is running at the player */
const ISRUN: usize = 20000;
//#define SEEMONST 040000		/* hero can detect unseen monsters */
const SEEMONST: usize = 40000;
//#define ISFLY	0040000		/* creature can fly */
const ISFLY: usize = 400000;
//#define ISSLOW	0100000		/* creature has been slowed */
const ISSLOW: usize = 1000000;

/*
 * Flags for level map
 */
//#define F_PASS		0x80		/* is a passageway */
const F_PASS: usize = 0x80;
//#define F_SEEN		0x40		/* have seen this spot before */
const F_SEEN: usize = 0x40;
//#define F_DROPPED	0x20		/* object was dropped here */
const F_DROPPED: usize = 0x20;
//#define F_LOCKED	0x20		/* door is locked */
const F_LOCKED: usize = 0x20;
//#define F_REAL		0x10		/* what you see is what you get */
const F_REAL: usize = 0x10;
//#define F_PNUM		0x0f		/* passage number mask */
const F_PNUM: usize = 0x0f;
//#define F_TMASK		0x07		/* trap number mask */
const F_TMASK: usize = 0x07;

/*
 * Trap types
 */
//#define T_DOOR	00
const T_DOOR: usize = 0;
//#define T_ARROW	01
const T_ARROW: usize = 1;
//#define T_SLEEP	02
const T_SLEEP: usize = 2;
//#define T_BEAR	03
const T_BEAR: usize = 3;
//#define T_TELEP	04
const T_TELEP: usize = 4;
//#define T_DART	05
const T_DART: usize = 5;
//#define T_RUST	06
const T_RUST: usize = 6;
//#define T_MYST  07
const T_MYST: usize = 7;
//#define NTRAPS	8
const NTRAPS: usize = 8;

/*
 * Potion types
 */
//#define P_CONFUSE	0
const P_CONFUSE: usize = 0;
//#define P_LSD		1
const P_LSD: usize = 1;
//#define P_POISON	2
const P_POISON: usize = 2;
//#define P_STRENGTH	3
const P_STRENGTH: usize = 3;
//#define P_SEEINVIS	4
const P_SEEINVIS: usize = 4;
//#define P_HEALING	5
const P_HEALING: usize = 5;
//#define P_MFIND		6
const P_MFIND: usize = 6;
//#define	P_TFIND 	7
const P_TFIND: usize = 7;
//#define	P_RAISE		8
const P_RAISE: usize = 8;
//#define P_XHEAL		9
const P_XHEAL: usize = 9;
//#define P_HASTE		10
const P_HASTE: usize = 10;
//#define P_RESTORE	11
const P_RESTORE: usize = 11;
//#define P_BLIND		12
const P_BLIND: usize = 12;
//#define P_LEVIT		13
const P_LEVIT: usize = 13;
//#define MAXPOTIONS	14
const MAXPOTIONS: usize = 14;

/*
 * Scroll types
 */
//#define S_CONFUSE	0
const S_CONFUSE: usize = 0;
//#define S_MAP		1
const S_MAP: usize = 1;
//#define S_HOLD		2
const S_HOLD: usize = 2;
//#define S_SLEEP		3
const S_SLEEP: usize = 3;
//#define S_ARMOR		4
const S_ARMOR: usize = 4;
//#define S_ID_POTION	5
const S_ID_POTION: usize = 5;
//#define S_ID_SCROLL	6
const S_ID_SCROLL: usize = 6;
//#define S_ID_WEAPON	7
const S_ID_WEAPON: usize = 7;
//#define S_ID_ARMOR	8
const S_ID_ARMOR: usize = 8;
//#define S_ID_R_OR_S	9
const S_ID_R_OR_S: usize = 9;
//#define S_SCARE		10
const S_SCARE: usize = 10;
//#define S_FDET		11
const S_FDET: usize = 11;
//#define S_TELEP		12
const S_TELEP: usize = 12;
//#define S_ENCH		13
const S_ENCH: usize = 13;
//#define S_CREATE	14
const S_CREATE: usize = 14;
//#define S_REMOVE	15
const S_REMOVE: usize = 15;
//#define S_AGGR		16
const S_AGGR: usize = 16;
//#define S_PROTECT	17
const S_PROTECT: usize = 17;
//#define MAXSCROLLS	18
const MAXSCROLLS: usize = 18;

/*
 * Weapon types
 */
//#define MACE		0
const MACE: usize = 0;
//#define SWORD		1
const SWORD: usize = 1;
//#define BOW		2
const BOW: usize = 2;
//#define ARROW		3
const ARROW: usize = 3;
//#define DAGGER		4
const DAGGER: usize = 4;
//#define TWOSWORD	5
const TWOSWORD: usize = 5;
//#define DART		6
const DART: usize = 6;
//#define SHIRAKEN	7
const SHIRAKEN: usize = 7;
//#define SPEAR		8
const SPEAR: usize = 8;
//#define FLAME		9	/* fake entry for dragon breath (ick) */
const FLAME: usize = 9;
//#define MAXWEAPONS	9	/* this should equal FLAME */
const MAXWEAPONS: usize = 9;

/*
 * Armor types
 */
//#define LEATHER		0
const LEATHER: usize = 0;
//#define RING_MAIL	1
const RING_MAIL: usize = 1;
//#define STUDDED_LEATHER	2
const STUDDED_LEATHER: usize = 2;
//#define SCALE_MAIL	3
const SCALE_MAIL: usize = 3;
//#define CHAIN_MAIL	4
const CHAIN_MAIL: usize = 4;
//#define SPLINT_MAIL	5
const SPLINT_MAIL: usize = 5;
//#define BANDED_MAIL	6
const BANDED_MAIL: usize = 6;
//#define PLATE_MAIL	7
const PLATE_MAIL: usize = 7;
//#define MAXARMORS	8
const MAXARMORS: usize = 8;

/*
 * Ring types
 */
//#define R_PROTECT	0
const R_PROTECT: usize = 0;
//#define R_ADDSTR	1
const R_ADDSTR: usize = 1;
//#define R_SUSTSTR	2
const R_SUSTSTR: usize = 2;
//#define R_SEARCH	3
const R_SEARCH: usize = 3;
//#define R_SEEINVIS	4
const R_SEEINVIS: usize = 4;
//#define R_NOP		5
const R_NOP: usize = 5;
//#define R_AGGR		6
const R_AGGR: usize = 6;
//#define R_ADDHIT	7
const R_ADDHIT: usize = 7;
//#define R_ADDDAM	8
const R_ADDDAM: usize = 8;
//#define R_REGEN		9
const R_REGEN: usize = 9;
//#define R_DIGEST	10
const R_DIGEST: usize = 10;
//#define R_TELEPORT	11
const R_TELEPORT: usize = 11;
//#define R_STEALTH	12
const R_STEALTH: usize = 12;
//#define R_SUSTARM	13
const R_SUSTARM: usize = 13;
//#define MAXRINGS	14
const MAXRINGS: usize = 14;

/*
 * Rod/Wand/Staff types
 */
//#define WS_LIGHT	0
const WS_LIGHT: usize = 0;
//#define WS_INVIS	1
const WS_INVIS: usize = 1;
//#define WS_ELECT	2
const WS_ELECT: usize = 2;
//#define WS_FIRE		3
const WS_FIRE: usize = 3;
//#define WS_COLD		4
const WS_COLD: usize = 4;
//#define WS_POLYMORPH	5
const WS_POLYMORPH: usize = 5;
//#define WS_MISSILE	6
const WS_MISSILE: usize = 6;
//#define WS_HASTE_M	7
const WS_HASTE_M: usize = 7;
//#define WS_SLOW_M	8
const WS_SLOW_M: usize = 8;
//#define WS_DRAIN	9
const WS_DRAIN: usize = 9;
//#define WS_NOP		10
const WS_NOP: usize = 10;
//#define WS_TELAWAY	11
const WS_TELAWAY: usize = 11;
//#define WS_TELTO	12
const WS_TELTO: usize = 12;
//#define WS_CANCEL	13
const WS_CANCEL: usize = 13;
//#define MAXSTICKS	14
const MAXSTICKS: usize = 14;

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
    h_ch: char,
    h_desc: String,
    h_print: bool,
}

/*
 * Coordinate data type
 */
/*typedef struct {
    int x;
    int y;
} coord;*/
pub struct Coord {
    x: i32,
    y: i32,
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
    oi_name: String,
    oi_prob: i32,
    oi_worth: i32,
    oi_guess: String,
    oi_know: bool,
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
pub struct Room {
    r_pos: Coord,
    r_max: Coord,
    r_gold: Coord,
    r_goldval: i32,
    r_flags: i32,
    r_nexits: i32,
    r_exit: [Coord; 12],
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
pub struct Stats {
    s_str: str_t,
    s_exp: i32,
    s_lvl: i32,
    s_arm: i32,
    s_hpt: i32,
    s_dmg: String,
    s_maxhp: i32,
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
pub enum Thing {
    T {
        _l_next: Box<Thing>,
        _l_prev: Box<Thing>,
        _t_pos: Coord,
        _t_turn: bool,
        _t_type: char,
        _t_disguise: char,
        _t_oldch: char,
        _t_dest: Box<Coord>,
        _t_flags: i32,
        _t_stats: Stats,
        _t_room: Box<Room>,
        _t_pack: Box<Thing>,
        _t_reserved: i32,
    },
    O {
        _l_next: Box<Thing>,
        _l_prev: Box<Thing>,
        _o_type: i32,
        _o_pos: Coord,
        _o_text: String,
        _o_launch: i32,
        _o_packch: char,
        _o_damage: String,
        _o_hurldmg: String,
        _o_count: i32,
        _o_which: i32,
        _o_hplus: i32,
        _o_dplus: i32,
        _o_arm: i32,
        _o_flags: i32,
        _o_group: i32,
        _o_label: String,
    },
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
pub struct Place {
    p_ch: char,
    p_flags: char,
    p_monst: Box<Thing>,
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
    m_name: String,
    m_carry: i32,
    m_flags: i16,
    m_stats: Stats,
}

/*
 * External variables
 */
//extern bool	after, again, allscore, amulet, door_stop, fight_flush,
//		firstmove, has_hit, inv_describe, jump, kamikaze,
//		lower_msg, move_on, msg_esc, pack_used[],
//		passgo, playing, q_comm, running, save_msg, see_floor,
//		seenstairs, stat_msg, terse, to_death, tombstone;

//extern char	dir_ch, file_name[], home[], huh[], *inv_t_name[],
//		l_last_comm, l_last_dir, last_comm, last_dir, *Numname,
//		outbuf[], *p_colors[], *r_stones[], *release, runch,
//		*s_names[], take, *tr_name[], *ws_made[], *ws_type[];

//extern int	a_class[], count, food_left, hungry_state, inpack,
//		inv_type, lastscore, level, max_hit, max_level, mpos,
//		n_objs, no_command, no_food, no_move, noscore, ntraps, purse,
//		quiet, vf_hit;

//extern unsigned int	numscores;

//extern int	dnum, e_levels[], seed;

//extern WINDOW	*hw;

//extern coord	delta, oldpos, stairs;

//extern PLACE	places[];

//extern THING	*cur_armor, *cur_ring[], *cur_weapon, *l_last_pick,
//		*last_pick, *lvl_obj, *mlist, player;

//extern struct h_list	helpstr[];

//extern struct room	*oldrp, passages[], rooms[];

//extern struct stats	max_stats;

//extern struct monster	monsters[];

//extern struct obj_info	arm_info[], pot_info[], ring_info[],
//			scr_info[], things[], ws_info[], weap_info[];

/*
 * Function types
 */
/*
void	_attach(THING **list, THING *item);
void	_detach(THING **list, THING *item);
void	_free_list(THING **ptr);
void	addmsg(char *fmt, ...);
bool	add_haste(bool potion);
void	add_pack(THING *obj, bool silent);
void	add_pass();
void	add_str(str_t *sp, int amt);
void	accnt_maze(int y, int x, int ny, int nx);
void	aggravate();
int	attack(THING *mp);
void	badcheck(char *name, struct obj_info *info, int bound);
void	bounce(THING *weap, char *mname, bool noend);
void	call();
void	call_it(struct obj_info *info);
bool	cansee(int y, int x);
int	center(char *str);
void	chg_str(int amt);
void	check_level();
void	conn(int r1, int r2);
void	command();
void	create_obj();

void	current(THING *cur, char *how, char *where);
void	d_level();
void	death(char monst);
char	death_monst();
void	dig(int y, int x);
void	discard(THING *item);
void	discovered();
int	dist(int y1, int x1, int y2, int x2);
int	dist_cp(coord *c1, coord *c2);
int	do_chase(THING *th);
void	do_daemons(int flag);
void	do_fuses(int flag);
void	do_maze(struct room *rp);
void	do_motion(THING *obj, int ydelta, int xdelta);
void	do_move(int dy, int dx);
void	do_passages();
void	do_pot(int type, bool knowit);
void	do_rooms();
void	do_run(char ch);
void	do_zap();
void	doadd(char *fmt, va_list args);
void	door(struct room *rm, coord *cp);
void	door_open(struct room *rp);
void	drain();
void	draw_room(struct room *rp);
void	drop();
void	eat();
size_t  encread(char *start, size_t size, FILE *inf);
size_t	encwrite(char *start, size_t size, FILE *outf);
int	endmsg();
void	enter_room(coord *cp);
void	erase_lamp(coord *pos, struct room *rp);
int	exp_add(THING *tp);
void	extinguish(void (*func)());
void	fall(THING *obj, bool pr);
void	fire_bolt(coord *start, coord *dir, char *name);
char	floor_at();
void	flush_type();
int	fight(coord *mp, THING *weap, bool thrown);
void	fix_stick(THING *cur);
void	fuse(void (*func)(), int arg, int time, int type);
bool	get_dir();
int	gethand();
void	give_pack(THING *tp);
void	help();
void	hit(char *er, char *ee, bool noend);
void	horiz(struct room *rp, int starty);
void	leave_room(coord *cp);
void	lengthen(void (*func)(), int xtime);
void	look(bool wakeup);
int	hit_monster(int y, int x, THING *obj);
void	identify();
void	illcom(int ch);
void	init_check();
void	init_colors();
void	init_materials();
void	init_names();
void	init_player();
void	init_probs();
void	init_stones();
void	init_weapon(THING *weap, int which);
bool	inventory(THING *list, int type);
void	invis_on();
void	killed(THING *tp, bool pr);
void	kill_daemon(void (*func)());
bool	lock_sc();
void	miss(char *er, char *ee, bool noend);
void	missile(int ydelta, int xdelta);
void	money(int value);
int	move_monst(THING *tp);
void	move_msg(THING *obj);
int	msg(char *fmt, ...);
void	nameit(THING *obj, char *type, char *which, struct obj_info *op, char *(*prfunc)(THING *));
void	new_level();
void	new_monster(THING *tp, char type, coord *cp);
void	numpass(int y, int x);
void	option();
void	open_score();
void	parse_opts(char *str);
void 	passnum();
char	*pick_color(char *col);
int	pick_one(struct obj_info *info, int nitems);
void	pick_up(char ch);
void	picky_inven();
void	pr_spec(struct obj_info *info, int nitems);
void	pr_list();
void	put_bool(void *b);
void	put_inv_t(void *ip);
void	put_str(void *str);
void	put_things();
void	putpass(coord *cp);
void	quaff();
void	raise_level();
char	randmonster(bool wander);
void	read_scroll();
void    relocate(THING *th, coord *new_loc);
void	remove_mon(coord *mp, THING *tp, bool waskill);
void	reset_last();
bool	restore(char *file, char **envp);
int	ring_eat(int hand);
void	ring_on();
void	ring_off();
int	rnd(int range);
int	rnd_room();
int	roll(int number, int sides);
int	rs_save_file(FILE *savef);
int	rs_restore_file(FILE *inf);
void	runto(coord *runner);
void	rust_armor(THING *arm);
int	save(int which);
void	save_file(FILE *savef);
void	save_game();
int	save_throw(int which, THING *tp);
void	score(int amount, int flags, char monst);
void	search();
void	set_know(THING *obj, struct obj_info *info);
void	set_oldch(THING *tp, coord *cp);
void	setup();
void	shell();
bool	show_floor();
void	show_map();
void	show_win(char *message);
int	sign(int nm);
int	spread(int nm);
void	start_daemon(void (*func)(), int arg, int type);
void	start_score();
void	status();
int	step_ok(int ch);
void	strucpy(char *s1, char *s2, int len);
int	swing(int at_lvl, int op_arm, int wplus);
void	take_off();
void	teleport();
void	total_winner();
void	thunk(THING *weap, char *mname, bool noend);
void	treas_room();
void	turnref();
void	u_level();
void	uncurse(THING *obj);
void	unlock_sc();
void	vert(struct room *rp, int startx);
void	wait_for(int ch);
THING  *wake_monster(int y, int x);
void	wanderer();
void	waste_time();
void	wear();
void	whatis(bool insist, int type);
void	wield();

bool	chase(THING *tp, coord *ee);
bool	diag_ok(coord *sp, coord *ep);
bool	dropcheck(THING *obj);
bool	fallpos(coord *pos, coord *newpos);
bool	find_floor(struct room *rp, coord *cp, int limit, bool monst);
bool	is_magic(THING *obj);
bool    is_symlink(char *sp); 
bool	levit_check();
bool	pack_room(bool from_floor, THING *obj);
bool	roll_em(THING *thatt, THING *thdef, THING *weap, bool hurl);
bool	see_monst(THING *mp);
bool	seen_stairs();
bool	turn_ok(int y, int x);
bool	turn_see(bool turn_off);
bool	is_current(THING *obj);
int	passwd();

char	be_trapped(coord *tc);
char	floor_ch();
char	pack_char();
char	readchar();
char	rnd_thing();

char	*charge_str(THING *obj);
char	*choose_str(char *ts, char *ns);
char	*inv_name(THING *obj, bool drop);
char	*nullstr(THING *ignored);
char	*num(int n1, int n2, char type);
char	*ring_num(THING *obj);
char	*set_mname(THING *tp);
char	*vowelstr(char *str);

int	get_bool(void *vp, WINDOW *win);
int	get_inv_t(void *vp, WINDOW *win);
int	get_num(void *vp, WINDOW *win);
int	get_sf(void *vp, WINDOW *win);
int	get_str(void *vopt, WINDOW *win);
int	trip_ch(int y, int x, int ch);

coord	*find_dest(THING *tp);
coord	*rndmove(THING *who);

THING	*find_obj(int y, int x);
THING	*get_item(char *purpose, int type);
THING	*leave_pack(THING *obj, bool newobj, bool all);
THING	*new_item();
THING	*new_thing();
*/

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
    d_type: i32,
    d_func: fn(),
    d_arg: i32,
    d_time: i32,
}

//pub static d_list: [DelayedAction; MAXDAEMONS] = [DelayedAction { d_type: 0, d_func: fn(), d_arg: 0, d_time: 0 }; MAXDAEMONS];

/*typedef struct {
    char	*st_name;
    int		st_value;
} STONE;*/
pub struct Stone {
    st_name: String,
    st_value: i32,
}

//extern coord    nh;
//extern char     *rainbow[];
//extern int      cNCOLORS;
//extern STONE    stones[];
//extern int      cNSTONES;
//extern char     *wood[];
//extern int      cNWOOD;
//extern char     *metal[];
//extern int      cNMETAL;
