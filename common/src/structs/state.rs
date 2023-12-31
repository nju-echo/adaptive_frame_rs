use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum State {
    On,
    Off,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(State::from_str("on").unwrap(), State::On);
        assert_eq!(State::from_str("off").unwrap(), State::Off);

        if let Err(e) = State::from_str("on1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse on1");
        }

        println!("{}", State::from_str("on").unwrap());
    }

    #[test]
    fn test_serialize() {
        let state = State::On;
        let serialized = serde_json::to_string(&state).unwrap();
        assert_eq!(serialized, "\"On\"");
    }

    #[test]
    fn test_deserialize() {
        let state = State::On;
        let deserialized: State = serde_json::from_str("\"On\"").unwrap();
        assert_eq!(deserialized, state);
    }
}
