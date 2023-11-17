#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckResult {
    InvGenerating,
    InvViolated,
    InvPassed,
}

impl CheckResult {
    pub fn from_string(result_str: &str) -> Result<CheckResult, String> {
        match result_str.to_lowercase().as_str() {
            "inv_generating" => Ok(CheckResult::InvGenerating),
            "inv_violated" => Ok(CheckResult::InvViolated),
            "inv_passed" => Ok(CheckResult::InvPassed),
            other => {
                //println!("{}",format!("No constant with text {} found in text", other));
                Err(format!("No constant with text {} found", other))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(
            CheckResult::from_string("Inv_Generating").unwrap(),
            CheckResult::InvGenerating
        );
        assert_eq!(
            CheckResult::from_string("Inv_Violated").unwrap(),
            CheckResult::InvViolated
        );
        assert_eq!(
            CheckResult::from_string("Inv_Passed").unwrap(),
            CheckResult::InvPassed
        );
        if let Err(e) = CheckResult::from_string("Inv_Generating1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse Inv_Generating1");
        }
    }
}
