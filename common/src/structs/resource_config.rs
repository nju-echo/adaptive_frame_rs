use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::structs::enumeration::resource_type::ResourceType;

///ResourceConfig used to describe the resource and be send to the platform
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceConfig {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub resource_type: ResourceType,
    pub fields: Option<Vec<String>>,
}

impl ResourceConfig {
    pub fn new(
        name: Option<String>,
        resource_type: ResourceType,
        fields: Option<Vec<String>>,
    ) -> Self {
        Self {
            name,
            resource_type,
            fields,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_and_deserialize() {
        let resource_config = ResourceConfig::new(
            Some("test".to_string()),
            ResourceType::Sensor,
            Some(vec!["test".to_string()]),
        );

        let json_str = serde_json::to_string(&resource_config).unwrap();
        println!("{}", json_str);

        let resource_config: ResourceConfig = serde_json::from_str(&json_str).unwrap();
        println!("{:?}", resource_config);

        let value = json!({
            "name": "test",
            "type": "Sensor",
            "fields": ["test"],
            "other": "test",
        });
        let resource_config: ResourceConfig = serde_json::from_value(value).unwrap();
        println!("{:?}", resource_config);
    }
}
