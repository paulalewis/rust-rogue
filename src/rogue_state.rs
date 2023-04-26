use crate::{init::init_scroll_names, player::Player};

// This represents the state of the game.
// It can be used to save and restore a game.
#[derive(Debug, Clone, PartialEq)]
pub struct RogueState {
    // amulet player found the amulet
    pub amulet: bool,
    // level what level the player is on
    pub level: usize,
    pub player: Player,
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
            player: Player::new(),
            scroll_names: init_scroll_names(),
            seed,
        }
    }
}
