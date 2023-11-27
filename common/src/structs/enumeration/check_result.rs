use std::str::FromStr;

use strum;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(ascii_case_insensitive)]
pub enum CheckResult {
    InvGenerating,
    InvViolated,
    InvPassed,
}

#[cfg(test)]
mod tests {
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
