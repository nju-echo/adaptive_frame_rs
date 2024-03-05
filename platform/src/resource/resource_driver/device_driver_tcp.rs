use std::io::Error;
use std::net::TcpStream;
use std::ptr::read;
use std::sync::{Arc, RwLock, Weak};

use log::{error, trace};

use common::socket::abstract_tcp::AbstractTCP;
use common::socket::tcp::TCP;
use common::structs::enumeration::resource_type::ResourceType;

use crate::resource::resource_driver::{ResourceDriver, SyncResourceDriver, WeakResourceDriver};

//todo: resource_driver和device_driver_tcp的依赖关系对吗？
// 感觉得反过来，device_driver_tcp应该依赖resource_driver,所以RwLock<Option<DeviceDriverTcp>>应该给resource_driver

///DeviceDriverTCP is a struct that implements TCP trait
/// it's a wrapper of AbstractTCP
/// should lock outside
pub struct DeviceDriverTCP {
    abstract_tcp: AbstractTCP,
    resource_driver_weak: RwLock<Option<WeakResourceDriver>>,
}

impl DeviceDriverTCP {
    pub fn new(socket: TcpStream, lock_flag: bool) -> Self {
        Self {
            abstract_tcp: AbstractTCP::new(socket, lock_flag),
            resource_driver_weak: RwLock::new(None),
        }
    }

    pub fn set_resource_driver_weak(&self, resource_driver: &SyncResourceDriver) {
        self.resource_driver_weak
            .write()
            .expect("write resource driver weak fail")
            .replace(Arc::downgrade(resource_driver));
        trace!(
            "resource tcp connection {}: set resource driver weak success",
            self.abstract_tcp.get_socket().peer_addr().unwrap()
        );
    }
}

impl TCP for DeviceDriverTCP {
    fn super_reference(&self) -> &AbstractTCP {
        &self.abstract_tcp
    }

    //TODO: HOW TO INVOKE IT JUST LIKE JAVA INTERFACE CALLBACK
    fn callback(&self) {
        error!(
            "{}: TCP connection is broken. Set the status to off",
            self.resource_driver_weak
                .read()
                .expect("read resource driver weak fail")
                .as_ref()
                .expect("resource driver weak is none")
                .upgrade()
                .expect("get resource driver fail")
                .device_name
                .read()
                .expect("read device name fail")
                .as_ref()
                .expect("device name is none")
        );

        let resource_driver = self
            .resource_driver_weak
            .read()
            .expect("read resource driver weak fail")
            .as_ref()
            .expect("resource driver weak is none")
            .upgrade()
            .expect("get resource driver fail");

        //notion: should not return resource_driver out, so should add ;
        match resource_driver
            .resource_type
            .read()
            .expect("read resource type fail")
            .as_ref()
            .expect("resource type is none")
        {
            ResourceType::Sensor => {
                resource_driver
                    .sensor_mgr
                    .read()
                    .expect("read sensor mgr fail")
                    .as_ref()
                    .expect("get sensor mgr fail")
                    .set_alive(false);
            }
            ResourceType::Actor => {
                resource_driver
                    .actor_mgr
                    .read()
                    .expect("read actor mgr fail")
                    .as_ref()
                    .expect("get actor mgr fail")
                    .set_alive(false);
            }
            ResourceType::Hybrid => {
                resource_driver
                    .sensor_mgr
                    .read()
                    .expect("read sensor mgr fail")
                    .as_ref()
                    .expect("get sensor mgr fail")
                    .set_alive(false);
                resource_driver
                    .actor_mgr
                    .read()
                    .expect("read actor mgr fail")
                    .as_ref()
                    .expect("get actor mgr fail")
                    .set_alive(false);
            }
        };

        //todo: unsubscribe the channel request
    }

    /*fn get_socket(&self) -> &TcpStream {
        self.abstract_tcp.get_socket()
    }

    fn send(&self, str: &str) -> bool {
        self.abstract_tcp.send(str)
    }

    fn recv(&self) -> Option<String> {
        self.abstract_tcp.recv()
    }

    fn close(&self) {
        self.abstract_tcp.close()
    }

    fn set_lock_flag(&self, lock_flag: bool) {
        self.abstract_tcp.set_lock_flag(lock_flag)
    }

    fn unlock(&self) {
        self.abstract_tcp.unlock()
    }*/
}
