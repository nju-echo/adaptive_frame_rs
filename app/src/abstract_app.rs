use std::any::type_name;
use std::sync::{Arc, RwLock};

use common::structs::sensor_data::SensorData;
use common::SyncString;

use crate::abstract_app::get_value_thread::RwLockOptionGetValueThread;
use crate::app::APP;

pub mod get_value_thread;

pub type SyncClientAppName = Arc<String>;

pub struct AbstractApp {
    app_name: SyncClientAppName,
    app_description: SyncClientAppName,
    get_value_thread: RwLockOptionGetValueThread,
}

impl AbstractApp {
    pub fn new() -> Self {
        let app_name = Arc::new(type_name::<Self>().to_string());
        let app_description = Arc::new(format!("This is {} .", app_name));
        Self {
            app_name,
            app_description,
            get_value_thread: RwLock::new(None),
        }
    }

    pub fn new_and_log_set() -> Self {
        let app = Self::new();
        app.log_set();
        app
    }

    pub fn set_app_name(&mut self, app_name: SyncString) {
        self.app_name = app_name;
    }

    pub fn set_app_description(&mut self, app_description: SyncString) {
        self.app_description = app_description;
    }
}

impl APP for AbstractApp {
    fn get_msg(&self, _sensor_name: String, _value: SensorData) {}
    fn config_app(&mut self) {}

    fn super_reference(&self) -> &AbstractApp {
        self
    }

    fn start_get_value_thread(&self, udp_port: u16) {
        todo!()
    }

    fn stop_get_value_thread(&self) {
        todo!()
    }

    fn wait_for_thread(&self) {
        todo!()
    }

    /// get_app_name
    fn get_app_name(&self) -> SyncString {
        self.app_name.clone()
    }

    /// get_description
    fn get_description(&self) -> SyncString {
        self.app_description.clone()
    }
}
