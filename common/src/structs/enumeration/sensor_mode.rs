use std::str::FromStr;

use strum;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum SensorMode {
    Active,
    Passive,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(SensorMode::from_str("active").unwrap(), SensorMode::Active);
        assert_eq!(
            SensorMode::from_str("passive").unwrap(),
            SensorMode::Passive
        );

        if let Err(e) = SensorMode::from_str("active1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse active1");
        }

        println!("{}", SensorMode::from_str("active").unwrap());
    }
}
