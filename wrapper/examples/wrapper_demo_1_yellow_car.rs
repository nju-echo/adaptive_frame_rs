use env_logger::Builder;
use serde_json::json;

use common::socket::cmd_message::CmdMessage;
use common::structs::enumeration::resource_type::ResourceType;
use common::structs::resource_config::ResourceConfig;
use wrapper::wrapper_remote_connector::WRAPPER_REMOTE_CONNECTOR;

fn main() {
    Builder::new().parse_filters("trace").init();

    let config = ResourceConfig::new(
        Some("YellowCar".to_string()),
        ResourceType::Sensor,
        Some(vec![
            "speed".to_string(),
            "longitude".to_string(),
            "latitude".to_string(),
        ]),
    );

    if WRAPPER_REMOTE_CONNECTOR.register("127.0.0.1", 9091, config) {
        let mut cnt = 0;
        loop {
            cnt += 1;

            //todo: check whether recv message?
            match WRAPPER_REMOTE_CONNECTOR.recv() {
                Some(cmd_message) => {
                    if cmd_message.cmd.expect("cmd is none").eq("sensory_request") {
                        let data = json!(
                            {
                                "speed": 10.0,
                                "longitude": 20.0,
                                "latitude": 30.0,
                            }
                        );
                        let ret: CmdMessage =
                            CmdMessage::new(Some("sensory_back".to_string()), Some(data));
                        WRAPPER_REMOTE_CONNECTOR
                            .send(&serde_json::to_string(&ret).expect("to string fail"));
                    }
                }
                None => {
                    panic!("recv error")
                }
            }
        }
    }
}
