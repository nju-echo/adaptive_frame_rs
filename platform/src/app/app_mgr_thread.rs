//todo: question: how to ensure port of tcp and udp is different

use std::fmt::format;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use dashmap::{DashMap, DashSet};
use log::{debug, trace};
use once_cell::sync::Lazy;

use common::socket::cmd_message_grp_ids::GroupId;

use crate::app::app_driver::AppDriver;
use crate::app::app_mgr::{AppMgr, SyncAppMgr, SyncAppName};
use crate::config;

pub type IpString = String;
pub type SyncIpString = Arc<IpString>;
pub type AppPort = u16;

pub struct AppMgrThread {
    listener: TcpListener,

    //store and manage all app_mgrs
    port_map: DashMap<IpString, DashSet<AppPort>>,
    app_grp_id_map: DashMap<SyncAppName, GroupId>,
    grp_id_app_map: DashMap<GroupId, SyncAppName>,
    app_mgrs: DashMap<SyncAppName, SyncAppMgr>,
}

impl AppMgrThread {
    /// register app mgr
    /// generate an app mgr and store it in app_mgrs
    pub fn register_app_mgr(&self, app_name: SyncAppName) -> SyncAppMgr {
        let app_mgr = Arc::new(AppMgr::new(app_name.clone()));
        self.app_mgrs.insert(app_name, app_mgr.clone());
        app_mgr
    }

    /// unregister app mgr
    /// remove app mgr by app name in app_mgrs
    pub fn unregister_app_mgr(&self, app_name: &SyncAppName) {
        self.app_mgrs.remove(app_name);
    }
}

impl AppMgrThread {
    fn new() -> Self {
        AppMgrThread {
            listener: TcpListener::bind(format!(
                "0.0.0.0:{}",
                config::configuration::TCP_CONFIG
                    .lock()
                    .expect("get tcp config fail")
                    .get_app_listen_port()
            ))
            .expect("bind app tcp port fail"),
            port_map: DashMap::new(),
            app_grp_id_map: DashMap::new(),
            grp_id_app_map: DashMap::new(),
            app_mgrs: DashMap::new(),
        }
    }

    fn handle_recv_tcp_stream(stream: TcpStream) {
        //create an app driver and add it to subscriber objs
        let app_mgr = AppDriver::add_to_subscriber_objs(stream);
        //run app driver
        AppDriver::run(app_mgr);
    }

    pub fn run(&self) {
        //todo: add platform control

        for stream in self.listener.incoming() {
            let stream = stream.expect("new app connection fail");
            trace!(
                "New app connection: {}",
                stream.peer_addr().expect("get peer addr fail")
            );

            thread::spawn(move || {
                Self::handle_recv_tcp_stream(stream);
            });
        }
    }

    /// get new app port
    /// generate a new app port for udp
    /// store socket port and udp port in port_map
    /// return udp port
    pub fn get_new_app_port(&self, stream: &TcpStream) -> AppPort {
        let client_host = stream
            .peer_addr()
            .expect("get peer addr fail")
            .ip()
            .to_string();
        let client_port = stream.peer_addr().expect("get peer addr fail").port();
        if self.port_map.contains_key(&client_host) == false {
            self.port_map.insert(client_host.clone(), DashSet::new());
        }
        let port_set = self.port_map.get(&client_host).expect("get port set fail");
        //todo: can not panic? be more robust
        if port_set.contains(&client_port) == true {
            panic!("port already exist");
        }
        let mut i: AppPort = 1;
        while port_set.contains(&(client_port + i)) == true {
            i += 1;
        }
        port_set.insert(client_port + i);
        client_port + i
    }

    /// remove app port
    /// remove socket port and udp port in port_map
    pub fn remove_app_port(&self, stream: &TcpStream, udp_port: AppPort) {
        let client_host = stream
            .peer_addr()
            .expect("get peer addr fail")
            .ip()
            .to_string();
        let client_port = stream.peer_addr().expect("get peer addr fail").port();
        let port_set = self.port_map.get(&client_host).expect("get port set fail");
        //todo: should we remove port_set if it is empty?
        port_set.remove(&client_port);
        port_set.remove(&udp_port);
    }

    /// get new grp id
    /// if contain app name, return grp id
    /// else generate a new grp id, store it in app_grp_id_map and grp_id_app_map
    /// return grp id
    pub fn get_new_grp_id(&self, app_name: SyncAppName) -> GroupId {
        if let Some(grp_id) = self.app_grp_id_map.get(&app_name) {
            return grp_id.value().clone();
        } else {
            let mut max: GroupId = 0;
            for key_value in self.grp_id_app_map.iter() {
                if key_value.key() > &max {
                    max = key_value.key().clone();
                }
            }
            max = max + 1;
            self.app_grp_id_map.insert(app_name.clone(), max);
            self.grp_id_app_map.insert(max, app_name.clone());
            max
        }
    }

    /// remove grp id
    /// remove grp id by app_name in app_grp_id_map and grp_id_app_map
    pub fn remove_grp_id(&self, app_name: &SyncAppName) {
        if let Some(grp_id) = self.app_grp_id_map.get(app_name) {
            self.grp_id_app_map.remove(&grp_id);
        }
        // to avoid deadlock, we should remove app_grp_id_map after grp_id_app_map
        self.app_grp_id_map.remove(app_name);
    }

    /// get app name
    /// get app name by grp id in grp_id_app_map
    /// it will return None if grp id not exist
    pub fn get_app_name(&self, grp_id: GroupId) -> Option<SyncAppName> {
        self.grp_id_app_map.get(&grp_id).map(|x| x.clone())
    }

    /// get grp id
    /// get grp id by app name in app_grp_id_map
    /// it will return None if app name not exist
    pub fn get_grp_id(&self, app_name: &SyncAppName) -> Option<GroupId> {
        self.app_grp_id_map.get(app_name).map(|x| x.clone())
    }

    /// get app mgrs
    pub fn get_app_mgrs(&self) -> &DashMap<SyncAppName, SyncAppMgr> {
        &self.app_mgrs
    }
}

//todo: create instance of app_mgr_thread
pub static APP_MGR_THREAD: Lazy<AppMgrThread> = Lazy::new(|| AppMgrThread::new());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grp_id_and_app_name() {
        let app_mgr_thread = AppMgrThread::new();
        let app_name = Arc::new("app_name".to_string());
        let grp_id = app_mgr_thread.get_new_grp_id(app_name.clone());
        assert_eq!(app_mgr_thread.get_app_name(grp_id).unwrap(), app_name);
        assert_eq!(app_mgr_thread.get_grp_id(&app_name).unwrap(), grp_id);
        app_mgr_thread.remove_grp_id(&app_name);
        assert_eq!(app_mgr_thread.get_app_name(grp_id), None);
        assert_eq!(app_mgr_thread.get_grp_id(&app_name), None);
    }
}
