use std::io::Write;
use std::net::TcpStream;
use std::thread;

use env_logger::Builder;
use log::debug;

use common::socket::cmd_message::CmdMessage;
use common::structs::enumeration::resource_type::ResourceType;
use common::structs::resource_config::ResourceConfig;
use platform::config::configuration::config_analyze;
use platform::resource::res_mgr_thread::RES_MGR_THREAD;

fn start_platform_simple() {
    Builder::new().parse_filters("trace").init();
    let config_file = "./configfile".to_string();
    config_analyze(config_file.as_ref());

    thread::spawn(move || {
        RES_MGR_THREAD.run();
    });
}

#[test]
fn test_resource_simple() {
    let config = ResourceConfig::new(
        Some("YellowCar".to_string()),
        ResourceType::Sensor,
        Some(vec![
            "speed".to_string(),
            "longitude".to_string(),
            "latitude".to_string(),
        ]),
    );

    start_platform_simple();

    thread::sleep(std::time::Duration::from_secs(1));

    let mut stream = TcpStream::connect("127.0.0.1:9091").unwrap();
    let cmd_message = CmdMessage::new(
        Some("register".to_string()),
        Some(serde_json::to_string(&config).unwrap()),
    );
    //should add '\n' to the end of json string
    let json_str = serde_json::to_string(&cmd_message).unwrap() + "\n";
    debug!("{}", json_str);

    stream.write_all(json_str.as_bytes()).unwrap();
    stream.flush().unwrap();

    thread::sleep(std::time::Duration::from_secs(1));
}
