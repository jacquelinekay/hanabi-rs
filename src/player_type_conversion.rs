extern crate serde;

use std::fmt;

use self::serde::ser::{Error, Serializer, SerializeSeq};
use self::serde::de::{Deserializer, SeqAccess, Visitor};

use super::types::PlayerType;
use super::player::{CommandLinePlayer, NaiveAIPlayer, NetworkPlayer};

pub fn serialize<S>(input: &Vec<PlayerType>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    serializer.serialize_seq(Some(input.len()))
        .and_then(
            |mut state|
            input.iter().filter_map(
                |e|
                match e {
                    &PlayerType::CommandLine(_) => state.serialize_element("command_line"),
                    &PlayerType::NaiveAI(_) => state.serialize_element("naive_ai"),
                    &PlayerType::Network(_) => state.serialize_element("network"),
                }.ok()
            ).fold(
                Result<SerializeSeq, S::Error>::Ok(state),
                |result, x|
                match x {
                    SerializeSeq::Error(_) => Err(S::Error::custom("yarr")),
                    _ => result,
                }
            )
        )
        .and_then(|mut state| state.end())
}

struct PlayerVecVisitor;

impl<'de> Visitor<'de> for PlayerVecVisitor {
    type Value = Vec<PlayerType>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // TODO
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where S: SeqAccess<'de>
    {
        let mut values: Vec<PlayerType> = Vec::new();
        while let Some(value) = try!(seq.next_element()) {
            match value {
                "command_line" => values.push(PlayerType::CommandLine(CommandLinePlayer {})),
                "naive_ai" => values.push(PlayerType::NaiveAI(NaiveAIPlayer {})),
                // TODO: need to serialize with metadata?
                "network" => values.push(PlayerType::Network(NetworkPlayer::new(String::from("")))),
                _ => return Err(serde::de::Error::custom("unrecognized player type")),  // TODO
            }
        }
        Ok(values)
    }
}


pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<PlayerType>, D::Error>
    where D: Deserializer<'de>
{
    deserializer.deserialize_seq(PlayerVecVisitor)
}

/*
impl Serialize for Vec<PlayerType> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_seq(self.len());
        for e in self {
            match e {
                PlayerType::CommandLine(_) => state.serialize_string("command_line"),
                PlayerType::NaiveAI(_) => state.serialize_string("naive_ai"),
                PlayerType::Network(_) => state.serialize_string("network"),
            }
        }
        state.end()
    }
}

impl<'de> Deserialize for Vec<PlayerType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_seq(PlayerVecVisitor);
    }
}
*/
