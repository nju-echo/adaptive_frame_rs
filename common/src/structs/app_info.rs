use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::structs::state::State;

/// AppInfo used to describe app and be send to the platform
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct AppInfo {
    pub app_name: Option<Arc<String>>,
    pub ctx: Option<bool>,
    pub inv: Option<bool>,
    pub state: State,
    pub sensors: Vec<String>,
    pub actors: Vec<String>,
}

impl AppInfo {
    pub fn new(
        app_name: Option<Arc<String>>,
        ctx: Option<bool>,
        inv: Option<bool>,
        state: State,
        sensors: Vec<String>,
        actors: Vec<String>,
    ) -> Self {
        Self {
            app_name,
            ctx,
            inv,
            state,
            sensors,
            actors,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_serialize_and_deserialize() {
        let app_info = AppInfo::new(
            Some(Arc::new("test".to_string())),
            Some(true),
            Some(true),
            State::On,
            vec!["test".to_string()],
            vec!["test".to_string()],
        );

        let json_str = serde_json::to_string(&app_info).unwrap();
        println!("{}", json_str);

        let app_info: AppInfo = serde_json::from_str(&json_str).unwrap();
        println!("{:?}", app_info);

        let value = json!({
            "ctx": true,
            "inv": true,
            "state": "On",
            "sensors": ["test"],
            "actors": ["test"],
            "other": "test",
        });
        let app_info: AppInfo = serde_json::from_value(value).unwrap();
        println!("{:?}", app_info);
    }
}
