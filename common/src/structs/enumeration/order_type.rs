#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderType {
    Asc,
    Desc,
}

impl OrderType {
    pub fn from_string(type_str: &str) -> Result<OrderType, String> {
        match type_str.to_lowercase().as_str() {
            "asc" => Ok(OrderType::Asc),
            "desc" => Ok(OrderType::Desc),
            other => Err(format!("No constant with text {} found", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(OrderType::from_string("ASC").unwrap(), OrderType::Asc);
        assert_eq!(OrderType::from_string("DESC").unwrap(), OrderType::Desc);
        if let Err(e) = OrderType::from_string("asc1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse asc1");
        }
    }
}
