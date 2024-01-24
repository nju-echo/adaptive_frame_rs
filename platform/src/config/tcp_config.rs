use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct TcpConfig {
    app_listen_port: u16,
    resource_listen_port: u16,
}

impl TcpConfig {
    pub fn get_app_listen_port(&self) -> u16 {
        self.app_listen_port
    }

    pub fn get_resource_listen_port(&self) -> u16 {
        self.resource_listen_port
    }

    pub fn tcp_config_init(json_object: Value) -> Self {
        serde_json::from_value(json_object).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_serialize_and_deserialize() {
        let tcp_config = TcpConfig {
            app_listen_port: 8080,
            resource_listen_port: 8081,
        };

        let json_str = serde_json::to_string(&tcp_config).unwrap();
        println!("{}", json_str);

        let tcp_config: TcpConfig = serde_json::from_str(&json_str).unwrap();
        println!("{:?}", tcp_config);

        let value = json!({
            "app_listen_port": 8080,
            "resource_listen_port": 8081,
        });
        let tcp_config: TcpConfig = serde_json::from_value(value).unwrap();
        println!("{:?}", tcp_config);
    }
}
