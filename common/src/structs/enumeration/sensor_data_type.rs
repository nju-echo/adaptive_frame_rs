use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum SensorDataType {
    Msg,
    IncResult,
    InvReport,
}

impl Default for SensorDataType {
    fn default() -> Self {
        SensorDataType::Msg
    }
}

impl SensorDataType {
    pub fn is_default(&self) -> bool {
        *self == SensorDataType::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(
            SensorDataType::from_str("msg").unwrap(),
            SensorDataType::Msg
        );
        assert_eq!(
            SensorDataType::from_str("incresult").unwrap(),
            SensorDataType::IncResult
        );
        assert_eq!(
            SensorDataType::from_str("invreport").unwrap(),
            SensorDataType::InvReport
        );

        if let Err(e) = SensorDataType::from_str("msg1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse msg1");
        }

        println!("{}", SensorDataType::from_str("msg").unwrap());
    }
}
