use std::collections::HashMap;
use std::error::Error;
use std::net::TcpStream;
use std::os::linux::raw::stat;
use std::sync::{Arc, RwLock, RwLockReadGuard};

use log::{info, trace};
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use thiserror::Error;

use common::socket::tcp::TCP;
use common::structs::actor_info::ActorInfo;
use common::structs::app_info::AppInfo;
use common::structs::enumeration::cmd_type::CmdType;
use common::structs::enumeration::sensor_mode::SensorMode;
use common::structs::enumeration::service_type::ServiceType;
use common::structs::sensor_data::SensorData;
use common::structs::sensor_info::SensorInfo;
use common::structs::service_config::ServiceConfig;
use common::structs::service_info::ServiceInfo;
use common::structs::service_result::ServiceResult;
use common::structs::time_line::FrequencyType;

use crate::abstract_app::SyncClientAppName;
use crate::app::{RwLockOptionSyncAbstractApp, SyncAbstractApp};
use crate::app_remote_connector::app_remote_connector_tcp::{
    AppRemoteConnectorTCP, RwLockOptionAppRemoteConnectorTCP,
};

pub mod app_remote_connector_tcp;

pub type RwLockOptionUdpPort = RwLock<Option<u16>>;
pub type SyncAppRemoteConnector = Arc<AppRemoteConnector>;

pub struct AppRemoteConnector {
    tcp: RwLockOptionAppRemoteConnectorTCP,
    udp_port: RwLockOptionUdpPort,
    app: RwLockOptionSyncAbstractApp,
}

#[derive(Error, Debug)]
pub enum PlatformError {
    #[error("connect platform fail")]
    ConnectPlatformFail,
}

impl AppRemoteConnector {
    pub fn new() -> Self {
        Self {
            tcp: RwLock::new(None),
            udp_port: RwLock::new(None),
            app: RwLock::new(None),
        }
    }

    /// get tcp
    /// return a read lock guard of tcp
    pub fn get_tcp(&self) -> RwLockReadGuard<Option<AppRemoteConnectorTCP>> {
        self.tcp.read().expect("tcp read lock fail")
    }

    /// get app name clone
    /// return app name
    pub fn get_app_name_clone(&self) -> SyncClientAppName {
        self.app
            .read()
            .expect("app read lock fail")
            .as_ref()
            .expect("app is none")
            .get_app_name()
    }

    /// raw
    /// tcp send cmd and return recv
    /// used by web server
    pub fn raw(&self, cmd: &str) -> Option<String> {
        self.get_tcp().as_ref().expect("tcp is none").send(cmd);
        let ret = self.get_tcp().as_ref().expect("tcp is none").recv();
        trace!("raw: {} -> {:?}", cmd, ret);
        ret
    }
}

/// app remote connector instance
pub static APP_REMOTE_CONNECTOR: Lazy<AppRemoteConnector> = Lazy::new(|| AppRemoteConnector::new());

fn check_return_string(ret: &str) -> Result<bool, Box<dyn Error>> {
    let jo: Value = serde_json::from_str(ret)?;
    let state = jo
        .get("state")
        .ok_or("no state")?
        .as_bool()
        .ok_or("state is not bool")?;
    Ok(state)
}

impl AppRemoteConnector {
    /// send
    /// send a json string to platform
    /// return a result to indicate whether send success
    fn send(&self, jo: &str) -> Result<(), PlatformError> {
        match self
            .tcp
            .read()
            .expect("tcp read lock fail")
            .as_ref()
            .expect("tcp is none")
            .send(jo)
        {
            true => Ok(()),
            false => Err(PlatformError::ConnectPlatformFail),
        }
    }

    /// recv
    /// recv a json string from platform
    /// return a result to indicate whether recv success
    /// if recv success, it will return a json string
    /// if recv fail, it will return a PlatformError
    fn recv(&self) -> Result<String, PlatformError> {
        match self
            .tcp
            .read()
            .expect("tcp read lock fail")
            .as_ref()
            .expect("tcp is none")
            .recv()
        {
            Some(s) => Ok(s),
            None => Err(PlatformError::ConnectPlatformFail),
        }
    }
}

/// below is platform related
impl AppRemoteConnector {
    /// connect platform
    /// use ip and port to connect platform
    /// return a bool to indicate whether connect success
    pub fn connect_platform(&self, ip: String, port: u16) -> Result<bool, PlatformError> {
        let jo: Value = json!({"api": "connect"});
        let mut state = false;

        match TcpStream::connect(format!("{}:{}", ip, port)) {
            Ok(stream) => {
                let tcp = AppRemoteConnectorTCP::new(stream, true);
                *self.tcp.write().expect("tcp write lock fail") = Some(tcp);

                self.send(&jo.to_string())?;
                let recv = self.recv()?;
                match check_return_string(&recv) {
                    Ok(s) => {
                        state = s;
                    }
                    Err(e) => {
                        trace!("connect platform fail: {}", e);
                    }
                }
            }
            Err(e) => {
                trace!("connect platform fail: {}", e);
            }
        }
        info!(
            "[AppConnector]: connect platform({}, {}) -> {}",
            ip, port, state
        );
        Ok(state)
    }

