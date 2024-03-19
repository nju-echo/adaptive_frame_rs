use std::fmt::Display;
use std::sync::{Arc, RwLock, Weak};

use dashmap::{DashMap, DashSet};

use common::socket::cmd_message_grp_ids::GroupId;
use common::structs::ctx_service_config::CtxServiceConfig;
use common::structs::sync::synchronous_string::SynchronousString;

use crate::app::app_driver::{RwLockGroupID, RwLockOptionWeakAppDriver, SyncAppDriver};
use crate::resource::actor_mgr::SyncActorNameSet;
use crate::resource::sensor_mgr::SyncSensorNameSet;

pub type SyncAppMgr = Arc<AppMgr>;
pub type RwLockOptionSyncAppMgr = RwLock<Option<SyncAppMgr>>;
pub type WeakAppMgr = Weak<AppMgr>;
pub type SyncAppName = Arc<String>;
pub type SyncAppNameSet = DashMap<SyncAppName, WeakAppMgr>;
pub type ChannelRequestSet = DashMap<String, SynchronousString>;

pub struct AppMgr {
    app_name: SyncAppName,
    grp_id: RwLockGroupID,
    sensors: SyncSensorNameSet,
    actors: SyncActorNameSet,
    //use app driver for udp port. This is the reason why
    app_driver: RwLockOptionWeakAppDriver,
    request_map: ChannelRequestSet,
    //todo:add database, ctx service and inv daikon related fields
}

impl AppMgr {
    /// new
    /// use app name to new an app mgr
    /// return a new app mgr
    pub fn new(app_name: SyncAppName) -> Self {
        let grp_id = RwLock::new(0);
        let sensors = DashSet::new();
        let actors = DashSet::new();
        let app_driver = RwLock::new(None);
        let request_map = DashMap::new();
        Self {
            app_name,
            grp_id,
            sensors,
            actors,
            app_driver,
            request_map,
        }
    }

    /// get request map
    /// used for app to get request from active resource
    /// return a reference of request map
    pub fn get_request_map(&self) -> &ChannelRequestSet {
        &self.request_map
    }

    /// get grp id clone
    /// return a clone of grp id
    pub fn get_grp_id_clone(&self) -> GroupId {
        todo!("get grp id clone")
    }

    /// set grp id
    pub fn set_grp_id(&self, grp_id: GroupId) {
        *self.grp_id.write().expect("set grp id fail") = grp_id;
    }

    /// get app name clone
    /// return a clone of app name
    pub fn get_app_name_clone(&self) -> SyncAppName {
        self.app_name.clone()
    }

    /// get sensors
    /// return a reference of sensors
    pub fn get_sensors(&self) -> &SyncSensorNameSet {
        &self.sensors
    }

    /// get sensor names vec
    /// return a vec of sensor names
    pub fn get_sensor_names_vec(&self) -> Vec<SyncAppName> {
        todo!("get sensor names vec")
    }

    /// add sensor
    /// if contain sensor, return false
    /// else add sensor and return true
    pub fn add_sensor(&self, sensor_name: SyncAppName) -> bool {
        todo!("add sensor")
    }

    /// remove sensor
    /// if contain sensor, remove sensor and return true
    /// else return false
    pub fn remove_sensor(&self, sensor_name: &SyncAppName) -> bool {
        todo!("remove sensor")
    }

    /// get actors
    /// return a reference of actors
    pub fn get_actors(&self) -> &SyncActorNameSet {
        &self.actors
    }

    /// get actor names vec
    /// return a vec of actor names
    pub fn get_actor_names_vec(&self) -> Vec<SyncAppName> {
        todo!("get actor names vec")
    }

    /// add actor
    /// if contain actor, return false
    /// else add actor and return true
    pub fn add_actor(&self, actor_name: SyncAppName) -> bool {
        todo!("add actor")
    }

    /// remove actor
    /// if contain actor, remove actor and return true
    /// else return false
    pub fn remove_actor(&self, actor_name: &SyncAppName) -> bool {
        todo!("remove actor")
    }

    /// set app driver
    /// set app driver for app mgr
    pub fn set_app_driver(&self, app_driver: &SyncAppDriver) {
        *self.app_driver.write().expect("set app driver fail") = Some(Arc::downgrade(app_driver));
    }

    /// get app driver
    /// return a reference of app driver
    /// if app driver is none, return none
    /// else return a reference of sync app driver
    pub fn get_app_driver(&self) -> &SyncAppDriver {
        todo!("get app driver")
    }

    //below is ctx service related

    /// set ctx service config
    /// an init function for ctx service of app mgr
    pub fn set_ctx_service_config(&self, ctx_service_config: CtxServiceConfig) {
        todo!()
    }

    //todo: add other ctx service related function

    //below is inv daikon related
    //todo: add inv daikon related function

    //below is database related
    //todo: get database and remove database
}

impl Display for AppMgr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("app mgr display")
    }
}
