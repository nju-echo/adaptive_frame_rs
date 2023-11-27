use std::str::FromStr;

use strum;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum State {
    On,
    Off,
}

#[cfg(test)]
mod tests {
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
}
