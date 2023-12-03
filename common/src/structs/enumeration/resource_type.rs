use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum ResourceType {
    Sensor,
    Actor,
    Hybrid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(
            ResourceType::from_str("sensor").unwrap(),
            ResourceType::Sensor
        );
        assert_eq!(
            ResourceType::from_str("actor").unwrap(),
            ResourceType::Actor
        );
        assert_eq!(
            ResourceType::from_str("hybrid").unwrap(),
            ResourceType::Hybrid
        );

        if let Err(e) = ResourceType::from_str("sensor1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse sensor1");
        }

        println!("{}", ResourceType::from_str("sensor").unwrap());
    }
}
