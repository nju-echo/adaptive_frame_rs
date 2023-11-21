#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensorDataType {
    Msg,
    IncResult,
    InvReport,
}

impl SensorDataType {
    pub fn from_string(type_str: &str) -> Result<SensorDataType, String> {
        match type_str.to_lowercase().as_str() {
            "msg" => Ok(SensorDataType::Msg),
            "inc_result" => Ok(SensorDataType::IncResult),
            "inv_report" => Ok(SensorDataType::InvReport),
            other => Err(format!("No constant with text {} found", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(
            SensorDataType::from_string("msg").unwrap(),
            SensorDataType::Msg
        );
        assert_eq!(
            SensorDataType::from_string("inc_result").unwrap(),
            SensorDataType::IncResult
        );
        assert_eq!(
            SensorDataType::from_string("inv_report").unwrap(),
            SensorDataType::InvReport
        );
        if let Err(e) = SensorDataType::from_string("msg1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse msg1");
        }
    }
}
