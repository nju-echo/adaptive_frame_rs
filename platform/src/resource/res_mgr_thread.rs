use std::net::{TcpListener, TcpStream};
use std::thread;

use dashmap::DashMap;
use log::trace;
use once_cell::sync::Lazy;

use common::SyncString;

use crate::config;
use crate::resource::actor_mgr::{SyncActorMgr, SyncActorName};
use crate::resource::resource_driver::ResourceDriver;
use crate::resource::sensor_mgr::{SyncSensorMgr, SyncSensorName};

/// res_mgr_thread is a webserver that provides a management interface for tcp connections.

pub struct ResMgrThread {
    sensor_mgrs: DashMap<SyncSensorName, SyncSensorMgr>,
    actor_mgrs: DashMap<SyncActorName, SyncActorMgr>,
    listener: TcpListener,
}

impl ResMgrThread {
    pub fn new() -> Self {
        ResMgrThread {
            sensor_mgrs: DashMap::new(),
            actor_mgrs: DashMap::new(),
            listener: TcpListener::bind(format!(
                "0.0.0.0:{}",
                config::configuration::TCP_CONFIG
                    .lock()
                    .expect("get tcp config fail")
                    .get_resource_listen_port()
            ))
            .expect("bind tcp port fail"),
        }
    }

    ///handle recv TcpStream
    fn handle_recv_tcp_stream(stream: TcpStream) {
        //create a resource driver and add it to subscriber objs
        let resource_driver = ResourceDriver::add_to_subscriber_objs(stream);
        //run resource driver
        ResourceDriver::run(resource_driver);
    }

    pub fn run(&self) {
        // 增加管理器启动标志...
        // Platform::incr_mgr_start_flag();

        // 锁定直到管理器启动标志达到特定值...
        // Platform::lock_until_mgr_start_flag_equal(3);

        for stream in self.listener.incoming().take(1) {
            let stream = stream.unwrap();
            trace!("New resource connection: {}", stream.peer_addr().unwrap());

            thread::spawn(move || {
                Self::handle_recv_tcp_stream(stream);
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
    use std::net::TcpStream;
    use std::thread;

    use env_logger::Builder;

    use crate::config::configuration::config_analyze;
    use crate::resource::res_mgr_thread::RES_MGR_THREAD;

    #[test]
    fn test_res_mgr_thread() {
        Builder::new().parse_filters("trace").init();
        let config_file = "./configfile".to_string();
        config_analyze(config_file.as_ref());

        let aa = thread::spawn(move || {
            RES_MGR_THREAD.run();
        });

        //link to server
        thread::sleep(std::time::Duration::from_secs(1));

        let mut stream = TcpStream::connect("127.0.0.1:9091").unwrap();

        thread::sleep(std::time::Duration::from_secs(1));
        aa.join();
    }
}
