use std::sync::RwLock;

/// todo: use two layers of lock, may have TOCTOU problem
///
///
///

pub type RwlockAlive = RwLock<bool>;

pub mod actor_mgr;
pub mod res_mgr_thread;
pub mod resource_driver;
pub mod sensor_mgr;
