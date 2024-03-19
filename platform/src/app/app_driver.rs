use std::fmt::Display;
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::{Arc, RwLock, RwLockReadGuard, Weak};

use log::{debug, error, info, trace};
use serde_json::error::Category::Eof;
use serde_json::{json, Value};
use thiserror::Error;

use common::socket::cmd_message_grp_ids::GroupId;
use common::socket::tcp::TCP;
use common::structs::enumeration::cmd_type::CmdType;
use common::structs::enumeration::sensor_mode::SensorMode;
use common::structs::enumeration::service_type::ServiceType;
use common::structs::service_config::ServiceConfig;
use common::structs::sync::synchronous_string::SynchronousString;
use common::structs::time_line::FrequencyType;
use common::SyncString;

use crate::app::app_driver::app_driver_tcp::AppDriverTCP;
use crate::app::app_mgr::{RwLockOptionSyncAppMgr, SyncAppName};
use crate::app::app_mgr_thread::{AppPort, IpString, SyncIpString, APP_MGR_THREAD};
use crate::pubsub::abstract_subscriber;
use crate::pubsub::abstract_subscriber::{AbstractSubscriber, SubscriberId};
use crate::pubsub::subscriber::Subscriber;
use crate::resource::actor_mgr::SyncActorName;
use crate::resource::sensor_mgr::SyncSensorName;

pub mod app_driver_tcp;

pub type RwLockOptionClientIp = RwLock<Option<IpString>>;
pub type RwLockOptionClientUdpPort = RwLock<Option<AppPort>>;
pub type RwLockGetMsgThreadState = RwLock<bool>;
pub type RwLockGroupID = RwLock<GroupId>;

pub type SyncAppDriver = Arc<AppDriver>;
pub type RwLockOptionWeakAppDriver = RwLock<Option<Weak<AppDriver>>>;

pub struct AppDriver {
    abstract_subscriber: AbstractSubscriber,
    tcp: AppDriverTCP,
    client_ip: RwLockOptionClientIp,
    client_udp_port: RwLockOptionClientUdpPort,
    grp_id: RwLockGroupID,
    get_msg_thread_state: RwLockGetMsgThreadState,
    app_mgr: RwLockOptionSyncAppMgr,
    _get_sensor_data: SynchronousString,
    _actor_cmd: SynchronousString,
}

#[derive(Error, Debug)]
pub enum AppDriverError {
    #[error("parse api get None err: {0}")]
    ParseApiGetNoneError(String),
    #[error("parse api mismatch err: {0}")]
    ParseApiMismatchError(#[from] serde_json::Error),
}

fn option_to_app_driver_error<'a, T: ?Sized>(
    option: Option<&'a T>,
    msg: &str,
) -> Result<&'a T, AppDriverError> {
    option.ok_or_else(|| AppDriverError::ParseApiGetNoneError(msg.to_string()))
}

//todo: in java while app drop tcp connection, it will throw null pointer exception, which conflict with its intended behavior: drop app manger
// in rust, with error handling, i will try to avoid this situation.( tcp.recv_result() return error, app_mgr will be dropped)

/// create a new AppDriver
impl AppDriver {
    fn new(stream: TcpStream, id: SubscriberId) -> Self {
        let abstract_subscriber = AbstractSubscriber::new(id);
        let tcp = AppDriverTCP::new(stream, false);
        let client_ip = RwLock::new(None);
        let client_udp_port = RwLock::new(None);
        let grp_id = RwLock::new(-1);
        let get_msg_thread_state = RwLock::new(false);
        let app_mgr = RwLock::new(None);
        let _get_sensor_data = SynchronousString::new();
        let _actor_cmd = SynchronousString::new();
        Self {
            abstract_subscriber,
            tcp,
            client_ip,
            client_udp_port,
            grp_id,
            get_msg_thread_state,
            app_mgr,
            _get_sensor_data,
            _actor_cmd,
        }
    }

    ///add to static set Abstract Subscriber objs
    pub fn add_to_subscriber_objs(stream: TcpStream) -> SyncAppDriver {
        let mut subscriber_objs = abstract_subscriber::get_objs()
            .write()
            .expect("get subscriber objs write lock fail");
        let app_driver = Arc::new(Self::new(stream, subscriber_objs.len() as SubscriberId));
        subscriber_objs.push(app_driver.clone());

        app_driver.tcp.set_app_driver_weak(&app_driver);

        trace!(
            "new app tcp connection {}: add to subscriber objs success",
            app_driver
                .tcp
                .get_socket()
                .peer_addr()
                .expect("get peer addr fail")
        );
        app_driver
    }
}

