extern crate rand;

#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;
use rand::Rng;
use std::io;
use std::iter;
use std::str::FromStr;
use std::vec::Vec;

custom_derive! {
    // TODO: Clone and Copy are possible antipatterns
    #[derive(Clone, Copy, PartialEq, Eq, Hash,
             IterVariants(SuiteVariants), IterVariantNames(SuitVariantNames))]
    pub enum Suite {
        White,
        Yellow,
        Green,
        Blue,
        Red,
    }
}

enum HintType {
    SuiteType(Suite),
    Number(usize),
}

enum Action {
    Hint { receiver_id: usize, hint: HintType }, // Give a hint to player_id
    Discard { index: usize }, // Discard the card at index
    Play { index: usize }, // Play the card at index
}

enum Status {
    InProgress,
    Won,
    Lost,
}

// Cards have value between 1 and 5
struct Card(Suite, usize);

type PlayerHand = Vec<Card>;

fn shuffle_and_deal(n_players: usize) -> (Vec<Card>, Vec<PlayerHand>) {
    let mut deck: Vec<Card> = Vec::new();

    for suite in Suite::iter_variants() {
        deck.extend(iter::repeat(1).take(4).map(|v| Card(suite, v)));

        for i in 2..5 {
            deck.extend(iter::repeat(i).take(3).map(|v| Card(suite, v)));
        }
        deck.push(Card(suite, 5));
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

fn read_command() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match String::from_str(input.as_str().trim()) {
                Ok(n) => n,
                Err(error) => panic!("Failed to read from io::stdin with: {}", error)
            }
        }
        Err(error) => panic!("Failed to read from io::stdin with: {}", error)
    }
}

// The almighty god object
struct State {
    n_players: usize, // could be runtime-determined. 2-5
    client_player: usize,
    information_tokens: usize, // initialized to 8
    fuses: usize, // initialized to 3
    player_hands: Vec<PlayerHand>,
    played_cards: HashMap<Suite, Vec<usize>>,
    deck: Vec<Card>, // randomly shuffled, always has same contents
    suite_name_map: HashMap<Suite, char>,
    name_suite_map: HashMap<char, Suite>,
}

impl State {
    fn new(n_players: usize) -> State {
        let (deck, player_hands) = shuffle_and_deal(n_players);
        let played_cards = Suite::iter_variants()
            .zip(iter::repeat(Vec::new()).take(Suite::iter_variants().len()))
            .collect::<HashMap<Suite, Vec<usize>>>();
        State {
            // TODO: Consider randomizing player order
            n_players: n_players,
            client_player: 0,
            information_tokens: 8,
            fuses: 3,
            player_hands: player_hands,
            played_cards: played_cards,
            deck: deck,
            suite_name_map: Suite::iter_variants()
                .zip(Suite::iter_variant_names().map(|n| n.chars().nth(0).unwrap()))
                .collect::<HashMap<Suite, char>>(),
            name_suite_map: Suite::iter_variant_names()
                .map(|n| n.to_lowercase().chars().nth(0).unwrap())
                .zip(Suite::iter_variants())
                .collect::<HashMap<char, Suite>>(),
        }
    }