    /// disconnect platform
    /// return a bool to indicate whether disconnect success
    pub fn disconnect_platform(&self) -> Result<bool, PlatformError> {
        let jo: Value = json!({"api": "disconnect"});
        let mut state = false;

        self.send(&jo.to_string())?;
        let recv = self.recv()?;

        match check_return_string(&recv) {
            Ok(s) => {
                state = s;
            }
            Err(e) => {
                trace!("disconnect platform fail: {}", e);
            }
        }

        info!("[AppConnector]: disconnect platform -> {}", state);

        if state && (self.app.read().expect("app read lock fail").is_some()) {
            self.app
                .read()
                .expect("app read lock fail")
                .as_ref()
                .expect("app is none")
                .stop_get_value_thread();
            self.app.write().expect("app write lock fail").take();
        }

        Ok(state)
    }

    /// check whether connected
    /// used after connect platform
    /// return a bool to indicate whether connected
    pub fn check_connected(&self) -> Result<bool, PlatformError> {
        let jo: Value = json!({"api": "is_connected"});
        let mut state = false;

        self.send(&jo.to_string())?;
        let recv = self.recv()?;

        match check_return_string(&recv) {
            Ok(s) => {
                state = s;
            }
            Err(e) => {
                trace!("check connected fail: {}", e);
            }
        }

        info!("[AppConnector]: check connected -> {}", state);
        Ok(state)
    }

    //todo: get database instance

    /// register app
    /// give a trait object of app to app remote connector
    /// it will store app in app field
    /// return a bool to indicate whether register success
    pub fn register_app(&self, app: SyncAbstractApp) -> Result<bool, PlatformError> {
        let mut jo: Value = json!({"api": "register_app"});
        let app_name = app.get_app_name();
        jo["app_name"] = Value::String(app_name.to_string());

        self.send(&jo.to_string())?;

        let mut state = false;

        let recv = self.recv()?;

        if let Ok(ret_json) = serde_json::from_str::<Value>(&recv) {
            if let Some(ret_state) = ret_json.get("state").and_then(|s| s.as_bool()) {
                state = ret_state;
                if state {
                    *self.app.write().expect("app write lock fail") = Some(app);
                    *self.udp_port.write().expect("udp_port write lock fail") = Some(
                        ret_json
                            .get("udp_port")
                            .and_then(|s| s.as_u64())
                            .unwrap_or(0) as u16,
                    );
                }
            }
        }

        info!("[AppConnector]: register app({}) -> {}", app_name, state);
        Ok(state)
    }

    /// unregister app
    /// it will remove app in app field
    /// return a bool to indicate whether unregister success
    pub fn unregister_app(&self, app: SyncAbstractApp) -> Result<bool, PlatformError> {
        let app_name = app.get_app_name().to_string();
        let jo: Value = json!({"api": "unregister_app", "app_name": app_name});

        self.send(&jo.to_string())?;

        let mut state = false;
        let recv = self.recv()?;

        if let Ok(ret_json) = serde_json::from_str::<Value>(&recv) {
            state = ret_json
                .get("state")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            if state {
                self.app
                    .read()
                    .expect("app read lock fail")
                    .as_ref()
                    .expect("app is none")
                    .stop_get_value_thread();
                self.app.write().expect("app write lock fail").take();
                self.udp_port
                    .write()
                    .expect("udp_port write lock fail")
                    .take();
            }
        }

        info!("unregister app({}) -> {}", app_name, state);
        Ok(state)
    }
}

/// below is sensor related
impl AppRemoteConnector {
    /// get supported sensors
    /// return a map of supported sensors name and sensor info
    pub fn get_supported_sensors(&self) -> Result<HashMap<String, SensorInfo>, PlatformError> {
        todo!()
    }

    /// get registered sensors
    /// return a map of registered sensors name and sensor info
    pub fn get_registered_sensors(&self) -> Result<HashMap<String, SensorInfo>, PlatformError> {
        todo!()
    }

    /// get registered sensors status
    /// return true if sensors status is on
    pub fn get_registered_sensors_status(&self) -> Result<bool, PlatformError> {
        todo!()
    }

    /// register sensor
    /// use sensor name to register sensor
    /// mode is used to indicate whether sensor is Active or Passive
    /// frequency is used to indicate how often Passive sensor send data to app
    /// return a bool to indicate whether register success
    pub fn register_sensor(
        &self,
        sensor_name: String,
        mode: SensorMode,
        frequency: FrequencyType,
    ) -> Result<bool, PlatformError> {
        todo!()
    }

    /// cancel sensor
    /// use sensor name to cancel sensor
    /// return a bool to indicate whether cancel success
    pub fn cancel_sensor(&self, sensor_name: String) -> Result<bool, PlatformError> {
        todo!()
    }

    /// cancel all sensors
    /// return a bool to indicate whether cancel all success
    pub fn cancel_all_sensors(&self) -> Result<bool, PlatformError> {
        todo!()
    }

