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
    // pack_used Is the character used in the pack?
    pub pack_used: [bool; MAX_PACK_SIZE], 
    // player the player stats
    pub player_stats: Option<Thing>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            cur_armor: None,
            cur_ring: [None, None],
            cur_weapon: None,
            pack_used: [false; MAX_PACK_SIZE],
            player_stats: None,
        }
    }
}