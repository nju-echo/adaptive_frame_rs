use std::sync::{Arc, RwLock};

use env_logger::Builder;

use common::structs::sensor_data::SensorData;
use common::SyncString;

use crate::abstract_app::AbstractApp;

pub type RwLockOptionSyncAbstractApp = RwLock<Option<SyncAbstractApp>>;
pub type SyncAbstractApp = Arc<dyn APP>;

pub trait APP: Send + Sync {
    /// get msg
    /// used when sensor send data
    fn get_msg(&self, sensor_name: String, value: SensorData);
    /// config app
    /// used when app is created
    fn config_app(&mut self);

    /// super reference
    fn super_reference(&self) -> &AbstractApp;

    //todo: question: why we can not use new() -> Self here with usage of trait object.(object safety?)

    /// below do not need to implement by self in common

    /// log set
    /// used when app is created
    fn log_set(&self) {
        //todo: log set
        // below is a simple version, please replace it with a real log set
        Builder::new().parse_filters("trace").init();
    }

    /// start get value thread
    fn start_get_value_thread(&self, udp_port: u16) {
        self.super_reference().start_get_value_thread(udp_port);
    }

    /// stop get value thread
    fn stop_get_value_thread(&self) {
        self.super_reference().stop_get_value_thread();
    }

    /// wait for thread
    /// this purpose is to wait for get value thread to finish just like java
    fn wait_for_thread(&self) {
        self.super_reference().wait_for_thread();
    }

    /// get app_name
    fn get_app_name(&self) -> SyncString {
        self.super_reference().get_app_name()
    }

    /// get description
    fn get_description(&self) -> SyncString {
        self.super_reference().get_description()
    }
}
