#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServiceType {
    Ctx,
    Inv,
    All,
}

impl ServiceType {
    pub fn from_string(type_str: &str) -> Result<ServiceType, String> {
        match type_str.to_lowercase().as_str() {
            "ctx" => Ok(ServiceType::Ctx),
            "inv" => Ok(ServiceType::Inv),
            "all" => Ok(ServiceType::All),
            other => Err(format!("No constant with text {} found", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(ServiceType::from_string("ctx").unwrap(), ServiceType::Ctx);
        assert_eq!(ServiceType::from_string("inv").unwrap(), ServiceType::Inv);
        assert_eq!(ServiceType::from_string("all").unwrap(), ServiceType::All);
        if let Err(e) = ServiceType::from_string("ctx1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse ctx1");
        }
    }
}
