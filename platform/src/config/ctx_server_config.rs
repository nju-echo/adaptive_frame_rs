use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::resource::sensor_mgr::SensorMgr;

/// CtxServerConfig is a struct that contains the configuration of ctx_server.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CtxServerConfig {
    server_on: bool,
    ctx_validator: String,
    #[serde(skip)]
    ctx_checker: String,
    #[serde(skip)]
    ctx_scheduler: String,
    base_rule_file: String,
    base_bfunc_file: String,
    base_pattern_file: String,
    base_mfunc_file: String,
    #[serde(skip)]
    sensor_mgr_map: HashMap<String, SensorMgr>,
}

impl CtxServerConfig {
    //getter
    pub fn is_server_on(&self) -> bool {
        self.server_on
    }

    pub fn get_ctx_validator(&self) -> &str {
        &self.ctx_validator
    }

    pub fn get_ctx_checker(&self) -> &str {
        &self.ctx_checker
    }

    pub fn get_ctx_scheduler(&self) -> &str {
        &self.ctx_scheduler
    }

    pub fn get_base_rule_file(&self) -> &str {
        &self.base_rule_file
    }

    pub fn get_base_bfunc_file(&self) -> &str {
        &self.base_bfunc_file
    }

    pub fn get_base_pattern_file(&self) -> &str {
        &self.base_pattern_file
    }

    pub fn get_base_mfunc_file(&self) -> &str {
        &self.base_mfunc_file
    }

    pub fn get_sensor_mgr_map(&self) -> &HashMap<String, SensorMgr> {
        &self.sensor_mgr_map
    }

    //setter
    pub fn add_sensor_config(&mut self, sensor_name: String, sensor_mgr: SensorMgr) {
        self.sensor_mgr_map.insert(sensor_name, sensor_mgr);
    }

    pub fn set_ctx_checker(&mut self) {
        self.ctx_checker = self
            .ctx_validator
            .split("+")
            .next()
            .unwrap_or("")
            .to_string();
    }

    pub fn set_ctx_scheduler(&mut self) {
        self.ctx_scheduler = self
            .ctx_validator
            .split("+")
            .nth(1)
            .unwrap_or("")
            .to_string();
    }

    //init
    pub fn ctx_server_config_init(json_object: Value) -> Self {
        let mut ret: CtxServerConfig = serde_json::from_value(json_object).unwrap();
        ret.set_ctx_checker();
        ret.set_ctx_scheduler();
        ret
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_serialize_and_deserialize() {
        let value = json!({
            "server_on": true,
            "ctx_validator": "ctx_checker+ctx_scheduler",
            "base_rule_file": "base_rule_file",
            "base_bfunc_file": "base_bfunc_file",
            "base_pattern_file": "base_pattern_file",
            "base_mfunc_file": "base_mfunc_file",
        });
        let mut ctx_server_config: CtxServerConfig = serde_json::from_value(value).unwrap();
        ctx_server_config.set_ctx_scheduler();
        ctx_server_config.set_ctx_checker();
        println!("{}", serde_json::to_string(&ctx_server_config).unwrap());
        println!("{:?}", ctx_server_config);
    }

    #[test]
    fn test_ctx_server_config_init() {
        let value = json!({
            "server_on": true,
            "ctx_validator": "ctx_checker+ctx_scheduler",
            "base_rule_file": "base_rule_file",
            "base_bfunc_file": "base_bfunc_file",
            "base_pattern_file": "base_pattern_file",
            "base_mfunc_file": "base_mfunc_file",
        });
        let ctx_server_config: CtxServerConfig = CtxServerConfig::ctx_server_config_init(value);
        println!("{:?}", ctx_server_config);
    }
}
