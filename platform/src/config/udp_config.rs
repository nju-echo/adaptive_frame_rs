use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct UdpConfig {
    server_port: u16,
    client_port: Vec<ClientPort>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ClientPort {
    port: u16,
}

impl UdpConfig {
    pub fn get_server_port(&self) -> u16 {
        self.server_port
    }

    pub fn get_client_port(&self) -> &Vec<ClientPort> {
        &self.client_port
    }

    pub fn udp_config_init(json_object: serde_json::Value) -> Self {
        serde_json::from_value(json_object).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_serialize_and_deserialize() {
        let udp_config = UdpConfig {
            server_port: 8080,
            client_port: vec![ClientPort { port: 8081 }],
        };

        let json_str = serde_json::to_string(&udp_config).unwrap();
        println!("{}", json_str);

        let udp_config: UdpConfig = serde_json::from_str(&json_str).unwrap();
        println!("{:?}", udp_config);

        let value = json!({
            "server_port": 8080,
            "client_port": [{"port": 8081}],
        });
        let udp_config: UdpConfig = serde_json::from_value(value).unwrap();
        println!("{:?}", udp_config);
    }
}
