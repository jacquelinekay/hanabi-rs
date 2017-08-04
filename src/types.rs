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

pub enum HintType {
    SuiteType(Suite),
    Number(usize),
}

pub enum Action {
    Hint { receiver_id: usize, hint: HintType }, // Give a hint to player_id
    Discard { index: usize }, // Discard the card at index
    Play { index: usize }, // Play the card at index
}

pub enum Status {
    InProgress,
    Won,
    Lost,
}

// Cards have value between 1 and 5
pub struct Card(pub Suite, pub usize);

pub type PlayerHand = Vec<Card>;
