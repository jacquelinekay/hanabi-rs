use std::vec::Vec;

use super::player::{CommandLinePlayer, NaiveAIPlayer, NetworkPlayer};

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

pub enum HintType {
    SuiteType(Suite),
    Number(usize),
}

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
