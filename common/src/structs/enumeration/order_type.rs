use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum OrderType {
    Asc,
    Desc,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(OrderType::from_str("asc").unwrap(), OrderType::Asc);
        assert_eq!(OrderType::from_str("desc").unwrap(), OrderType::Desc);

        if let Err(e) = OrderType::from_str("asc1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse asc1");
        }

        println!("{}", OrderType::from_str("asc").unwrap());
    }
}
