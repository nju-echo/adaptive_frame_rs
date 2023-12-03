use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum ValueType {
    String,
    Int,
    Double,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(ValueType::from_str("string").unwrap(), ValueType::String);
        assert_eq!(ValueType::from_str("int").unwrap(), ValueType::Int);
        assert_eq!(ValueType::from_str("double").unwrap(), ValueType::Double);

        if let Err(e) = ValueType::from_str("string1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse string1");
        }

        println!("{}", ValueType::from_str("string").unwrap());
    }
}
