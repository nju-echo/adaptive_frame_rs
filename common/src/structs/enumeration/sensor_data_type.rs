use std::str::FromStr;

use strum;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum SensorDataType {
    Msg,
    IncResult,
    InvReport,
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
