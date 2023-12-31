use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum CheckResult {
    InvGenerating,
    InvViolated,
    InvPassed,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(
            CheckResult::from_str("InvgeNerating").unwrap(),
            CheckResult::InvGenerating
        );
        assert_eq!(
            CheckResult::from_str("InvViolated").unwrap(),
            CheckResult::InvViolated
        );
        assert_eq!(
            CheckResult::from_str("InvPassed").unwrap(),
            CheckResult::InvPassed
        );

        if let Err(e) = CheckResult::from_str("InvGenerating1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse InvGenerating1");
        }

        println!("{}", CheckResult::from_str("InvGenerating").unwrap());
    }
}
