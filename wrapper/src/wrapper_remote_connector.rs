use std::sync::{Arc, RwLock, Weak};

use log::info;
use once_cell::sync::Lazy;
use serde_json::value::Index;

use common::socket::cmd_message::CmdMessage;
use common::socket::tcp::TCP;
use common::structs::resource_config::ResourceConfig;

use crate::wrapper_remote_connector::wrapper_remote_connector_tcp::RwLockOptionWrapperRemoteConnectorTCP;

pub mod wrapper_remote_connector_tcp;

pub type WeakWrapperRemoteConnector = Weak<WrapperRemoteConnector>;
pub type SyncWrapperRemoteConnector = Arc<WrapperRemoteConnector>;
pub type WrapperString = String;
pub type SyncWrapperString = Arc<WrapperString>;
pub type RwLockOptionSyncWrapperString = RwLock<Option<Arc<WrapperString>>>;

pub struct WrapperRemoteConnector {
    wrapper_name: RwLockOptionSyncWrapperString,
    tcp: RwLockOptionWrapperRemoteConnectorTCP,
}

impl WrapperRemoteConnector {
    /// new
    /// give a none wrapper_name and none tcp
    fn new() -> Self {
        Self {
            wrapper_name: RwLock::new(None),
            tcp: RwLock::new(None),
        }
    }

    /// recv    ///  from tcp
    /// transfer it to cmd_message
    /// return cmd_message
    pub fn recv(&self) -> Option<CmdMessage> {
        if let Some(recv) = self
            .tcp
            .read()
            .expect("read tcp fail")
            .as_ref()
            .expect("tcp is none")
            .recv()
        {
            let cmd_message: CmdMessage =
                serde_json::from_str(&recv).expect("parse cmd message fail");

            if cmd_message
                .cmd
                .as_ref()
                .expect("cmd is none")
                .eq_ignore_ascii_case("alive_request")
                == false
            {
                info!(
                    "[{}]: recv() -> {}",
                    self.wrapper_name
                        .read()
                        .expect("read wrapper name fail")
                        .as_ref()
                        .expect("wrapper name is none"),
                    recv
                );
            }
            return Some(cmd_message);
        }
        info!(
            "[{}]: recv() -> None",
            self.wrapper_name
                .read()
                .expect("read wrapper name fail")
                .as_ref()
                .expect("wrapper name is none")
        );
        None
    }

    /// send    ///  string to platform
    pub fn send(&self, send: &str) {
        self.tcp
            .read()
            .expect("read tcp fail")
            .as_ref()
            .expect("tcp is none")
            .send(send);
        info!(
            "[{}]: send({})",
            self.wrapper_name
                .read()
                .expect("read wrapper name fail")
                .as_ref()
                .expect("wrapper name is none"),
            send
        );
    }

    /// close tcp
    pub fn close(&self) {
        self.shutdown();
    }

    /// shutdown tcp
    pub fn shutdown(&self) -> bool {
        self.tcp
            .read()
            .expect("read tcp fail")
            .as_ref()
            .expect("tcp is none")
            .close();
        info!(
            "[{}]: shutdown()",
            self.wrapper_name
                .read()
                .expect("read wrapper name fail")
                .as_ref()
                .expect("wrapper name is none")
        );
        true
    }

    /// register to platform
    /// will set wrapper_name and tcp
    pub fn register(&self, ip_addr: &str, port: u16, resource_config: ResourceConfig) -> bool {
        //set wrapper_name
        let wrapper_name = resource_config
            .name
            .as_ref()
            .expect("resource name is none")
            .clone();
        self.wrapper_name
            .write()
            .expect("write wrapper name fail")
            .replace(Arc::new(wrapper_name));

        //todo: log set

        //generate cmd_message
        let cmd_message = CmdMessage::new(
            Some("register".to_string()),
            Some(serde_json::to_value(&resource_config).expect("to value fail")),
        );
        let mut state: bool = false;

        //由于err已经在内部处理，所以这里不需要处理err
        //get socket by ip_addr and port
        //notion: 在java中，使用了try catch，但实际上没有处理err，所以这里直接使用expect
        let socket = std::net::TcpStream::connect((ip_addr, port)).expect("connect fail");
        //set tcp
        let tcp = wrapper_remote_connector_tcp::WrapperRemoteConnectorTCP::new(
            socket,
            false,
            self.wrapper_name
                .read()
                .expect("read wrapper name fail")
                .as_ref()
                .expect("wrapper name is none")
                .clone(),
        );
        self.tcp.write().expect("write tcp fail").replace(tcp);

        //register
        self.tcp
            .read()
            .expect("read tcp fail")
            .as_ref()
            .expect("tcp is none")
            .send(&serde_json::to_string(&cmd_message).expect("to string fail"));

        //recv
        if let Some(recv) = self
            .tcp
            .read()
            .expect("read tcp fail")
            .as_ref()
            .expect("tcp is none")
            .recv()
        {
            let recv_cmd_message: CmdMessage =
                serde_json::from_str(&recv).expect("parse cmd message fail");
            if recv_cmd_message
                .cmd
                .as_ref()
                .expect("cmd is none")
                .eq("register_back")
            {
                //only message is "true" ignore case
                //todo: check it whether one is true
                match recv_cmd_message.message.as_ref() {
                    Some(message) => {
                        state = message
                            .as_str()
                            .expect("message as str fail")
                            .eq_ignore_ascii_case("true");
                    }
                    None => state = false,
                }
            }
        }

        info!(
            "[{}]: register({}, {}, {}) -> {}",
            self.wrapper_name
                .read()
                .expect("read wrapper name fail")
                .as_ref()
                .expect("wrapper name is none"),
            ip_addr,
            port,
            serde_json::to_string(&resource_config).expect("to string fail"),
            state
        );
        state
    }
}

pub static WRAPPER_REMOTE_CONNECTOR: Lazy<SyncWrapperRemoteConnector> =
    Lazy::new(|| Arc::new(WrapperRemoteConnector::new()));
