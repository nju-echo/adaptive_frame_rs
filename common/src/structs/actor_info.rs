use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::structs::state::State;
use crate::structs::value_type::ValueType;

/// ActorInfo used to describe actor and be send to the platform
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ActorInfo {
    pub actor_name: Option<Arc<String>>,
    pub value_type: ValueType,
    pub state: State,
    pub apps: Vec<String>,
}

impl ActorInfo {
    pub fn new(
        actor_name: Option<Arc<String>>,
        value_type: ValueType,
        state: State,
        apps: Vec<String>,
    ) -> Self {
        Self {
            actor_name,
            value_type,
            state,
            apps,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_serialize_and_deserialize() {
        let actor_info = ActorInfo::new(
            Some(Arc::new("test".to_string())),
            ValueType::String,
            State::On,
            vec!["test".to_string()],
        );

        let json_str = serde_json::to_string(&actor_info).unwrap();
        println!("{}", json_str);

        let actor_info: ActorInfo = serde_json::from_str(&json_str).unwrap();
        println!("{:?}", actor_info);

        let value = json!({
            "value_type": "String",
            "state": "On",
            "apps": ["test"],
        });
        let actor_info: ActorInfo = serde_json::from_value(value).unwrap();
        println!("{:?}", actor_info);
    }
}
