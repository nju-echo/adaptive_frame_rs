use std::str::FromStr;

use strum;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum CompareType {
    #[strum(serialize = "==", serialize = "EQ")]
    EQ,

    #[strum(serialize = "!=", serialize = "NE")]
    NE,

    #[strum(serialize = ">", serialize = "GT")]
    GT,

    #[strum(serialize = ">=", serialize = "GE")]
    GE,

    #[strum(serialize = "<", serialize = "LT")]
    LT,

    #[strum(serialize = "<=", serialize = "LE")]
    LE,

    #[strum(serialize = "in", serialize = "IN")]
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
}
