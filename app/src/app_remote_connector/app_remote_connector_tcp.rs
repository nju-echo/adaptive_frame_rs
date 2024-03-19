use std::net::TcpStream;
use std::sync::RwLock;

use log::info;

use common::socket::abstract_tcp::AbstractTCP;
use common::socket::tcp::TCP;

use crate::app::RwLockOptionSyncAbstractApp;
use crate::app_remote_connector::APP_REMOTE_CONNECTOR;

pub type RwLockOptionAppRemoteConnectorTCP = RwLock<Option<AppRemoteConnectorTCP>>;

pub struct AppRemoteConnectorTCP {
    abstract_tcp: AbstractTCP,
    // 原来需要一个引用，但是app_remote_connector是单例，所以直接拿来就用吧
}

impl AppRemoteConnectorTCP {
    pub fn new(stream: TcpStream, lock_flag: bool) -> Self {
        Self {
            abstract_tcp: AbstractTCP::new(stream, lock_flag),
        }
    }
}

impl TCP for AppRemoteConnectorTCP {
    fn super_reference(&self) -> &AbstractTCP {
        &self.abstract_tcp
    }

    fn close(&self) {
        self.super_reference().close();
        if let Some(app) = APP_REMOTE_CONNECTOR
            .app
            .read()
            .expect("app read lock fail")
            .as_ref()
        {
            app.stop_get_value_thread();
        }
    }

    fn callback(&self) {
        info!("[AppConnector]: TCP connection is broken.");
        if let Some(app) = APP_REMOTE_CONNECTOR
            .app
            .read()
            .expect("app read lock fail")
            .as_ref()
        {
            app.stop_get_value_thread();
        }
    }
}
