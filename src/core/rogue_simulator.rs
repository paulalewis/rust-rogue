use std::collections::HashSet;

use abstract_game_engine::core::{reward::Reward, simulator::{LegalActions, Simulator}};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::core::direction::Direction;

use super::constants::{AMULET, ARMOR, FOOD, POTION, RING, SCROLL, STICK, WEAPON, ARMOR_CLASS};
use super::{object_info::{ARMOR_INFO, POTION_INFO, RING_INFO, SCROLL_INFO, STICK_INFO, WEAPON_INFO}, rogue_action::RogueAction, rogue_state::RogueState};

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

    fn calculate_rewards(&mut self, state: &RogueState) -> Vec<Reward> {
        let reward = if self.is_terminal_state(state) {
            // todo also check if player is winner
            Self::calc_winner_rewards(state)
        } else {
            state.player.purse as isize
        };
        vec![Reward(reward)]
    }

    fn calculate_legal_actions(&mut self, state: &RogueState) -> Vec<LegalActions<RogueAction>> {
        let mut actions: HashSet<RogueAction> = HashSet::new();
        actions.insert(RogueAction::Move(Direction::Left));
        actions.insert(RogueAction::Move(Direction::Right));
        actions.insert(RogueAction::Move(Direction::Up));
        actions.insert(RogueAction::Move(Direction::Down));
        vec![LegalActions(actions)]
    }

    fn state_transition(&mut self, state: &RogueState, actions: &Vec<Option<RogueAction>>) -> RogueState {
        state.clone()
    }

    fn number_of_players(&mut self) -> usize {
        1
    }
}

impl RogueSimulator {
    fn calc_winner_rewards(state: &RogueState) -> isize {
        let mut worth = 0isize;
        let pack = &state.player.player_stats.pack;
        for object in pack {
            worth += match object.object_type {
                FOOD => 2 * object.count,
                WEAPON => {
                    WEAPON_INFO[object.which].worth as isize * (3 * (object.hplus + object.dplus) + object.count)
                    // obj->o_flags |= ISKNOW;
                },
                ARMOR => {
                    ARMOR_INFO[object.which].worth as isize + (9 - object.arm as isize) * 100 + (10 * (ARMOR_CLASS[object.which] - object.arm) as isize)
                    // obj->o_flags |= ISKNOW;
                },
                SCROLL => {
                    let know = SCROLL_INFO[object.which].know;
                    if know {
                        SCROLL_INFO[object.which].worth as isize * object.count
                    } else {
                        SCROLL_INFO[object.which].worth as isize * object.count / 2
                    }
                    // op->oi_know = TRUE;
                },
                POTION => {
                    let know = SCROLL_INFO[object.which].know;
                    if know {
                        POTION_INFO[object.which].worth as isize * object.count
                    } else {
                        POTION_INFO[object.which].worth as isize * object.count / 2
                    }
                    //op->oi_know = TRUE;
                },
                RING => {
                    let know = RING_INFO[object.which].know;
                    if know {
                        RING_INFO[object.which].worth as isize
                    } else {
                        RING_INFO[object.which].worth as isize / 2
                    }
                    //if (obj->o_which == R_ADDSTR || obj->o_which == R_ADDDAM ||
                    //    obj->o_which == R_PROTECT || obj->o_which == R_ADDHIT)
                    //{
                    //    if (obj->o_arm > 0)
                    //        worth += obj->o_arm * 100;
                    //    else
                    //        worth = 10;
                    //}
                    //obj->o_flags |= ISKNOW;
                    //op->oi_know = TRUE;
                },
                STICK => {
                    let know = RING_INFO[object.which].know;
                    if know {
                        (STICK_INFO[object.which].worth + 20 * object.charges) as isize
                    } else {
                        (STICK_INFO[object.which].worth + 20 * object.charges / 2) as isize
                    }
                    //obj->o_flags |= ISKNOW;
                    //op->oi_know = TRUE;
                },
                AMULET => 1000,
                _ => 0,
            }
        }
        state.player.purse as isize + if worth < 0 { 0 } else { worth }
    }
}