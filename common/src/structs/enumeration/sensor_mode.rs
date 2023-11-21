#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensorMode {
    Active,
    Passive,
}

impl SensorMode {
    pub fn from_string(mode_str: &str) -> Result<SensorMode, String> {
        match mode_str.to_lowercase().as_str() {
            "active" => Ok(SensorMode::Active),
            "passive" => Ok(SensorMode::Passive),
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
            SensorMode::from_string("active").unwrap(),
            SensorMode::Active
        );
        assert_eq!(
            SensorMode::from_string("PASSIVE").unwrap(),
            SensorMode::Passive
        );
        if let Err(e) = SensorMode::from_string("active1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse active1");
        }
    }
}
