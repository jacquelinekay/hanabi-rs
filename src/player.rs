use std::io;
use std::str::FromStr;

use super::config::GameConfig;

use super::state::State;
use super::types::Action;
use super::types::HintType;

pub trait Player {
    fn get_command(&self, player_id: usize, config: &GameConfig) -> Action;

    // Update internal player state based on game client state.
    fn state_update(&self, state: &State);
}

fn read_command() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match String::from_str(input.as_str().trim()) {
                Ok(n) => n,
                Err(error) => panic!("Failed to read from io::stdin with: {}", error),
            }
        }
        Err(error) => panic!("Failed to read from io::stdin with: {}", error),
    }
}

pub struct CommandLinePlayer;

impl Player for CommandLinePlayer {
    fn get_command(&self, player_id: usize, config: &GameConfig) -> Action {
        println!("Choose an action: (p)lay a card, (d)iscard a card, or (h)int a player.");
        match read_command().as_str() {
            "p" => {
                println!("Enter the index of the card you want to play.");
                let index = read_command().parse::<usize>().unwrap();
                Action::Play { index: index }
            }
            "d" => {
                println!("Enter the index of the card you want to discard.");
                let index = read_command().parse::<usize>().unwrap();
                Action::Discard { index: index }
            }
            "h" => {
                println!("Enter the ID of the player you want to hint.");
                let id = read_command().parse::<usize>().unwrap();
                if id == player_id {
                    panic!("You can't hint yourself!")
                }
                println!("What type of hint do you want to give?");
                println!("Enter (c)olor or (n)umber.");
                match read_command().as_str() {
                    "c" => {
                        println!("Enter the color you want to hint:");
                        println!("(w)hite, (y)ellow, (g)reen, (r)ed, (b)lue");
                        let suite = config
                            .name_suite_map
                            .get(&read_command().as_str().chars().nth(0).unwrap())
                            .unwrap();
                        Action::Hint {
                            receiver_id: id,
                            hint: HintType::SuiteType(*suite),
                        }
                    }
                    "n" => {
                        println!("Enter the number you want to hint:");
                        let index = read_command().parse::<usize>().unwrap();
                        Action::Hint {
                            receiver_id: id,
                            hint: HintType::Number(index),
                        }
                    }
                    _ => panic!("Received invalid input"),
                }
            }
            _ => panic!("Received invalid input"),
        }
    }

    fn state_update(&self, state: &State) {
        // All necessary state is in the client.
    }
}

// TODO: Observe the state
pub struct NaiveAIPlayer;

impl Player for NaiveAIPlayer {
    fn get_command(&self, player_id: usize, config: &GameConfig) -> Action {
        // TODO
        Action::Play { index: 0 }
    }

    fn state_update(&self, state: &State) {
        // TODO
    }
}

pub struct NetworkPlayer;

impl Player for NetworkPlayer {
    //
    fn get_command(&self, player_id: usize, config: &GameConfig) -> Action {
        // TODO
        Action::Play { index: 0 }
    }
    fn state_update(&self, state: &State) {
        // TODO
    }
}