    /// get sensor data
    /// use sensor name to get sensor data
    /// return sensor data
    pub fn get_sensor_data(&self, sensor_name: String) -> Result<SensorData, PlatformError> {
        todo!()
    }

    /// get all sensor data
    /// return a map of sensor name and sensor data
    /// if sensor data is none, it will be none
    pub fn get_all_sensor_data(&self) -> Result<HashMap<String, SensorData>, PlatformError> {
        todo!()
    }

    /// get msg thread
    /// use cmd to set msg thread
    /// return a bool to indicate whether you get msg thread success
    pub fn get_msg_thread(&self, cmd: CmdType) -> Result<bool, PlatformError> {
        todo!()
    }
}

/// below is actor related
impl AppRemoteConnector {
    /// get supported actors
    /// return a map of supported actors name and actor info
    pub fn get_supported_actors(&self) -> Result<HashMap<String, ActorInfo>, PlatformError> {
        todo!()
    }

    /// get registered actors
    /// return a map of registered actors name and actor info
    pub fn get_registered_actors(&self) -> Result<HashMap<String, ActorInfo>, PlatformError> {
        todo!()
    }

    /// get registered actors status
    /// return true if actors status is on
    pub fn get_registered_actors_status(&self) -> Result<bool, PlatformError> {
        todo!()
    }

    /// register actor
    /// use actor name to register actor
    /// return a bool to indicate whether register success
    pub fn register_actor(&self, actor_name: String) -> Result<bool, PlatformError> {
        todo!()
    }

    /// cancel actor
    /// use actor name to cancel actor
    /// return a bool to indicate whether cancel success
    pub fn cancel_actor(&self, actor_name: String) -> Result<bool, PlatformError> {
        todo!()
    }

    /// cancel all actors
    /// return a bool to indicate whether cancel all success
    pub fn cancel_all_actors(&self) -> Result<bool, PlatformError> {
        todo!()
    }

    /// set actor cmd
    /// use actor name and cmd to set actor cmd
    /// return a bool to indicate whether set actor cmd success
    pub fn set_actor_cmd(&self, actor_name: String, action: String) -> Result<bool, PlatformError> {
        todo!()
    }
}

/// below is info related
impl AppRemoteConnector {
    /// get sensor info
    /// use sensor name to get sensor info
    /// return sensor info
    pub fn get_sensor_info(&self, sensor_name: String) -> Result<SensorInfo, PlatformError> {
        todo!()
    }

    /// get all sensor info
    /// return a map of sensor name and sensor info
    pub fn get_all_sensor_info(&self) -> Result<HashMap<String, SensorInfo>, PlatformError> {
        todo!()
    }

    /// get sensor info and data
    /// use sensor name to get sensor info and data
    /// return a string to indicate sensor info and data
    pub fn get_sensor_info_and_data(&self, sensor_name: String) -> Result<String, PlatformError> {
        todo!()
    }

    /// get all sensor info and data
    /// return a map of sensor name and sensor info and data
    pub fn get_all_sensor_info_and_data(&self) -> Result<HashMap<String, String>, PlatformError> {
        todo!()
    }

    /// get actor info
    /// use actor name to get actor info
    /// return actor info
    pub fn get_actor_info(&self, actor_name: String) -> Result<ActorInfo, PlatformError> {
        todo!()
    }

    /// get all actor info
    /// return a map of actor name and actor info
    pub fn get_all_actor_info(&self) -> Result<HashMap<String, ActorInfo>, PlatformError> {
        todo!()
    }

    /// get app info
    /// return app info
    pub fn get_app_info(&self) -> Result<AppInfo, PlatformError> {
        todo!()
    }

    /// get all app info
    /// return a map of app name and app info
    pub fn get_all_app_info(&self) -> Result<HashMap<String, AppInfo>, PlatformError> {
        todo!()
    }

    /// get service info
    /// return service info
    pub fn get_service_info<T, F>(&self) -> Result<ServiceInfo<T, F>, PlatformError>
    where
        T: ServiceConfig,
        F: ServiceResult,
    {
        todo!()
    }
}

/// below is service related
impl AppRemoteConnector {
    /// is service on
    /// return a bool to indicate whether chose service is on
    pub fn is_service_on(&self, service: ServiceType) -> Result<bool, PlatformError> {
        todo!()
    }

    /// start service
    /// give service type and service config to start service
    /// return a bool to indicate whether start service success
    pub fn service_start<T: ServiceConfig>(
        &self,
        service: ServiceType,
        config: T,
    ) -> Result<bool, PlatformError> {
        todo!()
    }

    /// stop service
    /// give service type to stop service
    /// return a bool to indicate whether stop service success
    pub fn service_stop(&self, service: ServiceType) -> Result<bool, PlatformError> {
        todo!()
    }

    /// call service
    /// give service type, cmd and config to call service
    /// return a bool to indicate whether call service success
    pub fn service_call<T: ServiceConfig>(
        &self,
        service: ServiceType,
        cmd: CmdType,
        config: T,
    ) -> Result<bool, PlatformError> {
        todo!()
    }
}
