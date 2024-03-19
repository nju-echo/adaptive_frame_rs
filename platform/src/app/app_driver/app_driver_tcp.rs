use std::net::TcpStream;
use std::sync::{Arc, RwLock};

use log::{error, info, trace};

use common::socket::abstract_tcp::AbstractTCP;
use common::socket::tcp::TCP;

use crate::app::app_driver::{RwLockOptionWeakAppDriver, SyncAppDriver};

pub struct AppDriverTCP {
    abstract_tcp: AbstractTCP,
    app_driver_weak: RwLockOptionWeakAppDriver,
}

impl AppDriverTCP {
    /// new app driver tcp
    /// use socket and lock flag to new an app driver tcp
    pub fn new(socket: TcpStream, lock_flag: bool) -> Self {
        Self {
            abstract_tcp: AbstractTCP::new(socket, lock_flag),
            app_driver_weak: RwLock::new(None),
        }
    }

    /// set app driver weak
    /// set app driver weak to app driver tcp
    pub fn set_app_driver_weak(&self, app_driver: &SyncAppDriver) {
        self.app_driver_weak
            .write()
            .expect("write app driver weak fail")
            .replace(Arc::downgrade(app_driver));
        trace!(
            "app tcp connection {}: set app driver weak success",
            self.abstract_tcp.get_socket().peer_addr().unwrap()
        );
    }
}

impl TCP for AppDriverTCP {
    fn super_reference(&self) -> &AbstractTCP {
        &self.abstract_tcp
    }

    fn close(&self) {
        trace!(
            "app tcp connection {}: close",
            self.super_reference().get_socket().peer_addr().unwrap()
        );
        self.super_reference()
            .get_socket()
            .shutdown(std::net::Shutdown::Both)
            .expect("shutdown socket fail");
        trace!("close success");
        //disconnect platform
        self.app_driver_weak
            .read()
            .expect("read app driver weak fail")
            .as_ref()
            .expect("app driver weak is none")
            .upgrade()
            .expect("upgrade app driver weak fail")
            .disconnect_platform();
    }

    fn callback(&self) {
        let mut name = String::from("AppDrive");
        if self
            .app_driver_weak
            .read()
            .expect("read app driver weak fail")
            .as_ref()
            .expect("app driver weak is none")
            .upgrade()
            .expect("upgrade app driver weak fail")
            .app_mgr
            .read()
            .expect("read app mgr fail")
            .is_some()
        {
            name = self
                .app_driver_weak
                .read()
                .expect("read app driver weak fail")
                .as_ref()
                .expect("app driver weak is none")
                .upgrade()
                .expect("upgrade app driver weak fail")
                .app_mgr
                .read()
                .expect("read app mgr fail")
                .as_ref()
                .expect("app mgr is none")
                .get_app_name_clone()
                .as_str()
                .to_string();
        }
        error!(
            "[{}]: TCP connection is broken. Start releasing app resources...",
            name
        );
        if self
            .app_driver_weak
            .read()
            .expect("read app driver weak fail")
            .as_ref()
            .expect("app driver weak is none")
            .upgrade()
            .expect("upgrade app driver weak fail")
            .disconnect_platform()
            .eq("{\"state\":true}")
        {
            info!("[{}]: Release app resources success", name);
        } else {
            error!("[{}]: Release app resources fail", name);
        }
    }
}
