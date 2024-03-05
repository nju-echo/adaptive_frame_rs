use std::fs::File;

use env_logger::Builder;
use serde_json::json;

use common::socket::cmd_message::CmdMessage;
use common::structs::resource_config::ResourceConfig;
use wrapper::wrapper_remote_connector::WRAPPER_REMOTE_CONNECTOR;

fn main() {
    Builder::new().parse_filters("trace").init();

    let config_file = "resources/config/wrapper/yellow_car_motor.json".to_string();
    let config: ResourceConfig =
        serde_json::from_reader(File::open(config_file).expect("open file fail"))
            .expect("parse json fail");

    if WRAPPER_REMOTE_CONNECTOR.register("127.0.0.1", 9091, config) {
        loop {
            match WRAPPER_REMOTE_CONNECTOR.recv() {
                Some(recv) => {
                    if recv.cmd.expect("cmd is none").eq("action_request") {
                        let ret =
                            CmdMessage::new(Some("action_back".to_string()), Some(json!("true")));
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
