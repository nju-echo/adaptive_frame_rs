use std::sync::{Arc, Weak};

use dashmap::DashMap;

pub type SyncAppMgr = Arc<AppMgr>;
pub type WeakAppMgr = Weak<AppMgr>;
pub type SyncAppName = Arc<String>;
pub type WeakAppMap = DashMap<SyncAppName, WeakAppMgr>;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct AppMgr {
    //TODO
}
