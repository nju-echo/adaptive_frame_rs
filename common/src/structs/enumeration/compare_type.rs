use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum CompareType {
    #[strum(serialize = "==", serialize = "EQ")]
    #[serde(alias = "==")]
    EQ,

    #[strum(serialize = "!=", serialize = "NE")]
    #[serde(alias = "!=")]
    NE,

    #[strum(serialize = ">", serialize = "GT")]
    #[serde(alias = ">")]
    GT,

    #[strum(serialize = ">=", serialize = "GE")]
    #[serde(alias = ">=")]
    GE,

    #[strum(serialize = "<", serialize = "LT")]
    #[serde(alias = "<")]
    LT,

    #[strum(serialize = "<=", serialize = "LE")]
    #[serde(alias = "<=")]
    LE,

    #[strum(serialize = "in", serialize = "IN")]
    #[serde(alias = "in")]
    IN,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(CompareType::from_str("==").unwrap(), CompareType::EQ);
        assert_eq!(CompareType::from_str("eQ").unwrap(), CompareType::EQ);

        assert_eq!(CompareType::from_str("!=").unwrap(), CompareType::NE);
        assert_eq!(CompareType::from_str(">").unwrap(), CompareType::GT);
        assert_eq!(CompareType::from_str(">=").unwrap(), CompareType::GE);
        assert_eq!(CompareType::from_str("<").unwrap(), CompareType::LT);
        assert_eq!(CompareType::from_str("<=").unwrap(), CompareType::LE);
        assert_eq!("in".parse::<CompareType>().unwrap(), CompareType::IN);

        if let Err(e) = CompareType::from_str("==1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse ==1");
        }

        println!("{}", CompareType::from_str("==").unwrap());
    }

    #[test]
    fn test_serialize_and_deserialize() {
        let compare_type = CompareType::EQ;
        let serialized = serde_json::to_string(&compare_type).unwrap();
        assert_eq!(serialized, "\"EQ\"");

        let deserialized: CompareType = serde_json::from_str("\"==\"").unwrap();
        assert_eq!(deserialized, compare_type);
    }
}