/// getter and setter
impl AppDriver {
    /// get client ip clone
    /// if client_ip is none, panic
    pub fn get_client_ip(&self) -> RwLockReadGuard<Option<IpString>> {
        self.client_ip.read().expect("read client ip fail")
    }

    /// get udp port clone
    ///
    pub fn get_udp_port(&self) -> AppPort {
        self.client_udp_port
            .read()
            .expect("read client udp port fail")
            .as_ref()
            .expect("client udp port is none")
            .clone()
    }
}

/// thread run
impl AppDriver {
    /// run
    pub fn run(driver: SyncAppDriver) {
        loop {
            if let Some(msg_from_client) = driver.tcp.recv() {
                trace!("{}", msg_from_client);
                //todo: notion java中应该所有线程都关闭才借书，注意一下getmsgthread
                let json_object: Value =
                    serde_json::from_str(&msg_from_client).expect("parse json fail");

                // log the message
                if driver.app_mgr.read().expect("read app mgr fail").is_some() {
                    info!(
                        "[{} -> Platform]: {}",
                        driver
                            .app_mgr
                            .read()
                            .expect("read app mgr fail")
                            .as_ref()
                            .expect("app mgr is none")
                            .get_app_name_clone(),
                        msg_from_client
                    );
                } else {
                    info!("[AppDriver -> Platform]: {}", msg_from_client)
                }

                // get and parse api
                let api = json_object["api"].as_str();
                let ret: Option<String> = match Self::parse_api(driver.clone(), api, &json_object) {
                    Ok(ret) => Some(ret),
                    Err(e) => {
                        error!("{} -> platform: Error: {}", msg_from_client, e);
                        None
                    }
                };

                // send result
                if let Some(ret) = &ret {
                    driver.tcp.send(&ret);
                } else {
                    //todo: in java, it will return null pointer exception, how to handle it in rust
                    panic!("ret is none")
                }

                // log platform to app
                if driver.app_mgr.read().expect("read app mgr fail").is_some() {
                    info!(
                        "[Platform -> {}]: {}",
                        driver
                            .app_mgr
                            .read()
                            .expect("read app mgr fail")
                            .as_ref()
                            .expect("app mgr is none")
                            .get_app_name_clone(),
                        ret.expect("ret is none")
                    );
                } else {
                    info!("[Platform -> AppDriver]: {}", ret.expect("ret is none"))
                }

                // disconnect break
                // the reason here can use expect is that api is none, it will panic above
                if api.expect("api is none").to_ascii_lowercase().as_str() == "disconnect" {
                    break;
                }
            } else {
                break;
            }
        }
        trace!(
            "app tcp close {}: run success",
            driver
                .tcp
                .get_socket()
                .peer_addr()
                .expect("get peer addr fail")
        );
        driver.tcp.close();
    }

