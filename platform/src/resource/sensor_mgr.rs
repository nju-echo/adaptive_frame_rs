use std::fmt::Display;
use std::sync::{Arc, RwLock, Weak};

use serde::{Deserialize, Serialize};

use common::structs::sensor_info::SensorInfo;
use common::structs::time_line::SyncCondTimeLine;
use common::structs::value_type::ValueType;

use crate::app::app_mgr::{SyncAppName, WeakAppMap};
use crate::resource::sensor_mgr::value_thread::RwLockOptionValueThread;
use crate::resource::RwlockAlive;

pub mod value_thread;

///sensor_mgr is a struct that manages the lifecycle of sensors.
/// should be protected by a RwLock.
/// resoure_driver and app_driver both can own a write lock of sensor_mgr temporarily.

pub type SyncSensorMgr = Arc<SensorMgr>;
pub type SyncSensorName = Arc<String>;
pub type WeakSensorMgr = Weak<SensorMgr>;
pub type RwLockOptionSyncSensorMgr = RwLock<Option<SyncSensorMgr>>;

//todo: should add a Mutex and condvar to timeline
// it can be a bottleneck although it only has a thread read and many thread rare write

type SyncFieldNames = Arc<Vec<String>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorMgr {
    #[serde(rename = "value_type", default = "default_sensor_type")]
    sensor_type: ValueType,
    // shared data with value thread
    #[serde(rename = "name")]
    sensor_name: SyncSensorName,
    #[serde(rename = "fields", default = "default_fields_name")]
    fields_name: SyncFieldNames,
    #[serde(default = "default_is_alive")]
    is_alive: RwlockAlive,
    #[serde(default = "default_min_value_freq")]
    min_value_freq: u64,
    #[serde(default = "default_max_value_freq")]
    max_value_freq: u64,
    #[serde(skip)]
    get_value_thread: RwLockOptionValueThread,
    #[serde(skip)]
    apps: WeakAppMap,
    /// share data with value thread
    #[serde(skip)]
    time_line: SyncCondTimeLine,
}

fn default_sensor_type() -> ValueType {
    ValueType::String
}

fn default_fields_name() -> SyncFieldNames {
    Arc::new(vec!["default".to_string()])
}

fn default_is_alive() -> RwlockAlive {
    RwLock::new(true)
}

fn default_min_value_freq() -> u64 {
    1
}

fn default_max_value_freq() -> u64 {
    1000
}

impl SensorMgr {
    /// get clone of time_line
    pub fn get_time_line_clone(&self) -> SyncCondTimeLine {
        self.time_line.clone()
    }

    /// check value freq
    pub fn check_value_freq(&self, value_freq: u64) -> bool {
        value_freq >= self.min_value_freq && value_freq <= self.max_value_freq
    }

    /// get sensor type
    pub fn get_sensor_type_clone(&self) -> ValueType {
        self.sensor_type
    }

    /// get sensor name
    pub fn get_sensor_name(&self) -> &SyncSensorName {
        &self.sensor_name
    }

    /// get fields name
    pub fn get_fields_name(&self) -> &SyncFieldNames {
        &self.fields_name
    }

    /// is_alive function
    /// return true if the sensor is alive
    pub fn is_alive(&self) -> bool {
        todo!()
    }

    /// set_alive function
    /// set the sensor alive or not
    /// used when register or error
    pub fn set_alive(&self, alive: bool) {
        todo!()
    }

    /// is get value running
    pub fn is_get_value_running(&self) -> bool {
        self.get_value_thread
            .read()
            .expect("read get value thread fail")
            .is_some()
    }

    /// start get value
    pub fn start_get_value(&self) {
        todo!()
    }

    /// stop get value
    pub fn stop_get_value(&self) {
        todo!()
    }

    /// get apps
    /// todo: may add more other function instead of use it
    pub fn get_apps(&self) -> &WeakAppMap {
        todo!()
    }

    /// get apps names
    pub fn get_app_names_vec(&self) -> Vec<SyncAppName> {
        todo!()
    }

    /// generate sensor information
    pub fn create_sensor_info(&self) -> SensorInfo {
        todo!()
    }

    //todo: how to implement thread function
    // does it need to use Weak, or just shared data ?
}

impl Display for SensorMgr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
