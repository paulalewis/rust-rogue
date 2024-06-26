use super::rogue_state::RogueState;

pub struct Status {
    pub level: usize,
    pub gold: usize,
    pub health: (usize, usize),
    pub strength: (usize, usize),
    pub armor: isize,
    pub experience: (usize, usize),
}

impl Status {
    pub fn from_rogue_state(state: &RogueState) -> Self {
        Status {
            level: state.dungeon.level,
            gold: state.player.purse,
            health: (state.player.player_stats.stats.hpt, state.player.player_stats.stats.max_hp),
            strength: (state.player.player_stats.stats.str, state.player.player_stats.stats.str),
            armor: state.player.player_stats.stats.arm,
            experience: (state.player.player_stats.stats.lvl, state.player.player_stats.stats.exp),
        }
    }
}