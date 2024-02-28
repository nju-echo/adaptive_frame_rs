use std::fmt::Display;
use std::sync::{Arc, RwLock, Weak};

use serde::{Deserialize, Serialize};

use common::structs::value_type::ValueType;

use crate::app::app_mgr::{SyncAppMgr, SyncAppName, WeakAppMap};
use crate::resource::sensor_mgr::SensorMgr;
use crate::resource::RwlockAlive;

/// actor_mgr is a struct that manages the lifecycle of actors.
/// should be protected by a RwLock.
/// only one resoure_driver should own a write lock of  actor_mgr.
pub type SyncActorMgr = Arc<SensorMgr>;
pub type SyncActorName = Arc<String>;
pub type WeakActorMgr = Weak<SensorMgr>;
pub type RwLockOptionSyncActorMgr = RwLock<Option<SyncActorMgr>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ActorMgr {
    //TODO: set apps a set of Weak<RwLock<App>>
    #[serde(rename = "name")]
    actor_name: SyncActorName,
    #[serde(rename = "value_type", default = "default_actor_type")]
    actor_type: ValueType,
    #[serde(default = "default_is_alive")]
    is_alive: RwlockAlive,
    #[serde(skip)]
    apps: WeakAppMap,
}

fn default_actor_type() -> ValueType {
    ValueType::String
}

fn default_is_alive() -> RwlockAlive {
    RwLock::new(true)
}

impl ActorMgr {
    ///get actor name
    pub fn get_actor_name(&self) -> &SyncActorName {
        &self.actor_name
    }

    ///get actor type
    pub fn get_actor_type_clone(&self) -> ValueType {
        self.actor_type
    }

    /// #relate to apps

    /// add app
    pub fn add_app(&self, app: SyncAppMgr) {
        todo!()
    }

    /// remove app
    pub fn remove_app(&self, app: SyncAppMgr) {
        todo!()
    }

    /// get apps
    /// may leak information
    pub fn get_apps(&self) -> &WeakAppMap {
        todo!()
    }

    /// get app names
    pub fn get_app_names_vec(&self) -> Vec<SyncAppName> {
        todo!()
    }

    /// is_alive function
    /// return true if the actor is alive
    pub fn is_alive(&self) -> bool {
        todo!()
    }

    /// set_alive function
    /// set the actor alive or not
    /// used when register or error
    pub fn set_alive(&self, alive: bool) {
        todo!()
    }
}

impl Display for ActorMgr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
