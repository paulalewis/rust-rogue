use std::collections::HashSet;

use abstract_game_engine::core::{reward::Reward, simulator::{LegalActions, Simulator}};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::core::direction::Direction;

use super::{constants::ISKNOW, new_level::new_level, object_type::{ObjectType, RingType}};
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
        let mut state = RogueState::new();
        new_level(&mut state);
        // start_daemon(runners, 0, AFTER);
        // start_daemon(doctor, 0, AFTER);
        // fuse(swander, 0, WANDERTIME, AFTER);
        // start_daemon(stomach, 0, AFTER);
        state
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
        actions.insert(RogueAction::Move(Direction::UpLeft));
        actions.insert(RogueAction::Move(Direction::UpRight));
        actions.insert(RogueAction::Move(Direction::DownLeft));
        actions.insert(RogueAction::Move(Direction::DownRight));
        actions.insert(RogueAction::PickUp);
        actions.insert(RogueAction::Search);
        actions.insert(RogueAction::Rest);
        actions.insert(RogueAction::StairsDown);
        actions.insert(RogueAction::StairsUp);
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
        for opt_object in pack {
            if opt_object.is_none() { continue; }
            let object = opt_object.unwrap();
            let object_type = object.object_type;
            let base_value = object_type.base_value() as isize;
            worth += match object_type {
                ObjectType::Amulet => base_value,
                ObjectType::Food(_) => base_value * object.count,
                ObjectType::Weapon(_) => {
                    // object.flags |= ISKNOW;, should not mutate state here
                    base_value * (3 * (object.hplus + object.dplus) + object.count)
                },
                ObjectType::Armor(armor_type) => {
                    // object.flags |= ISKNOW;, should not mutate state here
                    base_value + (9 - object.arm as isize) * 100 + (10 * (armor_type.armor_class() as isize - object.arm) as isize)
                },
                ObjectType::Scroll(scroll_type) => {
                    let know = state.known_scrolls[scroll_type as usize];
                    // state.known_scrolls[scroll_type as usize] = true;, should not mutate state here
                    base_value * if know { object.count } else { object.count / 2 }
                },
                ObjectType::Potion(potion_type) => {
                    let know = state.known_potions[potion_type as usize];
                    // state.known_potions[potion_type as usize] = true;, should not mutate state here
                    base_value * if know { object.count } else { object.count / 2 }
                },
                ObjectType::Ring(ring_type) => {
                    let know = state.known_rings[ring_type as usize];
                    // object.flags |= ISKNOW;, should not mutate state here
                    // state.known_rings[ring_type as usize] = true;, should not mutate state here
                    let value = if know { base_value } else { base_value / 2 };
                    match ring_type {
                        RingType::AddStrength |
                        RingType::Protection | 
                        RingType::Dexterity |
                        RingType::IncreaseDamage => {
                            if object.arm > 0 { value + object.arm as isize * 100 } else { 10 }
                        }
                        _ => value
                    }
                },
                ObjectType::Stick(stick_type) => {
                    let know = state.known_sticks[stick_type as usize];
                    // object.flags |= ISKNOW;, should not mutate state here
                    // state.known_sticks[stick_type as usize] = true;, should not mutate state here
                    base_value + if know {
                        20 * object.charges as isize
                    } else {
                        20 * object.charges as isize / 2
                    }
                },
                _ => 0,
            }
        }
        state.player.purse as isize + if worth < 0 { 0 } else { worth }
    }
}