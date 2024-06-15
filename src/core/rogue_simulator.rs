use abstract_game_engine::core::simulator::Simulator;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use super::{rogue_action::RogueAction, rogue_state::RogueState};

pub struct RogueSimulator {
    seed: u64,
    rng: ChaCha8Rng,
}

impl RogueSimulator {
    pub fn new() -> Self {
        let mut rng = ChaCha8Rng::from_entropy();
        RogueSimulator::seed_from_u64(rng.next_u64())
    }
    
    pub fn seed_from_u64(seed: u64) -> Self {
        let rng = ChaCha8Rng::seed_from_u64(seed);
        RogueSimulator { seed, rng }
    }
}

impl Simulator<RogueState, RogueAction> for RogueSimulator {
    fn generate_initial_state(&mut self) -> RogueState {
        let mut rogue_state = RogueState::new();
        // new_level(&mut rogue_state);
        // start_daemon(runners, 0, AFTER);
        // start_daemon(doctor, 0, AFTER);
        // fuse(swander, 0, WANDERTIME, AFTER);
        // start_daemon(stomach, 0, AFTER);
        rogue_state
    }

    fn calculate_rewards(&mut self, state: &RogueState) -> Vec<abstract_game_engine::core::reward::Reward> {
        todo!()
    }

    fn calculate_legal_actions(&mut self, state: &RogueState) -> Vec<abstract_game_engine::core::simulator::LegalActions<RogueAction>> {
        todo!()
    }

    fn state_transition(&mut self, state: &RogueState, actions: &Vec<Option<RogueAction>>) -> RogueState {
        todo!()
    }

    fn number_of_players(&mut self) -> usize {
        1
    }
}