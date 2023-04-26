use crate::rogue::*;

// This represents the state of the game.
// It can be used to save and restore a game.
#[derive(Debug, Clone, PartialEq)]
pub struct RogueState {
    //THING *cur_armor;			/* What he is wearing */
    pub cur_armor: Option<Thing>,
    //THING *cur_ring[2];			/* Which rings are being worn */
    pub cur_ring: [Option<Thing>; 2],
    //THING *cur_weapon;			/* Which weapon he is weilding */
    pub cur_weapon: Option<Thing>,
    //level what level the player is on
    pub level: usize,
    //THING player;				/* His stats */
    pub player: Option<Thing>,
}

impl RogueState {
    pub fn new() -> RogueState {
        RogueState {
            cur_armor: None,
            cur_ring: [None, None],
            cur_weapon: None,
            level: 1,
            player: None,
        }
    }
}
