//Todo
//lazy_init: failed for fmt use unmut json
//refactot to be a strong struct by removing jo
//learn how to parse something Option and how to serialize enum by serde_json

use std::str::FromStr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::structs::state::State;
use crate::structs::value_type::ValueType;

/// SensorInfo used to describe sensor and be send to the platform
#[derive(Debug, Serialize, Deserialize)]
pub struct SensorInfo {
    pub sensor_name: Option<Arc<String>>,
    pub value_type: ValueType,
    pub fields: Arc<Vec<String>>,
    pub state: State,
    pub apps: Arc<Vec<String>>,
}

impl SensorInfo {
    pub fn new(
        sensor_name: Option<Arc<String>>,
        value_type: ValueType,
        fields: Arc<Vec<String>>,
        state: State,
        apps: Arc<Vec<String>>,
    ) -> Self {
        Self {
            sensor_name,
            value_type,
            fields,
            state,
            apps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_and_deserialize() {
        let sensor_info = SensorInfo::new(
            Some(Arc::new("test".to_string())),
            ValueType::String,
            Arc::new(vec!["test".to_string()]),
            State::On,
            Arc::new(vec!["test".to_string()]),
        );

        let json_str = serde_json::to_string(&sensor_info).unwrap();
        println!("{}", json_str);

        let sensor_info: SensorInfo = serde_json::from_str(&json_str).unwrap();
        println!("{:?}", sensor_info);

        let value = json!({
            "value_type": "String",
            "fields": ["test"],
            "state": "On",
            "apps": ["test"],
            "other": "test",
        });
        let sensor_info: SensorInfo = serde_json::from_value(value).unwrap();
        println!("{:?}", sensor_info);

        let json_str = serde_json::to_string(&sensor_info).unwrap();
        println!("{}", json_str);

        let json_value = serde_json::to_value(&sensor_info).unwrap();
        println!("{:?}", json_value);

        let json_str = String::from("{\"value_type\":\"String\",\"fields\":[\"test\"],\"state\":\"On\",\"apps\":[\"test\"]}");
        let sensor_info: SensorInfo = serde_json::from_str(&json_str).unwrap();
        println!("{:?}", sensor_info);
    }
}
