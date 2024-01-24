use std::net::TcpListener;
use std::thread;

use dashmap::DashMap;
use log::trace;
use once_cell::sync::Lazy;

use common::SyncString;

use crate::resource::actor_mgr::SyncActorMgr;
use crate::resource::resource_driver::ResourceDriver;
use crate::resource::sensor_mgr::SyncSensorMgr;

/// res_mgr_thread is a webserver that provides a management interface for tcp connections.

pub struct ResMgrThread {
    sensor_mgrs: DashMap<SyncString, SyncSensorMgr>,
    actor_mgrs: DashMap<SyncString, SyncActorMgr>,
    listener: TcpListener,
}

impl ResMgrThread {
    pub fn new() -> Self {
        ResMgrThread {
            sensor_mgrs: DashMap::new(),
            actor_mgrs: DashMap::new(),
            //todo: bind to a port by config
            listener: TcpListener::bind("???").unwrap(),
        }
    }

    pub fn run(&self) {
        // 增加管理器启动标志...
        // Platform::incr_mgr_start_flag();

        // 锁定直到管理器启动标志达到特定值...
        // Platform::lock_until_mgr_start_flag_equal(3);

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            trace!("New resource connection: {}", stream.peer_addr().unwrap());

            thread::spawn(move || {
                let resource_driver = ResourceDriver::new(stream);
                resource_driver.run();
            });
        }
    }

    pub fn get_sensor_mgrs(&self) -> &DashMap<SyncString, SyncSensorMgr> {
        &self.sensor_mgrs
    }

    pub fn get_actor_mgrs(&self) -> &DashMap<SyncString, SyncActorMgr> {
        &self.actor_mgrs
    }
}

// create instance of res_mgr_thread
pub static RES_MGR_THREAD: Lazy<ResMgrThread> = Lazy::new(|| ResMgrThread::new());

#[cfg(test)]
mod tests {
    #[test]
    fn test_res_mgr_thread() {
        //RES_MGR_THREAD.run();
    }
}
