extern crate hanabi;

use std::collections::HashMap;

use hanabi::config::GameConfig;
use hanabi::display::CommandLineDisplay;
use hanabi::player::{CommandLinePlayer, NaiveAIPlayer};
use hanabi::state::State;
use hanabi::types::{PlayerType, Suite};

fn main() {
    // TODO: Convenience initializer, read from file
    let players = vec![
        PlayerType::CommandLine(CommandLinePlayer),
        PlayerType::NaiveAI(NaiveAIPlayer),
        PlayerType::NaiveAI(NaiveAIPlayer)];

    // TODO this suite_name_map stuff is kinda terrible
    let suite_name_map = Suite::iter_variants()
                .zip(Suite::iter_variant_names().map(|n| n.chars().nth(0)
                .unwrap())).collect::<HashMap<Suite, char>>();

    let config = GameConfig::new(0, players);
    let display = CommandLineDisplay{suite_name_map};
    let mut game = State::new(config, display);
    game.game_loop();
}
