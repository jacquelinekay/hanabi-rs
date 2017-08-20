use std::collections::HashMap;
use std::vec::Vec;
use std::boxed::Box;

use super::types::{PlayerType, Suite};

// Properties of the game
pub struct GameConfig {
    pub client_player: usize,
    pub players: Vec<PlayerType>,
    // TODO: not general enough for config.
    pub name_suite_map: HashMap<char, Suite>,
}

impl GameConfig {
    pub fn new(client_player: usize, players: Vec<PlayerType>) -> GameConfig {
        GameConfig {
            client_player: client_player,
            players: players,
            name_suite_map: Suite::iter_variant_names()
                .map(|n| n.to_lowercase().chars().nth(0).unwrap())
                .zip(Suite::iter_variants())
                .collect::<HashMap<char, Suite>>(),
        }
    }
}
