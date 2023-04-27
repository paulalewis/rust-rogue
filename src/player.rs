use std::char::MAX;

use crate::{rogue::Thing, constants::MAX_PACK_SIZE};

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
        Player {
            cur_armor: None,
            cur_ring: [None, None],
            cur_weapon: None,
            food_left: 0,
            hungry_state: 0,
            inpack: 0,
            pack_used: [false; MAX_PACK_SIZE],
            player_stats: None,
            purse: 0,
        }
    }
}