    /// parse api and do the corresponding operation
    /// return a result of string
    fn parse_api(
        driver: SyncAppDriver,
        api: Option<&str>,
        json_object: &Value,
    ) -> Result<String, AppDriverError> {
        if let Some(api) = api {
            match api.to_ascii_lowercase().as_str() {
                "connect" => {
                    return Ok(driver.connect_platform());
                }
                "disconnect" => {
                    return Ok(driver.disconnect_platform());
                }
                "is_connected" => {
                    return Ok(driver.check_connect());
                }
                "register_app" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    return Ok(Self::register_app(
                        driver.clone(),
                        Arc::new(app_name.to_string()),
                    ));
                }
                "unregister_app" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    return Ok(driver.unregister_app(Arc::new(app_name.to_string())));
                }
                "get_supported_sensors" => {
                    return Ok(driver.get_supported_sensors());
                }
                "get_registered_sensors" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    return Ok(driver.get_registered_sensors(Arc::new(app_name.to_string())));
                }
                "get_registered_sensors_status" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    return Ok(driver.get_registered_sensors_status(Arc::new(app_name.to_string())));
                }
                "register_sensor" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    let sensor_name = option_to_app_driver_error(
                        json_object["sensor_name"].as_str(),
                        "sensor_name is none",
                    )?;
                    let sensor_mode: SensorMode =
                        serde_json::from_str(option_to_app_driver_error(
                            json_object["sensor_mode"].as_str(),
                            "sensor_mode is none",
                        )?)?;
                    let freq = option_to_app_driver_error(
                        json_object["freq"].as_u64().as_ref(),
                        "freq is none",
                    )?
                    .clone();
                    return Ok(driver.register_sensor(
                        Arc::new(app_name.to_string()),
                        Arc::new(sensor_name.to_string()),
                        sensor_mode,
                        freq as FrequencyType,
                    ));
                }
                "cancel_sensor" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    let sensor_name = option_to_app_driver_error(
                        json_object["sensor_name"].as_str(),
                        "sensor_name is none",
                    )?;
                    return Ok(driver.cancel_sensor(
                        &Arc::new(app_name.to_string()),
                        &Arc::new(sensor_name.to_string()),
                    ));
                }
                "cancel_all_sensors" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    return Ok(driver.cancel_all_sensors(&Arc::new(app_name.to_string())));
                }
                "get_sensor_data" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    let sensor_name = option_to_app_driver_error(
                        json_object["sensor_name"].as_str(),
                        "sensor_name is none",
                    )?;
                    return Ok(driver.get_sensor_data(
                        Arc::new(app_name.to_string()),
                        Arc::new(sensor_name.to_string()),
                    ));
                }
                "get_all_sensor_data" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    return Ok(driver.get_all_sensor_data(Arc::new(app_name.to_string())));
                }
                "get_msg_thread" => {
                    let cmd = serde_json::from_str(option_to_app_driver_error(
                        json_object["cmd"].as_str(),
                        "cmd is none",
                    )?)?;
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    return Ok(driver.get_msg_thread(Arc::new(app_name.to_string()), cmd));
                }
                "get_supported_actors" => {
                    return Ok(driver.get_supported_actors());
                }
                "get_registered_actors" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    return Ok(driver.get_registered_actors(Arc::new(app_name.to_string())));
                }
                "get_registered_actors_status" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    return Ok(driver.get_registered_actors_status(Arc::new(app_name.to_string())));
                }
                "register_actor" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    let actor_name = option_to_app_driver_error(
                        json_object["actor_name"].as_str(),
                        "actor_name is none",
                    )?;
                    return Ok(driver.register_actor(
                        Arc::new(app_name.to_string()),
                        Arc::new(actor_name.to_string()),
                    ));
                }
                "cancel_actor" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    let actor_name = option_to_app_driver_error(
                        json_object["actor_name"].as_str(),
                        "actor_name is none",
                    )?;
                    return Ok(driver.cancel_actor(
                        &Arc::new(app_name.to_string()),
                        &Arc::new(actor_name.to_string()),
                    ));
                }
                "cancel_all_actors" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    return Ok(driver.cancel_all_actors(&Arc::new(app_name.to_string())));
                }
                "set_actor_cmd" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    let actor_name = option_to_app_driver_error(
                        json_object["actor_name"].as_str(),
                        "actor_name is none",
                    )?;
                    let action = option_to_app_driver_error(
                        json_object["action"].as_str(),
                        "action is none",
                    )?;
                    return Ok(driver.set_actor_cmd(
                        Arc::new(app_name.to_string()),
                        Arc::new(actor_name.to_string()),
                        action.to_string(),
                    ));
                }
                "is_service_on" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    let service = serde_json::from_str(option_to_app_driver_error(
                        json_object["service_type"].as_str(),
                        "service is none",
                    )?)?;
                    return Ok(driver.is_service_on(Arc::new(app_name.to_string()), service));
                }
                "get_sensor_info" => {
                    let sensor_name = option_to_app_driver_error(
                        json_object["sensor_name"].as_str(),
                        "sensor_name is none",
                    )?;
                    return Ok(driver.get_sensor_info(Arc::new(sensor_name.to_string())));
                }
                "get_sensor_info_and_data" => {
                    let sensor_name = option_to_app_driver_error(
                        json_object["sensor_name"].as_str(),
                        "sensor_name is none",
                    )?;
                    return Ok(driver.get_sensor_info_and_data(Arc::new(sensor_name.to_string())));
                }
                "get_all_sensor_info" => {
                    return Ok(driver.get_supported_sensors());
                }
                "get_all_sensor_info_and_data" => {
                    return Ok(driver.get_all_sensor_info_and_data());
                }
                "get_actor_info" => {
                    let actor_name = option_to_app_driver_error(
                        json_object["actor_name"].as_str(),
                        "actor_name is none",
                    )?;
                    return Ok(driver.get_actor_info(Arc::new(actor_name.to_string())));
                }
                "get_all_actor_info" => {
                    return Ok(driver.get_supported_actors());
                }
                "get_app_info" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    return Ok(driver.get_app_info(Arc::new(app_name.to_string())));
                }
                "get_all_app_info" => {
                    return Ok(driver.get_all_app_info());
                }
                "get_service_info" => {
                    let service = serde_json::from_str(option_to_app_driver_error(
                        json_object["service_type"].as_str(),
                        "service is none",
                    )?)?;
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    return Ok(driver.get_service_info(Arc::new(app_name.to_string()), service));
                }
                "get_all_service_info" => {
                    let app_name = option_to_app_driver_error(
                        json_object["app_name"].as_str(),
                        "app_name is none",
                    )?;
                    return Ok(driver.get_all_service_info(Arc::new(app_name.to_string())));
                }

                "start_service" => {
                    todo!()
                }
                "stop_service" => {
                    todo!()
                }
                "service-call" => {
                    todo!()
                }
                "inv_monitor" => {
                    todo!()
                }
                "inv_is_monitored" => {
                    todo!()
                }
                "inv_check" => {
                    todo!()
                }
                "inv_save" => {
                    todo!()
                }
                "inv_load" => {
                    todo!()
                }
                "inv_check_generated" => {
                    todo!()
                }

                "db_create" => {
                    todo!()
                }
                "db_drop" => {
                    todo!()
                }
                "db_insert" => {
                    todo!()
                }
                "db_inserts" => {
                    todo!()
                }
                "db_update" => {
                    todo!()
                }
                "db_delete" => {
                    todo!()
                }
                "db_deletes" => {
                    todo!()
                }
                "db_row" => {
                    todo!()
                }
                "db_rows" => {
                    todo!()
                }
                "db_column" => {
                    todo!()
                }
                "db_columns" => {
                    todo!()
                }
                "db_cell" => {
                    todo!()
                }
                "db_header" => {
                    todo!()
                }
                "db_count" => {
                    todo!()
                }

                _ => {
                    return Err(AppDriverError::ParseApiGetNoneError(format!(
                        "api: {} is not supported",
                        api
                    )));
                }
            }
        } else {
            return Err(AppDriverError::ParseApiGetNoneError(
                "api is none".to_string(),
            ));
        }

        //todo
        Ok("".to_string())
    }
}

