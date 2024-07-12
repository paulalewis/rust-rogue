use super::constants::{AMULET, ARMOR, ARMOR_CLASS, ISPROT, POTION, RING, SCROLL, STICK, WEAPON};
use super::coord::Coord;

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
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Object {
    // type
    pub object_type: char,
    pub pos: Coord,
    pub text: String,
    pub launch: i32,
    pub packch: char,
    pub damage: String,
    pub hurldmg: String,
    pub count: isize,
    pub which: usize,
    pub hplus: isize,
    pub dplus: isize,
    pub arm: usize,
    pub flags: usize,
    pub group: i32,
    pub label: String,
    pub charges: usize,
}

impl Object {
    pub fn new() -> Self {
        Object {
            object_type: '\0',
            pos: Default::default(),
            text: String::new(),
            launch: 0,
            packch: '\0',
            damage: String::new(),
            hurldmg: String::new(),
            count: 0,
            which: 0,
            hplus: 0,
            dplus: 0,
            arm: 0,
            flags: 0,
            group: 0,
            label: String::new(),
            charges: 0,
        }
    }

    // on()
    pub fn on(&self, flag: usize) -> bool {
	    self.flags & flag != 0
    }

    // Returns true if an object radiates magic
    // is_magic(THING *obj)
    pub fn is_magic(&self) -> bool {
        match self.object_type {
            ARMOR => self.on(ISPROT) || self.arm != ARMOR_CLASS[self.which],
            WEAPON => self.hplus != 0 || self.dplus != 0,
            POTION | SCROLL | STICK | RING | AMULET => true,
            _ => false,
        }
    }
}