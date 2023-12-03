//use serde_value to deserialize json to a generic type
//getter and setter for sensor data

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::structs::enumeration::sensor_data_type::SensorDataType;

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorData {
    #[serde(default)]
    #[serde(skip_serializing_if = "SensorDataType::is_default")]
    sensor_data_type: SensorDataType,
    #[serde(flatten)]
    data: HashMap<String, Value>,
}

impl SensorData {
    /// constructor
    pub fn new(sensor_data_type: SensorDataType, fields: Vec<String>, values: Vec<Value>) -> Self {
        let data = fields.into_iter().zip(values.into_iter()).collect();
        Self {
            sensor_data_type,
            data,
        }
    }

    pub fn new_with_one_field(
        sensor_data_type: SensorDataType,
        field: String,
        value: Value,
    ) -> Self {
        let mut data = HashMap::new();
        data.insert(field, value);
        Self {
            sensor_data_type,
            data,
        }
    }

    pub fn new_with_json_str(sensor_data_type: SensorDataType, json_str: &str) -> Self {
        let data: HashMap<String, Value> = serde_json::from_str(json_str).unwrap();
        Self {
            sensor_data_type,
            data,
        }
    }

    pub fn new_without_data(sensor_data_type: SensorDataType) -> Self {
        Self {
            sensor_data_type,
            data: HashMap::new(),
        }
    }

    pub fn new_with_default_type(fields: Vec<String>, values: Vec<Value>) -> Self {
        SensorData::new(SensorDataType::default(), fields, values)
    }

    pub fn new_with_one_field_with_default_type(field: String, value: Value) -> Self {
        SensorData::new_with_one_field(SensorDataType::default(), field, value)
    }

    pub fn new_with_json_str_with_default_type(json_str: &str) -> Self {
        SensorData::new_with_json_str(SensorDataType::default(), json_str)
    }

    pub fn new_without_data_with_default_type() -> Self {
        SensorData::new_without_data(SensorDataType::default())
    }

    /// getter
    pub fn get_sensor_data_type(&self) -> SensorDataType {
        self.sensor_data_type
    }

    pub fn get_data_size(&self) -> usize {
        self.data.len()
    }

    pub fn get_all_data(&self) -> &HashMap<String, Value> {
        &self.data
    }

    pub fn get_data(&self, field: &str) -> Option<&Value> {
        self.data.get(field)
    }

    pub fn get_default_data(&self) -> Option<&Value> {
        self.data.get("default")
    }

    /// setter
    pub fn set_one_data(&mut self, field: String, value: Value) {
        self.data.insert(field, value);
    }

    pub fn set_default_data(&mut self, value: Value) {
        self.data.insert("default".to_string(), value);
    }

    pub fn set_data(&mut self, fields: Vec<String>, values: Vec<Value>) {
        fields
            .into_iter()
            .zip(values.into_iter())
            .for_each(|(field, value)| {
                self.data.insert(field, value);
            });
    }

    pub fn set_data_with_json_str(&mut self, json_str: &str) {
        let data: HashMap<String, Value> = serde_json::from_str(json_str).unwrap();
        self.data.extend(data);
    }

    //default error
    pub fn default_error() -> Self {
        Self::new_with_one_field_with_default_type("default".to_string(), json!("@#$%"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_and_deserialize() {
        let json_str = r#"{
        "test" : 64
        }"#;
        let sensor_data: SensorData = serde_json::from_str(json_str).unwrap();
        println!("{:?}", sensor_data);
        println!("{}", serde_json::to_string(&sensor_data).unwrap());
    }

    #[test]
    fn test_new() {
        let sensor_data = SensorData::new(
            SensorDataType::IncResult,
            vec!["test".to_string()],
            vec![json!("test")],
        );
        println!("{:?}", sensor_data);
        println!("{}", serde_json::to_string(&sensor_data).unwrap());
    }

    #[test]
    fn test_new_with_json_str() {
        let json_str = r#"{
        "test" : "test"
        }"#;
        let sensor_data = SensorData::new_with_json_str(SensorDataType::IncResult, json_str);
        println!("{:?}", sensor_data);
        println!("{}", serde_json::to_string(&sensor_data).unwrap());
    }

    #[test]
    fn test_getter() {
        let json_str = r#"{
        "test" : "test"
        }"#;
        let sensor_data = SensorData::new_with_json_str(SensorDataType::IncResult, json_str);
        println!("{:?}", sensor_data);
        println!("{}", serde_json::to_string(&sensor_data).unwrap());
        println!("{:?}", sensor_data.get_data("test"));
        println!("{:?}", sensor_data.get_data("test1"));
        println!("{:?}", sensor_data.get_default_data());
        println!("{:?}", sensor_data.get_default_data());
        println!("{:?}", sensor_data.get_all_data());
        println!("{:?}", sensor_data.get_data_size());
        println!("{:?}", sensor_data.get_sensor_data_type());
    }

    #[test]
    fn test_setter() {
        let json_str = r#"{
        "test" : "test"
        }"#;
        let mut sensor_data = SensorData::new_with_json_str(SensorDataType::IncResult, json_str);
        println!("{:?}", sensor_data);
        println!("{}", serde_json::to_string(&sensor_data).unwrap());
        sensor_data.set_one_data("test1".to_string(), json!("test1"));
        println!("{:?}", sensor_data);
        println!("{}", serde_json::to_string(&sensor_data).unwrap());
        sensor_data.set_default_data(json!("default"));
        println!("{:?}", sensor_data);
        println!("{}", serde_json::to_string(&sensor_data).unwrap());
        sensor_data.set_data(vec!["test2".to_string()], vec![json!("test2")]);
        println!("{:?}", sensor_data);
        println!("{}", serde_json::to_string(&sensor_data).unwrap());
        sensor_data.set_data_with_json_str(
            r#"{
        "test3" : "test3"
        }"#,
        );
        println!("{:?}", sensor_data);
        println!("{}", serde_json::to_string(&sensor_data).unwrap());
    }
}