//todo : may be set sync to &

/// below is private function
/// used for run
/// platform control
impl AppDriver {
    /// connect platform
    /// return a string about connect state
    fn connect_platform(&self) -> String {
        let ret_json = json!({"state" : true});
        ret_json.to_string()
    }

    /// disconnect platform
    /// return a string about disconnect state
    fn disconnect_platform(&self) -> String {
        let app_option = self
            .app_mgr
            .read()
            .expect("read app mgr fail")
            .as_ref()
            .map(|app_mgr| app_mgr.clone());

        if let Some(app) = app_option {
            self.unregister_app(app.get_app_name_clone())
        } else {
            let ret_json = json!({"state" : true});
            ret_json.to_string()
        }
    }

    /// check connect
    /// return a string about connect state
    fn check_connect(&self) -> String {
        if let Some(app) = self.app_mgr.read().expect("read app mgr fail").as_ref() {
            let ret_json = json!({"state" : true});
            ret_json.to_string()
        } else {
            let ret_json = json!({"state" : false});
            ret_json.to_string()
        }
    }

    /// register app
    /// return a string about app info
    fn register_app(driver: SyncAppDriver, app_name: SyncAppName) -> String {
        // 限制了锁的作用范围，以避免死锁
        let app_option = driver
            .app_mgr
            .read()
            .expect("read app mgr fail")
            .as_ref()
            .map(|app_mgr| app_mgr.clone());

        if let Some(app) = app_option {
            trace!(
                "app tcp connection {}: register app fail, app {} is already registered",
                driver
                    .tcp
                    .get_socket()
                    .peer_addr()
                    .expect("get peer addr fail"),
                app_name
            );
            let ret_json = json!({"state" : false});
            ret_json.to_string()
        } else {
            driver
                .client_ip
                .write()
                .expect("write client ip fail")
                .replace(
                    driver
                        .tcp
                        .get_socket()
                        .peer_addr()
                        .expect("get peer addr fail")
                        .ip()
                        .to_string(),
                );
            let udp_port = APP_MGR_THREAD.get_new_app_port(&driver.tcp.get_socket());
            driver
                .client_udp_port
                .write()
                .expect("write client udp port fail")
                .replace(udp_port);

            // set grp id and app mgr
            let grp_id = APP_MGR_THREAD.get_new_grp_id(app_name.clone());
            *driver.grp_id.write().expect("write grp id fail") = grp_id;

            driver
                .app_mgr
                .write()
                .expect("write app mgr fail")
                .replace(APP_MGR_THREAD.register_app_mgr(app_name.clone()));
            let app_mgr = driver.app_mgr.read().expect("read app mgr fail");
            app_mgr
                .as_ref()
                .expect("app mgr is none")
                .set_grp_id(grp_id);
            app_mgr
                .as_ref()
                .expect("app mgr is none")
                .set_app_driver(&driver);

            let ret_json = json!({"state" : true, "udp_port" : udp_port});
            ret_json.to_string()
        }
    }

