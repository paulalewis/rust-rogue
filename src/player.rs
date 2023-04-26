use crate::rogue::Thing;

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    // cur_armor what player is wearing
    pub cur_armor: Option<Thing>,
    // cur_ring which rings are being worn
    pub cur_ring: [Option<Thing>; 2],
    // cur_weapon which weapon he is weilding
    pub cur_weapon: Option<Thing>,
    // player the player stats
    pub player_stats: Option<Thing>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            cur_armor: None,
            cur_ring: [None, None],
            cur_weapon: None,
            player_stats: None,
        }
    }
}