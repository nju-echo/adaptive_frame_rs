use std::fs;
use std::path::Path;
use std::sync::Mutex;

use log::{error, info};
use once_cell::sync::Lazy;
use serde_json::{json, Value};

use crate::config::ctx_server_config::CtxServerConfig;
use crate::config::tcp_config::TcpConfig;

/// analyze config file and init config
/// config file format: JSON(should not have comma at the end of the line)
pub static CTX_SERVER_CONFIG: Lazy<Mutex<CtxServerConfig>> = Lazy::new(|| {
    Mutex::new(CtxServerConfig::ctx_server_config_init(json!({
        "server_on": false,
        "ctx_validator": "default_ctx_validator",
        "base_rule_file": "default_base_rule_file",
        "base_bfunc_file": "default_base_bfunc_file",
        "base_pattern_file": "default_base_pattern_file",
        "base_mfunc_file": "default_base_mfunc_file",
    })))
});

pub static TCP_CONFIG: Lazy<Mutex<TcpConfig>> = Lazy::new(|| {
    Mutex::new(TcpConfig::tcp_config_init(json!({
        "app_listen_port": 0,
        "resource_listen_port": 0,
    })))
});

pub fn config_analyze(config_file: &Path) {
    match fs::read_to_string(config_file) {
        Ok(config_str) => match serde_json::from_str::<Value>(&config_str) {
            Ok(config_json) => {
                let ctx_server_config = config_json["ctx_server_config"].clone();
                let tcp_config = config_json["tcp_config"].clone();

                let mut ctx_server_config_mut = CTX_SERVER_CONFIG.lock().unwrap();
                *ctx_server_config_mut = CtxServerConfig::ctx_server_config_init(ctx_server_config);

                let mut tcp_config_mut = TCP_CONFIG.lock().unwrap();
                *tcp_config_mut = TcpConfig::tcp_config_init(tcp_config);

                info!("config file analyze success");
                info!("ctx_server_config: {:?}", *ctx_server_config_mut);
                info!("tcp_config: {:?}", *tcp_config_mut);
            }
            Err(e) => {
                error!("parse config file error: {}", e);
            }
        },
        Err(e) => {
            error!("read config file: {:?} error: {}", config_file, e);
        }
    }
}

#[cfg(test)]
mod tests {
    use env_logger::Builder;

    use super::*;

    #[test]
    fn test_config_analyze() {
        Builder::new().parse_filters("info").init();
        let config_file = "./configfile".to_string();
        config_analyze(config_file.as_ref());
    }
}
