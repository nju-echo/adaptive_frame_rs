#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompareType {
    EQ,
    NE,
    GT,
    GE,
    LT,
    LE,
    IN,
}

impl CompareType {
    pub fn get_symbol(&self) -> &str {
        match self {
            CompareType::EQ => "==",
            CompareType::NE => "!=",
            CompareType::GT => ">",
            CompareType::GE => ">=",
            CompareType::LT => "<",
            CompareType::LE => "<=",
            CompareType::IN => "in",
        }
    }

    pub fn from_string(type_str: &str) -> Result<CompareType, String> {
        match type_str.to_lowercase().as_str() {
            "eq" | "==" => Ok(CompareType::EQ),
            "ne" | "!=" => Ok(CompareType::NE),
            "gt" | ">" => Ok(CompareType::GT),
            "ge" | ">=" => Ok(CompareType::GE),
            "lt" | "<" => Ok(CompareType::LT),
            "le" | "<=" => Ok(CompareType::LE),
            "in" => Ok(CompareType::IN),
            other => Err(format!("No constant with text {} found", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_symbol() {
        assert_eq!(CompareType::EQ.get_symbol(), "==");
        assert_eq!(CompareType::NE.get_symbol(), "!=");
        assert_eq!(CompareType::GT.get_symbol(), ">");
        assert_eq!(CompareType::GE.get_symbol(), ">=");
        assert_eq!(CompareType::LT.get_symbol(), "<");
        assert_eq!(CompareType::LE.get_symbol(), "<=");
        assert_eq!(CompareType::IN.get_symbol(), "in");
    }

    #[test]
    fn test_from_string() {
        assert_eq!(CompareType::from_string("eq").unwrap(), CompareType::EQ);
        assert_eq!(CompareType::from_string("==").unwrap(), CompareType::EQ);
        assert_eq!(CompareType::from_string("ne").unwrap(), CompareType::NE);
        assert_eq!(CompareType::from_string("!=").unwrap(), CompareType::NE);
        assert_eq!(CompareType::from_string("gt").unwrap(), CompareType::GT);
        assert_eq!(CompareType::from_string(">").unwrap(), CompareType::GT);
        assert_eq!(CompareType::from_string("ge").unwrap(), CompareType::GE);
        assert_eq!(CompareType::from_string(">=").unwrap(), CompareType::GE);
        assert_eq!(CompareType::from_string("lt").unwrap(), CompareType::LT);
        assert_eq!(CompareType::from_string("<").unwrap(), CompareType::LT);
        assert_eq!(CompareType::from_string("le").unwrap(), CompareType::LE);
        assert_eq!(CompareType::from_string("<=").unwrap(), CompareType::LE);
        assert_eq!(CompareType::from_string("in").unwrap(), CompareType::IN);
        if let Err(e) = CompareType::from_string("eq1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse eq1");
        }
    }
}
