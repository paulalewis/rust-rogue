use crate::core::coord::Coord;

use crate::core::creature::Creature;
use crate::core::object::Object;

use super::constants::{HUNGERTIME, ISHALU, LEFT, MAX_PACK_SIZE, RAINBOW, RIGHT, VS_MAGIC, XP_LEVELS};
use super::monster::save_throw;
use super::object_type::{ObjectType, RingType};
use super::rogue_message::RogueMessage;
use super::rogue_state::RogueState;
use super::room::Room;
use super::stats::{Attack, DmgStats, Stats};
use super::utils::{rnd, roll};

// #define INIT_STATS { 16, 0, 1, 10, 12, "1x4", 12 }
// struct stats max_stats = INIT_STATS;	/* The maximum for the player */
// const MAX_INITIAL_STR: usize = 16;
// pub static max_str: usize = MAX_INITIAL_STR;
const INIT_STATS: Stats = Stats {
    str: 16,
    exp: 0,
    lvl: 1,
    arm: 10,
    hpt: 12,
    dmg: DmgStats::Single(Attack(1, 4)),
    max_hp: 12,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Player {
    // cur_armor what player is wearing
    pub cur_armor: Option<Object>,
    // cur_ring which rings are being worn
    pub cur_ring: [Option<Object>; 2],
    // cur_weapon which weapon he is weilding
    pub cur_weapon: Option<Object>,
    // food_left amount of food in hero's stomach
    pub food_left: usize,
    //hungry_state how hungry is he 
    pub hungry_state: usize,
    // inpack number of things in pack 
    pub inpack: usize,
    // pack_used Is the character used in the pack?
    pub pack_used: [bool; MAX_PACK_SIZE], 
    // player the player stats
    pub player_stats: Creature,
    // purse how much gold he has 
    pub purse: usize,
    // oldpos position before last look() call
    pub oldpos: Coord,
    // oldrp roomin(&oldpos)
    pub oldrp: Option<Room>,
}

impl Player {
    pub fn new() -> Player {
        /*void init_player() {
            register THING *obj;

            pstats = max_stats;
            // Give him some food
            obj = new_item();
            obj->o_type = FOOD;
            obj->o_count = 1;
            add_pack(obj, TRUE);
            // And his suit of armor
            obj = new_item();
            obj->o_type = ARMOR;
            obj->o_which = RING_MAIL;
            obj->o_arm = a_class[RING_MAIL] - 1;
            obj->o_flags |= ISKNOW;
            obj->o_count = 1;
            cur_armor = obj;
            add_pack(obj, TRUE);
            // Give him his weaponry.  First a mace.
            obj = new_item();
            init_weapon(obj, MACE);
            obj->o_hplus = 1;
            obj->o_dplus = 1;
            obj->o_flags |= ISKNOW;
            add_pack(obj, TRUE);
            cur_weapon = obj;
            // Now a +1 bow
            obj = new_item();
            init_weapon(obj, BOW);
            obj->o_hplus = 1;
            obj->o_flags |= ISKNOW;
            add_pack(obj, TRUE);
            // Now some arrows
            obj = new_item();
            init_weapon(obj, ARROW);
            obj->o_count = rnd(15) + 25;
            obj->o_flags |= ISKNOW;
            add_pack(obj, TRUE);
        }*/
        let mut player_stats = Creature::new();
        player_stats.stats = INIT_STATS;
        let pos = player_stats.pos;
        Player {
            cur_armor: None,
            cur_ring: [None, None],
            cur_weapon: None,
            food_left: HUNGERTIME,
            hungry_state: 0,
            inpack: 0,
            pack_used: [false; MAX_PACK_SIZE],
            player_stats: player_stats,
            purse: 0,
            //oldpos = hero;
            oldpos: pos,
            //oldrp = roomin(&hero);
            oldrp: None,
        }
    }
    
    // If he is halucinating, pick a random color name and return it,
    // otherwise return the given color.
    pub fn pick_color<'a>(&self, color: &'a str) -> &'a str {
        return if self.player_stats.on(ISHALU) { RAINBOW[rnd(RAINBOW.len())] } else { color };
    }

    //#define ISRING(h,r), not needed, just check if NONE

    //#define ISWEARING(r)
    fn is_wearing(&self, r: usize) -> bool {
        self.cur_ring[r] != None
    }

    // See if the object is one of the currently used items
    // bool is_current(THING *obj)
    pub fn is_current(&self, state: &mut RogueState, object: &Object) -> bool {
	    let cur_armor = self.cur_armor.as_ref().unwrap();
	    let cur_ring = self.cur_ring.as_ref();
	    let cur_weapon = self.cur_weapon.as_ref().unwrap();
	    if object == cur_armor || object == cur_weapon || object == cur_ring[0].as_ref().unwrap() || object == cur_ring[1].as_ref().unwrap() {
		    // screen.show_message("That's already in use");
            state.message = Some(RogueMessage::AlreadyInUse);
		    true
	    } else {
		    false
	    }
    }

    // Choose the first or second string depending on whether it the player is tripping
    // choose_str
    pub fn choose_str<'a>(&self, ts: &'a str, ns: &'a str) -> &'a str {
	    if self.player_stats.on(ISHALU) { ts } else { ns }
    }

    // See if he saves against various nasty things
    //int save(int which)
    pub fn save(&self, mut which: usize) -> bool {
        if which == VS_MAGIC {
            which -= self.adjust_saving_throw(LEFT);
            which -= self.adjust_saving_throw(RIGHT);
        }
        save_throw(which, &self.player_stats)
    }

    fn adjust_saving_throw(&self, side: usize) -> usize {
        match self.cur_ring[side] {
            Some(object) => {
                let object_type = object.object_type;
                match object_type {
                    ObjectType::Ring(RingType::Protection) => {
                        object.arm as usize
                    }
                    _ => 0
                }
            }
            None => 0,
        }
    }

    // Check to see if the guy has gone up a level.
    /*void check_level() {
        int i, add, olevel;

        for (i = 0; e_levels[i] != 0; i++)
	    if (e_levels[i] > pstats.s_exp)
	        break;
        i++;
        olevel = pstats.s_lvl;
        pstats.s_lvl = i;
        if (i > olevel)
        {
	    add = roll(i - olevel, 10);
	    max_hp += add;
	    pstats.s_hpt += add;
	    msg("welcome to level %d", i);
        }
    }*/
    pub fn check_level(&mut self, state: &mut RogueState) {
        let mut index = 0;
        for (i, xp) in XP_LEVELS.iter().enumerate() {
            if *xp > self.player_stats.stats.exp {
                index = i + 1;
                break;
            }
        }
        let old_level = self.player_stats.stats.lvl;
        self.player_stats.stats.lvl = index;
        if index > old_level {
            let add = roll(index - old_level, 10);
            self.player_stats.stats.max_hp += add;
            self.player_stats.stats.hpt += add;
            // screen.show_message(&format!("Welcome to level {}", index));
            state.message = Some(RogueMessage::WelcomeToLevel(index));
        }
    }
}
