use std::sync::{Arc, RwLock};

///sensor_mgr is a struct that manages the lifecycle of sensors.
/// should be protected by a RwLock.
/// resoure_driver and app_driver both can own a write lock of sensor_mgr temporarily.

pub type SyncSensorMgr = Arc<RwLock<SensorMgr>>;

#[derive(Debug, Eq, PartialEq)]
pub struct SensorMgr {
    //TODO
}
