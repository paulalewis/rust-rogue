
pub const ADVENTURER_NAME: &str = "Rustacean"; //whoami
pub const FRUIT: &str = "slime-mold";
pub const PRESS_SPACE_TO_CONTINUE: &str = "--Press space to continue--";
pub const RELEASE: &str = "5.4.4";

pub const MAXSTR: usize = 1024; // maximum length of strings
pub const MAXLINES: usize = 32; // maximum number of screen lines used 
pub const MAXCOLS: usize = 80; // maximum number of screen columns used
pub const MAXDAEMONS: usize = 20;
pub const MAXROOMS: usize = 9;
pub const MAXTHINGS: usize = 9;
pub const MAXOBJ: usize = 9;
pub const MAXPACK: usize = 23;
pub const MAXTRAPS: usize = 10;
pub const AMULETLEVEL: usize = 26;
pub const NUMTHINGS: usize = 7; // number of types of things
pub const MAXPASS: usize = 13; // upper limit on number of passages
pub const NUMLINES: usize = 24;
pub const NUMCOLS: usize = 80;
pub const STATLINE: usize = NUMLINES - 1;
pub const BORE_LEVEL: usize = 50;

// *** flags for rooms ***

pub const ISDARK: usize = 1; // ISDARK room is dark
pub const ISGONE: usize = 2; // ISGONE room is gone (a corridor)
pub const ISMAZE: usize = 4; // ISMAZE room is a maze

// *** flags for objects ***

pub const ISCURSED: usize = 1; // ISCURSED object is cursed
pub const ISKNOW: usize = 2; // ISKNOW player knows details about the object
pub const ISMISL: usize = 4; // ISMISL object is a missile type
pub const ISMANY: usize = 10; // ISMANY object comes in groups
/*	ISFOUND 0000020		...is used for both objects and creatures */
pub const ISPROT: usize = 40; // ISPROT armor is permanently protected

// *** flags for creatures ***

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

// *** Flags for level map ***
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

// *** Trap types ***

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

// *** Potion types ***

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

// *** Scroll types ***

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

// *** Weapon types ***

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

// *** Armor types ***

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

// *** Ring types ***

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

// *** Rod/Wand/Staff types ***

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

// *** Save against things ***

pub const VS_POISON: usize = 0;
pub const VS_MAGIC: usize = 3;

// *** game object characters ***

pub const PASSAGE: char = '#';
pub const DOOR: char = '+';
pub const FLOOR: char = '.';
pub const PLAYER: char = '@';
pub const TRAP: char = '^';
pub const STAIRS: char = '%';
pub const GOLD: char = '*';
pub const POTION: char = '!';
pub const SCROLL: char = '?';
pub const MAGIC: char = '$';
pub const FOOD: char = ':';
pub const WEAPON: char = ')';
pub const ARMOR: char = ']';
pub const AMULET: char = ',';
pub const RING: char = '=';
pub const STICK: char = '/';
pub const CALLABLE: isize = -1;
pub const R_OR_S: isize = -2;

