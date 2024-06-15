use abstract_game_engine::core::simulator::Simulator;

use crate::core::{rogue_simulator::RogueSimulator, rogue_state::RogueState};

use super::{command::{process_command, CommandStatus}, console_screen::ConsoleScreen};

pub struct Game {
    simulator: RogueSimulator,
    current_state: RogueState,
    screen: ConsoleScreen,
}

impl Game {
    pub fn new() -> Game {
        let mut simulator = RogueSimulator::new();
        let current_state = simulator.generate_initial_state();
        Game { simulator, current_state, screen: ConsoleScreen::new() }
    }

    pub fn new_from_seed(seed: u64) -> Game {
        let mut simulator = RogueSimulator::seed_from_u64(seed);
        let current_state = simulator.generate_initial_state();
        Game { simulator, current_state, screen: ConsoleScreen::new() }
    }
    
    fn restore_game_from_file(filepath: String) -> RogueSimulator {
        dbg!(filepath);
        todo!();
    }

    pub fn new_from_file(filepath: String) -> Game {
        let mut simulator = Self::restore_game_from_file(filepath);
        let current_state = simulator.generate_initial_state();
        Game { simulator, current_state, screen: ConsoleScreen::new() }
    }

    pub fn play(&mut self) {
        loop {
            match process_command(&mut self.screen, &mut self.simulator, &self.current_state) {
                CommandStatus::Continue => continue,
                CommandStatus::Quit => break,
                CommandStatus::Win => todo!(),
                CommandStatus::Loss => todo!(),
                CommandStatus::Save => todo!(),
                CommandStatus::Error => todo!(),
                CommandStatus::StateChange(state) => self.current_state = state,
            }
        }
        /*
        while !simulator.is_terminal_state(&current_state) {
            // todo!("update screen");
            let legal_actions = simulator.calculate_legal_actions(&current_state);
            let action = agent.select_action(0, &current_state, &mut simulator);
            let mut actions = HashMap::new();
            actions.insert(0, action);
            current_state = simulator.state_transition(&current_state, &actions);
        }*/
    }
}