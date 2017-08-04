extern crate hanabi;

use hanabi::config::GameConfig;
use hanabi::state::State;

fn main() {
    let config = GameConfig::new(3, 0);
    let mut game = State::new(config);
    game.game_loop();
}
