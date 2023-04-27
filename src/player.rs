use std::char::MAX;

use crate::{rogue::{Thing, rainbow, NCOLORS}, constants::{MAX_PACK_SIZE, ISHALU, R_PROTECT, VS_MAGIC, LEFT, RIGHT, HUNGERTIME}, utils::{on, rnd}, io::msg, monsters::save_throw};

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    // cur_armor what player is wearing
    pub cur_armor: Option<Thing>,
    // cur_ring which rings are being worn
    pub cur_ring: [Option<Thing>; 2],
    // cur_weapon which weapon he is weilding
    pub cur_weapon: Option<Thing>,
    // food_left amount of food in hero's stomach
    pub food_left: usize,
    //hungry_state how hungry is he 
    pub hungry_state: usize,
    // inpack number of things in pack 
    pub inpack: usize,
    // pack_used Is the character used in the pack?
    pub pack_used: [bool; MAX_PACK_SIZE], 
    // player the player stats
    pub player_stats: Option<Thing>,
    // purse how much gold he has 
    pub purse: usize,
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
        Player {
            cur_armor: None,
            cur_ring: [None, None],
            cur_weapon: None,
            food_left: HUNGERTIME,
            hungry_state: 0,
            inpack: 0,
            pack_used: [false; MAX_PACK_SIZE],
            player_stats: None,
            purse: 0,
        }
    }
    
    // If he is halucinating, pick a random color name and return it,
    // otherwise return the given color.
    pub fn pick_color<'a>(&self, color: &'a str) -> &'a str {
        return if on(self.player_stats.as_ref().unwrap(), ISHALU) { rainbow[rnd(NCOLORS)] } else { color };
    }

    //#define ISRING(h,r)
    pub fn is_ring(&self, h: usize, r: usize) -> bool {
	    let cur_ring = self.cur_ring.as_ref();
        match cur_ring[h] {
            Some(ref ring) => {
                match ring {
                    Thing::Object { which, .. } => {
                        *which == r
                    },
                    _ => false
                }
            },
            None => false
        }
    }

    //#define ISWEARING(r)
    fn is_wearing(&self, r: usize) -> bool {
	    self.is_ring(0, r) || self.is_ring(1, r)
    }

    // See if the object is one of the currently used items
    // bool is_current(THING *obj)
    pub fn is_current(&self, object: &Thing) -> bool {
	    let cur_armor = self.cur_armor.as_ref().unwrap();
	    let cur_ring = self.cur_ring.as_ref();
	    let cur_weapon = self.cur_weapon.as_ref().unwrap();
	    if object == cur_armor || object == cur_weapon || object == cur_ring[0].as_ref().unwrap() || object == cur_ring[1].as_ref().unwrap() {
		    msg("That's already in use");
		    true
	    } else {
		    false
	    }
    }

    // Choose the first or second string depending on whether it the player is tripping
    // choose_str
    pub fn choose_str<'a>(&self, ts: &'a str, ns: &'a str) -> &'a str {
	    if on(&self.player_stats.as_ref().unwrap(), ISHALU) { ts } else { ns }
    }

    // See if he saves against various nasty things
    //int save(int which)
    pub fn save(&self, mut which: usize) -> bool {
        if which == VS_MAGIC {
            which -= self.adjust_saving_throw(LEFT);
            which -= self.adjust_saving_throw(RIGHT);
        }
        match &self.player_stats {
            Some(player_stats) => {
                save_throw(which, player_stats)
            },
            None => panic!("save: player is None"),
        }
    }

    fn adjust_saving_throw(&self, side: usize) -> usize {
        let cur_ring = self.cur_ring.as_ref();
        if self.is_ring(side, R_PROTECT) {
            match &cur_ring[side] {
                Some(ring) => {
                    match ring {
                        Thing::Object { arm, .. } => *arm as usize,
                        _ => 0,
                    }
                },
                None => 0,
            }
        } else {
            0
        }
    }
}
