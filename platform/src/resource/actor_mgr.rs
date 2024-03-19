use std::fmt::Display;
use std::sync::{Arc, RwLock, Weak};

use dashmap::DashSet;
use serde::{Deserialize, Serialize};

use common::structs::actor_info::ActorInfo;
use common::structs::state::State;
use common::structs::value_type::ValueType;

use crate::app::app_mgr::{SyncAppMgr, SyncAppName, SyncAppNameSet};
use crate::resource::sensor_mgr::SensorMgr;
use crate::resource::RwlockAlive;

/// actor_mgr is a struct that manages the lifecycle of actors.
/// should be protected by a RwLock.
/// only one resoure_driver should own a write lock of  actor_mgr.
pub type SyncActorMgr = Arc<ActorMgr>;
pub type SyncActorName = Arc<String>;
pub type WeakActorMgr = Weak<SensorMgr>;
pub type RwLockOptionSyncActorMgr = RwLock<Option<SyncActorMgr>>;
pub type SyncActorNameSet = DashSet<SyncActorName>;

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
    apps: SyncAppNameSet,
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
    pub fn get_apps(&self) -> &SyncAppNameSet {
        todo!()
    }

    /// get app names
    pub fn get_app_names_vec(&self) -> Vec<SyncAppName> {
        todo!()
    }

    /// is_alive function
    /// return true if the actor is alive
    pub fn is_alive(&self) -> bool {
        *self.is_alive.read().expect("read is alive fail")
    }

    /// set_alive function
    /// set the actor alive or not
    /// used when register or error
    pub fn set_alive(&self, alive: bool) {
        *self.is_alive.write().expect("write is alive fail") = alive;
    }
}

impl Display for ActorMgr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let actor_info = ActorInfo::new(
            Some(self.actor_name.clone()),
            self.actor_type.clone(),
            match self.is_alive() {
                true => State::On,
                false => State::Off,
            },
            self.get_app_names_vec(),
        );
        write!(
            f,
            "{}",
            serde_json::to_string(&actor_info).expect("actor info to string fail")
        )
    }
}
