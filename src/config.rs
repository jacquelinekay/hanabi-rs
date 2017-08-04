use std::collections::HashMap;
use super::types::Suite;

// Properties of the game
pub struct GameConfig {
    pub n_players: usize, // could be runtime-determined. 2-5
    pub client_player: usize,
    pub suite_name_map: HashMap<Suite, char>,
    pub name_suite_map: HashMap<char, Suite>,
}

impl GameConfig {
    pub fn new(n_players: usize, client_player: usize) -> GameConfig {
        GameConfig {
            n_players: n_players,
            client_player: client_player,
            suite_name_map: Suite::iter_variants()
                .zip(Suite::iter_variant_names().map(|n| n.chars().nth(0).unwrap()))
                .collect::<HashMap<Suite, char>>(),
            name_suite_map: Suite::iter_variant_names()
                .map(|n| n.to_lowercase().chars().nth(0).unwrap())
                .zip(Suite::iter_variants())
                .collect::<HashMap<char, Suite>>(),
        }
    }
}