    /// unregister app
    /// return a string about app info
    fn unregister_app(&self, app_name: SyncAppName) -> String {
        let app_option = self
            .app_mgr
            .read()
            .expect("read app mgr fail")
            .as_ref()
            .map(|app_mgr| app_mgr.clone());

        match app_option {
            Some(app) if app.get_app_name_clone().eq_ignore_ascii_case(&app_name) => {
                //todo:
                //self.cancel_all_sensors(&app_name);
                //self.cancel_all_actors(&app_name);

                //todo: remove app.database

                self.app_mgr.write().expect("write app mgr fail").take();

                APP_MGR_THREAD.unregister_app_mgr(&app_name);
                APP_MGR_THREAD.remove_app_port(self.tcp.get_socket(), self.get_udp_port());

                self.client_ip.write().expect("write client ip fail").take();
                self.client_udp_port
                    .write()
                    .expect("write client udp port fail")
                    .take();
                //todo: check msg thread later
                *self
                    .get_msg_thread_state
                    .write()
                    .expect("write get msg thread state fail") = false;
                //todo: does grd id be set to -1? :question: can we do set actor for other app?

                //todo: service call

                let ret_json = json!({"state" : true});
                ret_json.to_string()
            }
            _ => {
                let ret_json = json!({"state" : false});
                ret_json.to_string()
            }
        }
    }
}

/// sensor related
impl AppDriver {
    /// get supported sensors
    /// return a string about sensor info
    fn get_supported_sensors(&self) -> String {
        todo!()
    }

    /// get registered sensors
    /// return a string about sensor name of app
    fn get_registered_sensors(&self, app_name: SyncAppName) -> String {
        todo!()
    }

    /// get registered sensors status
    /// return a string about sensor status of app
    fn get_registered_sensors_status(&self, app_name: SyncAppName) -> String {
        todo!()
    }

    /// register sensor inner
    fn _register_sensor(
        &self,
        app_name: SyncAppName,
        sensor_name: SyncSensorName,
        sensor_mode: SensorMode,
        freq: FrequencyType,
    ) {
        todo!()
    }

    /// register sensor
    /// return a string about whether register sensor success
    fn register_sensor(
        &self,
        app_name: SyncAppName,
        sensor_name: SyncSensorName,
        sensor_mode: SensorMode,
        freq: FrequencyType,
    ) -> String {
        todo!()
    }

    /// cancel sensor inner
    fn _cancel_sensor(&self, app_name: &SyncAppName, sensor_name: &SyncSensorName) {
        todo!()
    }

    /// cancel sensor
    /// return a string about whether cancel sensor success
    fn cancel_sensor(&self, app_name: &SyncAppName, sensor_name: &SyncSensorName) -> String {
        todo!()
    }

    /// cancel all sensors of an app
    /// return a string about whether cancel all sensors success
    fn cancel_all_sensors(&self, app_name: &SyncAppName) -> String {
        todo!()
    }

    /// get sensor data
    /// return a string about sensor data
    fn get_sensor_data(&self, app_name: SyncAppName, sensor_name: SyncSensorName) -> String {
        todo!()
    }

    /// get all sensor data
    /// return a string about all sensor data
    fn get_all_sensor_data(&self, app_name: SyncAppName) -> String {
        todo!()
    }

    /// get msg thread
    /// open or close get msg thread depend on cmd
    /// return a string about get msg thread state
    fn get_msg_thread(&self, app_name: SyncAppName, cmd: CmdType) -> String {
        todo!()
    }
}

/// actor related
impl AppDriver {
    /// get supported actors
    /// return a string about actor info
    fn get_supported_actors(&self) -> String {
        todo!()
    }

    /// get registered actors
    /// return a string about actor name of app
    fn get_registered_actors(&self, app_name: SyncAppName) -> String {
        todo!()
    }

