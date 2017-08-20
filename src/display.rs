extern crate itertools;

use self::itertools::Itertools;
use std::collections::HashMap;

use super::state::State;
use super::types::{Action, Card, HintType, PlayerHand, PlayerType, Status, Suite};

pub trait Display {
    fn lost(&self, reason: &str, score: usize);

    fn won(&self, score: usize);

    fn fuse(&self, n_fuses: usize);

    fn discard(&self, player_id: usize, card: &Card);

    fn draw(&self, player_id: usize, card: &Card);

    fn play(&self, player_id: usize, card: &Card);

    fn hint_suite(&self, player_id: usize, suite: Suite, matched: &Vec<usize>);

    fn hint_number(&self, player_id: usize, number: usize, matched: &Vec<usize>);

    fn show_state<T: Display>(&self, state: &State<T>);
}

pub struct CommandLineDisplay {
    pub suite_name_map: HashMap<Suite, char>
}

impl Display for CommandLineDisplay {
    fn lost(&self, reason: &str, score: usize) {
        println!("You lose! Reason: {}", reason);
        println!("Game over! Final score: {}", score);
    }

    fn won(&self, score: usize) {
        println!("You Won!! Great job! Final score: {}", score);
    }

    fn fuse(&self, n_fuses: usize) {
        println!("Invalid move: fuse blown!");
        println!("You only have {} fuses left.", n_fuses);
    }

    fn discard(&self, player_id: usize, card: &Card) {
        println!("Player {} discarded: {}{}",
                 player_id,
                 self.suite_name_map.get(&card.suite).unwrap(),
                 card.value);
    }

    fn draw(&self, player_id: usize, card: &Card) {
        println!("Player {} drew: {}{}",
                 player_id,
                 self.suite_name_map.get(&card.suite).unwrap(),
                 card.value);
    }

    fn play(&self, player_id: usize, card: &Card) {
        println!("Player {} played: {}{}",
                 player_id,
                 self.suite_name_map.get(&card.suite).unwrap(),
                 card.value);
    }


    fn hint_suite(&self, player_id: usize, suite: Suite, matched: &Vec<usize>) {
        println!("Player {} has {} {} cards at indices: {}.",
                 player_id,
                 matched.len(),
                 self.suite_name_map.get(&suite).unwrap(),
                 matched.iter().format(", "));
    }

    fn hint_number(&self, player_id: usize, number: usize, matched: &Vec<usize>) {
        println!("Player {} has {} cards numbered {}: {}",
                 player_id,
                 matched.len(),
                 number,
                 matched.iter().format(", "));
    }

    fn show_state<T: Display>(&self, state: &State<T>) {
        // print!("{}[2J", 27 as char);
        let n_players = state.config.players.len();
        for player_id in 0..n_players {
            println!("Player {}:", player_id);
            if player_id == state.config.client_player {
                println!("\tHIDDEN");
                continue;
            }
            for ref card in state.player_hands.get(player_id).unwrap() {
                print!("\t");
                print!("{}{} ",
                       self.suite_name_map.get(&card.suite).unwrap(),
                       card.value);
                print!("\n");
            }
        }
        println!("Fuses: {}", state.fuses);
        println!("Information tokens: {}", state.information_tokens);
    }
}

pub struct GraphicalDisplay;

impl Display for GraphicalDisplay {
    fn lost(&self, reason: &str, score: usize) {
    }

    fn won(&self, score: usize) {
    }

    fn fuse(&self, n_fuses: usize) {
    }

    fn discard(&self, player_id: usize, card: &Card) {
    }

    fn draw(&self, player_id: usize, card: &Card) {
    }

    fn play(&self, player_id: usize, card: &Card) {
    }

    fn hint_suite(&self, player_id: usize, suite: Suite, matched: &Vec<usize>) {
    }

    fn hint_number(&self, player_id: usize, number: usize, matched: &Vec<usize>) {
    }

    fn show_state<T: Display>(&self, state: &State<T>) {
    }
}
