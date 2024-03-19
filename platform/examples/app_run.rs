use std::thread;

use env_logger::Builder;

use platform::app::app_mgr_thread::APP_MGR_THREAD;
use platform::config::configuration::config_analyze;

fn main() {
    Builder::new().parse_filters("trace").init();
    let config_file = "platform/configfile".to_string();
    config_analyze(config_file.as_ref());

    thread::spawn(move || {
        APP_MGR_THREAD.run();
    })
    .join()
    .unwrap();
}
