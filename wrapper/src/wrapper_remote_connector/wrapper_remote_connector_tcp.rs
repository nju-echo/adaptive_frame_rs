use std::net::TcpStream;
use std::sync::{Arc, RwLock};

use log::info;

use common::socket::abstract_tcp::AbstractTCP;
use common::socket::tcp::TCP;

use crate::wrapper_remote_connector::{
    SyncWrapperRemoteConnector, SyncWrapperString, WeakWrapperRemoteConnector,
};

pub type RwLockOptionWrapperRemoteConnectorTCP = RwLock<Option<WrapperRemoteConnectorTCP>>;

pub struct WrapperRemoteConnectorTCP {
    abstract_tcp: AbstractTCP,
    wrapper_remote_connector_name: SyncWrapperString,
}

impl WrapperRemoteConnectorTCP {
    pub fn new(
        socket: TcpStream,
        lock_flag: bool,
        wrapper_remote_connector_name: SyncWrapperString,
    ) -> Self {
        Self {
            abstract_tcp: AbstractTCP::new(socket, lock_flag),
            wrapper_remote_connector_name,
        }
    }
}

impl TCP for WrapperRemoteConnectorTCP {
    fn super_reference(&self) -> &AbstractTCP {
        &self.abstract_tcp
    }

    fn close(&self) {
        self.super_reference().close();
    }

    fn callback(&self) {
        info!(
            "[{}]: TCP connection is broken.",
            self.wrapper_remote_connector_name
        );
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
