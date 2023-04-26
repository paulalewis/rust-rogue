use crate::rogue::*;

// This represents the state of the game.
// It can be used to save and restore a game.
#[derive(Debug, Clone, PartialEq)]
pub struct RogueState {
    // amulet player found the amulet
    pub amulet: bool,
    // cur_armor what player is wearing
    pub cur_armor: Option<Thing>,
    // cur_ring which rings are being worn
    pub cur_ring: [Option<Thing>; 2],
    // cur_weapon which weapon he is weilding
    pub cur_weapon: Option<Thing>,
    // level what level the player is on
    pub level: usize,
    // player the player stats
    pub player: Option<Thing>,
}

impl RogueState {
    pub fn new() -> RogueState {
        RogueState {
            amulet: false,
            cur_armor: None,
            cur_ring: [None, None],
            cur_weapon: None,
            level: 1,
            player: None,
        }
    }
}
