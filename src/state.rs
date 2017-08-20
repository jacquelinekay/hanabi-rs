extern crate rand;
extern crate itertools;

use self::itertools::Itertools;
use self::rand::Rng;
use std::collections::HashMap;
use std::iter;
use super::config::GameConfig;
use super::display::Display;
use super::player::{Player, CommandLinePlayer, NaiveAIPlayer, NetworkPlayer};

use super::types::{Action, Card, HintType, PlayerHand, PlayerType, Status, Suite};

fn shuffle_and_deal(n_players: usize) -> (Vec<Card>, Vec<PlayerHand>) {
    let mut deck: Vec<Card> = Vec::new();

    for suite in Suite::iter_variants() {
        deck.extend(iter::repeat(1).take(4).map(|value| Card{suite, value}));

        for i in 2..5 {
            deck.extend(iter::repeat(i).take(3).map(|value| Card{suite, value}));
        }
        // TODO: wtf
        let value = 5;
        deck.push(Card{suite, value});
    }

    rand::thread_rng().shuffle(deck.as_mut_slice());

    let mut player_hands: Vec<PlayerHand> = Vec::new();
    let n_cards = if n_players < 4 { 5 } else { 4 };
    for _ in 0..n_players {
        let mut hand: PlayerHand = Vec::new();
        for _ in 0..n_cards {
            hand.push(deck.pop().unwrap());
        }
        player_hands.push(hand);
    }
    (deck, player_hands)
}

// Mutable game state
pub struct State<T: Display> {
    pub config: GameConfig,
    display: T,
    pub information_tokens: usize, // initialized to 8
    pub fuses: usize, // initialized to 3
    pub player_hands: Vec<PlayerHand>,
    played_cards: HashMap<Suite, Vec<usize>>,
    deck: Vec<Card>, // randomly shuffled, always has same contents
}

impl<T: Display> State<T> {
    pub fn new(config: GameConfig, display: T) -> State<T> {
        let n_players = config.players.len();
        let (deck, player_hands) = shuffle_and_deal(n_players);
        let played_cards = Suite::iter_variants()
            .zip(iter::repeat(Vec::new()).take(Suite::iter_variants().len()))
            .collect::<HashMap<Suite, Vec<usize>>>();
        State {
            // TODO: Consider randomizing player order?
            config: config,
            display: display,
            information_tokens: 8,
            fuses: 3,
            player_hands: player_hands,
            played_cards: played_cards,
            deck: deck,
        }
    }

    fn turn(&mut self, player_id: usize, action: Action) -> Status {
        // TODO: Is it valid to pass?
        match action {
            Action::Discard { index } => {
                let card = self.discard(player_id, index);
                self.display.discard(player_id, &card);

                if !self.deck.is_empty() {
                    // TODO: is it valid to discard if deck empty?
                    self.draw(player_id);
                    Status::InProgress
                } else {
                    self.display.lost("Ran out of cards!", self.score());
                    Status::Lost
                }
            }
            Action::Play { index } => {
                let card = self.discard(player_id, index);
                self.display.play(player_id, &card);
                if self.valid_to_play(&card) {
                    self.play_card(&card);

                    if self.played_cards.get(&card.suite).unwrap().len() == 5 {
                        if self.information_tokens < 8 {
                            self.information_tokens += 1;
                        }
                    }

                    self.draw(player_id);
                    // check win condition
                    for suite in Suite::iter_variants() {
                        if self.played_cards.get(&suite).unwrap().len() != 5 {
                            return Status::InProgress;
                        }
                    }
                    Status::Won
                } else {
                    self.fuses -= 1;
                    self.display.fuse(self.fuses);
                    if self.fuses == 0 {
                        self.display.lost("No more fuses!", self.score());
                        Status::Lost
                    } else {
                        Status::InProgress
                    }
                }
            }
            Action::Hint { receiver_id, hint } => {
                if self.information_tokens == 0 {
                    panic!("Can't give a hint, out of information tokens!");
                }
                self.information_tokens -= 1;
                self.give_hint(receiver_id, hint);
                Status::InProgress
            }
        }
    }

