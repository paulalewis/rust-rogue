use crate::core::stats::Stats;

use super::{coord::Coord, object::Object, room::Room};

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
pub struct Creature {
    pub next: Box<Option<Creature>>,
    pub prev: Box<Option<Creature>>,
    pub pos: Coord,
    pub turn: bool,
    // type
    pub creature_type: char,
    pub disguise: char,
    pub oldch: char,
    pub dest: Option<Coord>,
    pub flags: usize,
    pub stats: Stats,
    pub room: Box<Room>,
    pub pack: Vec<Object>,
}

impl Creature {
    pub fn new() -> Self {
        Creature {
            next: Box::new(None),
            prev: Box::new(None),
            pos: Default::default(),
            turn: false,
            creature_type: '\0',
            disguise: '\0',
            oldch: '\0',
            dest: None,
            flags: 0,
            stats: Stats::new(),
            room: Box::new(Room::new()),
            pack: Vec::new(),
        }
    }

    //#define on(thing,flag)
    pub fn on(&self, flag: usize) -> bool {
	    self.flags & flag != 0
    }
}