    fn turn(&mut self, player_id: usize, action: Action) -> Status {
        // TODO: Is it valid to pass?
        match action {
            Action::Discard { index } => {
                let Card(suite, value) = self.discard(player_id, index);
                println!("Player {} discarded: {}{}",
                         player_id,
                         self.suite_name_map.get(&suite).unwrap(),
                         value);

                // TODO: notify display on game events
                if !self.deck.is_empty() {
                    // TODO: is it valid to discard if deck empty?
                    self.draw(player_id);
                    Status::InProgress
                } else {
                    println!("Oh no! We ran out of cards!");
                    Status::Lost
                }
            }
            Action::Play { index } => {
                let Card(played_suite, value) = self.discard(player_id, index);
                println!("Player {} played: {}{}",
                         player_id,
                         self.suite_name_map.get(&played_suite).unwrap(),
                         value);
                if self.valid_to_play(&played_suite, value) {
                    self.play_card(&played_suite, value);

                    if self.played_cards.get(&played_suite).unwrap().len() == 5 {
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
                    // TODO: notify display
                } else {
                    println!("Invalid move: fuse blown!");
                    self.fuses -= 1;
                    if self.fuses == 0 {
                        println!("BOOM! You lose!");
                        Status::Lost
                    } else {
                        Status::InProgress
                    }
                    // TODO: notify display
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

    fn play_card(&mut self, suite: &Suite, value: usize) {
        // TODO Error handling for unwrap
        self.played_cards.get_mut(suite).unwrap().push(value);
    }

    fn draw(&mut self, player_id: usize) {
        // TODO Error handling for unwrap
        self.player_hands
            .get_mut(player_id)
            .unwrap()
            .push(self.deck.pop().unwrap());
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

    fn valid_to_play(&self, suite: &Suite, value: usize) -> bool {
        // TODO Error handling for unwrap
        if self.played_cards.get(suite).unwrap().len() == 0 && value == 1 {
            true
        } else if self.played_cards.get(suite).unwrap().len() > 0{
            *self.played_cards.get(suite).unwrap().last().unwrap() == value - 1
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
                let mut matched_suite : Vec<usize> = Vec::new();
                for (i, &Card(suite, _)) in hand.iter().enumerate() {
                    if suite == hint_suite {
                        matched_suite.push(i);
                    }
                }

                println!("Player {} has {} {} cards at indices: {}.",
                       receiver_id,
                       matched_suite.len(),
                       self.suite_name_map.get(&hint_suite).unwrap(),
                       matched_suite.iter().format(", ")
                       );
            }
            HintType::Number(hint_value) => {
                let hand = self.player_hands.get(receiver_id).unwrap();
                let mut matched_value : Vec<usize> = Vec::new();
                for (i, &Card(_, value)) in hand.iter().enumerate() {
                    if value == hint_value {
                        matched_value.push(i);
                    }
                }

                println!("Player {} has {} cards numbered {}: {}",
                       receiver_id,
                       matched_value.len(),
                       hint_value,
                       matched_value.iter().format(", "));
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

    // This is a customization point for human or AI players
    // TODO: duplicate validity checking in this function
    // TODO: Error handling, more state-machine-y
    fn get_player_action(&self, player_id: usize) -> Action {
        if player_id == self.client_player {
            // get input from stdin
            println!("Choose an action: (p)lay a card, (d)iscard a card, or (h)int a player.");
            match read_command().as_str() {
                "p" => {
                    println!("Enter the index of the card you want to play.");
                    /*
                    let mut input = String::new();
                    io::stdin().read_line(&mut input);
                    */
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
                            let suite = self.name_suite_map
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
                        _ => panic!("Received invalid input")
                    }
                }
                _ => panic!("Received invalid input")
            }
        } else {
            self.ai_move(player_id)
        }
    }

    fn ai_move(&self, player_id: usize) -> Action {
        Action::Play { index: 0 }
    }

    fn score(&self) -> usize {
        self.played_cards.values().map(|v| v.len()).sum()
    }

    fn print(&self) {
        for player_id in 0..self.n_players {
            println!("Player {}:", player_id);
            if player_id == self.client_player {
                println!("\tHIDDEN");
                continue;
            }
            for &Card(suite, value) in self.player_hands.get(player_id).unwrap() {
                print!("\t");
                print!("{}{} ", self.suite_name_map.get(&suite).unwrap(), value);
                print!("\n");
            }
        }
        println!("Fuses: {}", self.fuses);
        println!("Information tokens: {}", self.information_tokens);
    }

    fn game_loop(&mut self) {
        let mut status = Status::InProgress;
        loop {
            match status {
                Status::InProgress => {
                    for player_id in 0..self.n_players {
                        // print!("{}[2J", 27 as char);
                        self.print();
                        let action = self.get_player_action(player_id);
                        status = self.turn(player_id, action);
                    }
                }
                Status::Won => {
                    println!("You Won!! Great job! Final score: {}", self.score());
                    break;
                }
                Status::Lost => {
                    println!("Game over! Final score: {}", self.score());
                    break;
                }
            }
        }

    }
}


fn main() {
    let mut game = State::new(3);
    game.game_loop();
}