    fn play_card(&mut self, card: &Card) {
        // TODO Error handling for unwrap
        let suite = card.suite;
        let value = card.value;
        self.played_cards.get_mut(&suite).unwrap().push(value);
    }

    fn draw(&mut self, player_id: usize) {
        // TODO Error handling for unwrap
        let card = self.deck.pop().unwrap();
        if player_id != self.config.client_player {
            self.display.draw(player_id, &card);
        }
        self.player_hands
            .get_mut(player_id)
            .unwrap()
            .push(card);
    }

    fn discard(&mut self, player_id: usize, index: usize) -> Card {
        // TODO Error handling for unwrap
        let hand = self.player_hands.get_mut(player_id).unwrap();
        if index >= hand.len() {
            panic!("Can't remove index which doesn't exist!");
        }
        if hand.is_empty() {
            panic!("Can't discard from an empty hand!");
        }
        hand.remove(index)
    }

    fn valid_to_play(&self, card: &Card) -> bool {
        let suite = card.suite;
        let value = card.value;
        // TODO Error handling for unwrap
        if self.played_cards.get(&suite).unwrap().len() == 0 && value == 1 {
            true
        } else if self.played_cards.get(&suite).unwrap().len() > 0 {
            *self.played_cards.get(&suite).unwrap().last().unwrap() == value - 1
        } else {
            false
        }
    }

    // Display-specific
    fn give_hint(&self, receiver_id: usize, hint: HintType) {
        // TODO: cache hints
        match hint {
            HintType::SuiteType(hint_suite) => {
                let hand = self.player_hands.get(receiver_id).unwrap();
                let mut matched_suite: Vec<usize> = Vec::new();
                // TODO: does this destructuring work?
                for (i, ref card) in hand.iter().enumerate() {
                    let suite = card.suite;
                    if suite == hint_suite {
                        matched_suite.push(i);
                    }
                }
                self.display.hint_suite(receiver_id, hint_suite, &matched_suite);
            }
            HintType::Number(hint_value) => {
                let hand = self.player_hands.get(receiver_id).unwrap();
                let mut matched_value: Vec<usize> = Vec::new();
                for (i, ref card) in hand.iter().enumerate() {
                    let value = card.value;
                    if value == hint_value {
                        matched_value.push(i);
                    }
                }

                self.display.hint_number(receiver_id, hint_value, &matched_value);
            }
        }
    }

    /*
    fn check_end_conditions(&self) -> Status {
        if self.deck.is_empty() || self.fuses == 0 {
            Status::Lost
        } else {
            for suite in Suite::iter_variants() {
                // TODO Error handling for unwrap
                if self.played_cards.get(&suite).unwrap().len() != 5 {
                    return Status::InProgress
                }
            }
            Status::Won
        }
    }
    */

    fn score(&self) -> usize {
        self.played_cards.values().map(|v| v.len()).sum()
    }

    fn unwrap_player<'a>(&self, player_tag: &'a PlayerType) -> &'a Player {
        match *player_tag {
            PlayerType::CommandLine(ref player) => player,
            PlayerType::NaiveAI(ref player) => player,
            PlayerType::Network(ref player) => player
        }
    }

    fn get_action(&self, player_id: usize) -> Action {
        let player_tag = self.config.players.get(player_id).unwrap();
        let player = self.unwrap_player(player_tag);

        // TODO
        // player.state_update(&self);
        player.get_command(player_id, &self.config)
    }

    pub fn game_loop(&mut self) {
        let mut status = Status::InProgress;
        loop {
            match status {
                Status::InProgress => {
                    let n_players = self.config.players.len();
                    for player_id in 0..n_players {
                        self.display.show_state(self);
                        let action = self.get_action(player_id);
                        status = self.turn(player_id, action);
                    }
                }
                Status::Won => {
                    self.display.won(self.score());
                    break;
                }
                Status::Lost => {
                    break;
                }
            }
        }

    }
}
