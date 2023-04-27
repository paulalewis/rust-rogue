use crate::{init::{init_scroll_names, init_potion_colors}, player::Player};

// This represents the state of the game.
// It can be used to save and restore a game.
#[derive(Debug, Clone, PartialEq)]
pub struct RogueState {
    // amulet player found the amulet
    pub amulet: bool,
    // level what level the player is on
    pub level: usize,
    //no_food number of levels without food 
    pub no_food: usize,
    //ntraps number of traps on this level
    pub n_traps: usize,
    // max_level deepest player has gone 
    pub max_level: usize,
    pub player: Player,
    // p_colors colors of the potions
    pub potion_colors: Vec<String>,
    // quiet number of quiet turns 
    pub quiet: usize,
    // s_names names of the scrolls
    pub scroll_names: Vec<String>,
    //seed random number seed
    pub seed: u64,
}

impl RogueState {
    pub fn new(seed: u64) -> RogueState {
        RogueState {
            amulet: false,
            level: 1,
            no_food: 0,
            n_traps: 0,
            max_level: 1,
            player: Player::new(),
            potion_colors: init_potion_colors(),
            quiet: 0,
            scroll_names: init_scroll_names(),
            seed,
        }
    }
}
