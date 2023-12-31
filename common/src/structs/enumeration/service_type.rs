use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum ServiceType {
    Ctx,
    Inv,
    All,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(ServiceType::from_str("ctx").unwrap(), ServiceType::Ctx);
        assert_eq!(ServiceType::from_str("inv").unwrap(), ServiceType::Inv);
        assert_eq!(ServiceType::from_str("all").unwrap(), ServiceType::All);

        if let Err(e) = ServiceType::from_str("ctx1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse ctx1");
        }

        println!("{}", ServiceType::from_str("ctx").unwrap());
    }
}
