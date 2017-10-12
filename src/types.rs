use std::vec::Vec;

use super::player::{CommandLinePlayer, NaiveAIPlayer, NetworkPlayer};

/*
custom_derive! {
    #[derive(PartialEq, Eq, Hash, IterVariants(SuiteVariants))]
    pub enum Suite {
        White,
        Yellow,
        Green,
        Blue,
        Red,
    }
}
*/

#[derive(PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Suite {
    White,
    Yellow,
    Green,
    Blue,
    Red,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum HintType {
    SuiteType(Suite),
    Number(usize),
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Action {
    Hint { receiver_id: usize, hint: HintType },
    Discard { index: usize },
    Play { index: usize },
}

pub enum Status {
    InProgress,
    Won,
    Lost,
}

// Cards have value between 1 and 5
// pub struct Card(pub Suite, pub usize);
pub struct Card {
    pub suite: Suite,
    pub value: usize,
}

pub type PlayerHand = Vec<Card>;

pub enum PlayerType {
    CommandLine(CommandLinePlayer),
    NaiveAI(NaiveAIPlayer),
    Network(NetworkPlayer),
}
