use std::vec::Vec;

use super::player::{CommandLinePlayer, NaiveAIPlayer};
use super::player_type_conversion;
use super::types::PlayerType;

// Properties of the game
#[derive(Serialize, Deserialize)]
pub struct GameConfig {
    pub client_player: usize,
    #[serde(with = "player_type_conversion")]
    pub players: Vec<PlayerType>,
}

pub fn default_game_config() -> GameConfig {
    let players = vec![PlayerType::CommandLine(CommandLinePlayer),
                       PlayerType::NaiveAI(NaiveAIPlayer),
                       PlayerType::NaiveAI(NaiveAIPlayer)];

    GameConfig {
        client_player: 0,
        players,
    }
}
