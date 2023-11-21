#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Sensor,
    Actor,
    Hybrid,
}

impl ResourceType {
    pub fn from_string(type_str: &str) -> Result<ResourceType, String> {
        match type_str.to_lowercase().as_str() {
            "sensor" => Ok(ResourceType::Sensor),
            "actor" => Ok(ResourceType::Actor),
            "hybrid" => Ok(ResourceType::Hybrid),
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
            ResourceType::from_string("sensor").unwrap(),
            ResourceType::Sensor
        );
        assert_eq!(
            ResourceType::from_string("actor").unwrap(),
            ResourceType::Actor
        );
        assert_eq!(
            ResourceType::from_string("hybrid").unwrap(),
            ResourceType::Hybrid
        );
        if let Err(e) = ResourceType::from_string("sensor1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse sensor1");
        }
    }
}
