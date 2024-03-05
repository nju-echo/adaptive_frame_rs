use std::thread;

use env_logger::Builder;

use platform::config::configuration::config_analyze;
use platform::resource::res_mgr_thread::RES_MGR_THREAD;

fn main() {
    //输出所在的文件名和行号
    Builder::new().parse_filters("trace").init();
    let config_file = "platform/configfile".to_string();
    config_analyze(config_file.as_ref());

    thread::spawn(move || {
        RES_MGR_THREAD.run();
    })
    .join();
}
