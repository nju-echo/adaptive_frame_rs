use std::sync::{Arc, RwLock};

use crate::resource::sensor_mgr::SensorMgr;

/// actor_mgr is a struct that manages the lifecycle of actors.
/// should be protected by a RwLock.
/// only one resoure_driver should own a write lock of  actor_mgr.
pub type SyncActorMgr = Arc<RwLock<SensorMgr>>;

pub struct ActorMgr {
    //TODO
}
