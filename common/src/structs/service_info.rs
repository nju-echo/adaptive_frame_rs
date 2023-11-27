use serde_json::Value;

use crate::structs::enumeration::service_type::ServiceType;
use crate::structs::service_config::ServiceConfig;
use crate::structs::service_result::ServiceResult;
use crate::structs::state::State;

#[derive(Debug)]
pub struct ServiceInfo<TConfig, TResult>
where
    TConfig: ServiceConfig,
    TResult: ServiceResult,
{
    pub service_type: ServiceType,
    pub app_name: String,
    pub state: State,
    pub config: Option<TConfig>,
    pub result: Option<TResult>,
    pub jo: Option<Value>,
}
