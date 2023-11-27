use serde_json::Value;

pub trait ServiceResult {
    fn to_json_string(&self) -> String;
    fn to_json_object(&self) -> Value;
}