    /// get registered actors status
    /// return a string about actor status of app
    fn get_registered_actors_status(&self, app_name: SyncAppName) -> String {
        todo!()
    }

    /// register actor
    /// return a string about whether register actor success
    fn register_actor(&self, app_name: SyncAppName, actor_name: SyncActorName) -> String {
        todo!()
    }

    /// cancel actor
    /// return a string about whether cancel actor success
    fn cancel_actor(&self, app_name: &SyncAppName, actor_name: &SyncActorName) -> String {
        todo!()
    }

    /// cancel all actors of an app
    /// return a string about whether cancel all actors success
    fn cancel_all_actors(&self, app_name: &SyncAppName) -> String {
        todo!()
    }

    /// set actor cmd
    /// return a string about whether set actor cmd success
    fn set_actor_cmd(
        &self,
        app_name: SyncAppName,
        actor_name: SyncActorName,
        action: String,
    ) -> String {
        todo!()
    }
}

/// info related
impl AppDriver {
    /// below is for the whole platform, does not depend on app

    /// get Sensor info
    /// return a string about sensor info
    fn get_sensor_info(&self, sensor_name: SyncSensorName) -> String {
        todo!()
    }

    /// get sensor info and data
    /// return a string about sensor info and data
    fn get_sensor_info_and_data(&self, sensor_name: SyncSensorName) -> String {
        todo!()
    }

    /// get all sensor info and data
    /// return a string about all sensor info and data
    fn get_all_sensor_info_and_data(&self) -> String {
        todo!()
    }

    /// get actor info
    /// return a string about actor info
    fn get_actor_info(&self, actor_name: SyncActorName) -> String {
        todo!()
    }

    //todo: why not get all actor info

    /// get app info
    /// return a string about app info
    fn get_app_info(&self, app_name: SyncAppName) -> String {
        todo!()
    }

    /// get all app info
    /// return a string about all app info
    fn get_all_app_info(&self) -> String {
        todo!()
    }

    /// get service info
    /// return a string about service info
    fn get_service_info(&self, app_name: SyncAppName, service: ServiceType) -> String {
        todo!()
    }

    /// get all service info
    /// return a string about all service info
    fn get_all_service_info(&self, app_name: SyncAppName) -> String {
        todo!()
    }
}

/// service related: todo
impl AppDriver {
    /// is service on
    /// return a string about service state
    fn is_service_on(&self, app_name: SyncAppName, service: ServiceType) -> String {
        todo!()
    }

    /// service call
    /// return a string about service call state
    fn service_call<T: ServiceConfig>(
        &self,
        app_name: SyncAppName,
        service: ServiceType,
        cmd: CmdType,
        config: T,
    ) -> String {
        todo!()
    }

    /// monitor
    /// return a string about monitor state
    fn monitor(&self, app_name: SyncAppName, objs: Value) -> String {
        todo!()
    }

    /// is monitored
    /// return a string about monitor state
    fn is_monitored(&self, app_name: SyncAppName, objs: Value) -> String {
        todo!()
    }

    /// check
    /// return a string about check state
    fn check(&self, app_name: SyncAppName, objs: Value) -> String {
        todo!()
    }

    /// save
    /// return a string about save content
    fn save(&self, app_name: SyncAppName) -> String {
        todo!()
    }

    /// load
    /// return a string about load state
    fn load(&self, app_name: SyncAppName, content: String) -> String {
        todo!()
    }

    /// check generated
    /// return a string about check generated state
    fn check_generated(&self, app_name: SyncAppName) -> String {
        todo!()
    }
}

/// database related: todo
impl AppDriver {
    /// database create
    /// return a string about database create state
    fn database_create(
        &self,
        app_name: SyncAppName,
        table_name: String,
        primary_key: String,
        header: Vec<String>,
        row_limit: i32,
    ) -> String {
        todo!()
    }

    /// database drop
    /// return a string about database drop state
    fn database_drop(&self, app_name: SyncAppName, table_name: String) -> String {
        todo!()
    }

    /// database insert
    /// return a string about database insert state\
    fn database_insert() -> String {
        todo!()
    }

    //todo: add other database operation
}

/// display trait
impl Display for AppDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// subscriber trait
impl Subscriber for AppDriver {
    fn super_reference(&self) -> &AbstractSubscriber {
        &self.abstract_subscriber
    }

    fn on_message(&self, channel: SyncString, msg: SyncString) {
        todo!()
    }
}
