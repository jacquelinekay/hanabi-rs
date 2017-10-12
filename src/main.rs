extern crate hanabi;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use hanabi::config::{default_game_config, GameConfig};
use hanabi::display::CommandLineDisplay;
use hanabi::state::State;

fn main() {
    let config: GameConfig = env::args()
        .nth(1)
        .and_then(|filename| File::open(filename).ok())
        .and_then(|mut file| {
                      let mut data = String::new();
                      // TODO: check result or ignore warning
                      file.read_to_string(&mut data);
                      Some(data)
                  })
        .and_then(|data| serde_json::from_str(data.as_str()).ok())
        .unwrap_or_else(default_game_config);

    // Establish player connections in initialization of NetworkPlayer (?)
    // Need to determine if player is host or not.

    let display = CommandLineDisplay {};
    let mut game = State::new(config, display);
    game.game_loop();
}
