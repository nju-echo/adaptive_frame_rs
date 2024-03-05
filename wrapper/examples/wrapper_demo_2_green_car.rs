use std::fs::File;

use env_logger::Builder;

use common::socket::cmd_message::CmdMessage;
use common::structs::resource_config::ResourceConfig;
use wrapper::wrapper_remote_connector::WRAPPER_REMOTE_CONNECTOR;

fn main() {
    Builder::new().parse_filters("trace").init();

    let config_file = "resources/config/wrapper/green_car.json".to_string();
    let config: ResourceConfig =
        serde_json::from_reader(File::open(config_file).expect("open file fail"))
            .expect("parse json fail");

    if WRAPPER_REMOTE_CONNECTOR.register("127.0.0.1", 9091, config) {
        let mut cnt = 0;
        let mut flag = true;

        loop {
            flag = ((cnt % 20) == 0) != flag;
            cnt = cnt + 1;

            match WRAPPER_REMOTE_CONNECTOR.recv() {
                Some(recv) => {
                    if recv.cmd.expect("cmd is none").eq("sensory_request") {
                        let data = match flag {
                            true => serde_json::json!(
                                {
                                    "speed": 10.0,
                                    "longitude": 20.0,
                                    "latitude": 30.0,
                                }
                            ),
                            false => serde_json::json!(
                                {
                                    "speed": 20.0,
                                    "longitude": 30.0,
                                    "latitude": 10.0,
                                }
                            ),
                        };

                        let ret: CmdMessage =
                            CmdMessage::new(Some("sensory_back".to_string()), Some(data));

                        WRAPPER_REMOTE_CONNECTOR
                            .send(&serde_json::to_string(&ret).expect("to string fail"));
                        println!(
                            "[Wrapper]: {}",
                            serde_json::to_string(&ret).expect("to string fail")
                        );
                    }
                }
                None => {
                    panic!("recv error")
                }
            }
        }
    }
}
