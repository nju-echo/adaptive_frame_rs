use std::any::type_name;
use std::fmt::{Display, Formatter};
use std::net::TcpStream;
use std::string::ToString;
use std::sync::{Arc, RwLock, Weak};
use std::thread;

use log::{info, trace};

use common::socket::cmd_message::CmdMessage;
use common::socket::cmd_message_grp_ids::CmdMessageGrpIds;
use common::socket::tcp::TCP;
use common::structs::enumeration::resource_type::{ResourceType, RwLockOptionResourceType};
use common::structs::resource_config::ResourceConfig;
use common::SyncString;

use crate::pubsub::abstract_subscriber::SubscriberId;
use crate::pubsub::channel::{get_actor_request, get_sensor_request};
use crate::pubsub::grp_prio_pair::{GroupId, GrpPrioPair, PrioId};
use crate::pubsub::subscriber::Subscriber;
use crate::pubsub::{abstract_subscriber, publisher};
use crate::resource::actor_mgr::{ActorMgr, RwLockOptionSyncActorMgr};
use crate::resource::res_mgr_thread::RES_MGR_THREAD;
use crate::resource::resource_driver::device_driver_tcp::DeviceDriverTCP;
use crate::resource::sensor_mgr::{RwLockOptionSyncSensorMgr, SensorMgr, SyncSensorMgr};

pub mod device_driver_tcp;

pub type SyncResourceDriver = Arc<ResourceDriver>;
pub type WeakResourceDriver = Weak<ResourceDriver>;
pub type SyncResourceName = Arc<String>;
pub type RwLockOptionSyncResourceName = RwLock<Option<SyncResourceName>>;

const DEFAULT_NONE_STR: &str = "@#$%";

/// resource_driver is a job thread that res_mgr_thread can use to perform resource operations.
//todo: add lock inside instead of outside to avoid field config
pub struct ResourceDriver {
    // inherit from AbstractSubscriber
    abstract_subscriber: abstract_subscriber::AbstractSubscriber,
    //TODO: how to define a struct in a thread?
    tcp: DeviceDriverTCP,
    sensor_mgr: RwLockOptionSyncSensorMgr,
    actor_mgr: RwLockOptionSyncActorMgr,
    resource_type: RwLockOptionResourceType,
    device_name: RwLockOptionSyncResourceName,
}

impl ResourceDriver {
    pub fn new(stream: TcpStream, id: SubscriberId) -> Self {
        let abstract_subscriber = abstract_subscriber::AbstractSubscriber::new(id);
        let tcp = DeviceDriverTCP::new(stream, false);

        Self {
            abstract_subscriber,
            tcp,
            sensor_mgr: RwLock::new(None),
            actor_mgr: RwLock::new(None),
            resource_type: RwLock::new(None),
            device_name: RwLock::new(None),
        }
    }

    ///add to static set Abstract Subscriber objs
    pub fn add_to_subscriber_objs(stream: TcpStream) -> SyncResourceDriver {
        let mut subscriber_objs = abstract_subscriber::get_objs()
            .write()
            .expect("get subscriber objs failed");
        let resource_driver = Arc::new(Self::new(stream, subscriber_objs.len() as i32));
        subscriber_objs.push(resource_driver.clone());

        resource_driver
            .tcp
            .set_resource_driver_weak(&resource_driver);
        trace!(
            "New resource connection: {} add to subscriber objs success",
            resource_driver.tcp.get_socket().peer_addr().unwrap()
        );
        resource_driver
    }

    pub fn run(driver: SyncResourceDriver) {
        if let Some(msg_from_client) = driver.tcp.recv() {
            //todo: should handle cmd message when err?
            let cmd_message = serde_json::from_str::<CmdMessage>(&msg_from_client)
                .expect("parse cmd message fail");
            info!("[wrapper -> platform]: {}", cmd_message);

            let joo = serde_json::to_string(&cmd_message.message).expect("parse cmd message fail");

            //handle register
            if cmd_message
                .cmd
                .expect("cmd is none")
                .eq_ignore_ascii_case("register")
            {
                let resource_config: ResourceConfig =
                    serde_json::from_str(&joo).expect("parse resource config fail");
                let resource_type = resource_config.resource_type;
                let device_name = Arc::new(resource_config.name.expect("device name is none"));

                //set resource_type and device_name
                driver
                    .resource_type
                    .write()
                    .expect("write resource type fail")
                    .replace(resource_type);
                driver
                    .device_name
                    .write()
                    .expect("write device name fail")
                    .replace(device_name.clone());

                match resource_type {
                    ResourceType::Sensor => {
                        Self::register_sensor(driver.clone(), device_name, &joo);
                    }
                    ResourceType::Actor => {
                        Self::register_actor(driver.clone(), device_name, &joo);
                    }
                    ResourceType::Hybrid => {
                        Self::register_hybrid(driver.clone(), device_name, &joo);
                    }
                }
            }

            //register back
            let return_msg = CmdMessage::new(
                Some("register_back".to_string()),
                Some(serde_json::json!("true")),
            );
            driver.tcp.send(&return_msg.to_string());
            info!(
                "[platform -> {}]: {}",
                driver
                    .device_name
                    .read()
                    .expect("read device name fail")
                    .as_ref()
                    .expect("device name is none"),
                return_msg
            );

            //set tcp lock_flag
            driver.tcp.set_lock_flag(true);

            //alive request loop
            loop {
                thread::sleep(std::time::Duration::from_secs(1));

                match driver
                    .resource_type
                    .read()
                    .expect("read resource type fail")
                    .as_ref()
                    .expect("resource type is none")
                {
                    ResourceType::Sensor => {
                        if driver
                            .sensor_mgr
                            .read()
                            .expect("read sensor mgr fail")
                            .as_ref()
                            .expect("get sensor mgr fail")
                            .is_alive()
                            == false
                        {
                            break;
                        }
                    }
                    ResourceType::Actor => {
                        if driver
                            .actor_mgr
                            .read()
                            .expect("read actor mgr fail")
                            .as_ref()
                            .expect("get actor mgr fail")
                            .is_alive()
                            == false
                        {
                            break;
                        }
                    }
                    ResourceType::Hybrid => {
                        if driver
                            .sensor_mgr
                            .read()
                            .expect("read sensor mgr fail")
                            .as_ref()
                            .expect("get sensor mgr fail")
                            .is_alive()
                            == false
                            || driver
                                .actor_mgr
                                .read()
                                .expect("read actor mgr fail")
                                .as_ref()
                                .expect("get actor mgr fail")
                                .is_alive()
                                == false
                        {
                            break;
                        }
                    }
                }

                let alive_request = CmdMessage::new(Some("alive_request".to_string()), None);

                if driver.tcp.send(&alive_request.to_string()) == false {
                    break;
                }
                trace!(
                    "[platform -> {}]: {}",
                    driver
                        .device_name
                        .read()
                        .expect("read device name fail")
                        .as_ref()
                        .expect("device name is none"),
                    alive_request
                );

                driver.tcp.unlock();
            }
        }
    }

    fn register_sensor(driver: SyncResourceDriver, device_name: SyncResourceName, joo: &str) {
        if RES_MGR_THREAD.get_sensor_mgrs().contains_key(&device_name) {
            let sensor_mgr = RES_MGR_THREAD
                .get_sensor_mgrs()
                .get(&device_name)
                .expect("get sensor mgr fail")
                .clone();
            driver
                .sensor_mgr
                .write()
                .expect("get write sensor mgr fail")
                .replace(sensor_mgr);
            driver
                .sensor_mgr
                .read()
                .expect("get read sensor mgr fail")
                .as_ref()
                .expect("sensor mgr is none")
                .set_alive(true);

            trace!("Sensor: {} get from sensor mgrs success", device_name);
        } else {
            let sensor_mgr: Arc<SensorMgr> =
                Arc::new(serde_json::from_str(joo).expect("parse sensor mgr fail"));
            driver
                .sensor_mgr
                .write()
                .expect("get write sensor mgr fail")
                .replace(sensor_mgr.clone());
            RES_MGR_THREAD
                .get_sensor_mgrs()
                .insert(device_name.clone(), sensor_mgr);
            trace!("New sensor: {} add to sensor mgrs success", device_name);
        }
        driver.subscribe(&get_sensor_request(&device_name), None, None);

        trace!("New sensor: {} register to platform success", device_name);
    }

    fn register_actor(driver: SyncResourceDriver, device_name: SyncResourceName, joo: &str) {
        if RES_MGR_THREAD.get_actor_mgrs().contains_key(&device_name) {
            let actor_mgr = RES_MGR_THREAD
                .get_actor_mgrs()
                .get(&device_name)
                .expect("get actor mgr fail")
                .clone();
            driver
                .actor_mgr
                .write()
                .expect("get write actor mgr fail")
                .replace(actor_mgr);
            driver
                .actor_mgr
                .read()
                .expect("get read actor mgr fail")
                .as_ref()
                .expect("actor mgr is none")
                .set_alive(true);

            trace!("Actor: {} get from actor mgrs success", device_name);
        } else {
            let actor_mgr: Arc<ActorMgr> =
                Arc::new(serde_json::from_str(joo).expect("parse actor mgr fail"));
            driver
                .actor_mgr
                .write()
                .expect("get write actor mgr fail")
                .replace(actor_mgr.clone());
            RES_MGR_THREAD
                .get_actor_mgrs()
                .insert(device_name.clone(), actor_mgr);
            trace!("New actor: {} add to actor mgrs success", device_name);
        }
        driver.subscribe(&get_actor_request(&device_name), None, None);

        trace!("New actor: {} register to platform success", device_name);
    }

    fn register_hybrid(driver: SyncResourceDriver, device_name: SyncResourceName, joo: &str) {
        Self::register_sensor(driver.clone(), device_name.clone(), joo);
        Self::register_actor(driver.clone(), device_name.clone(), joo);
    }

    fn on_message_handle(&self, channel: SyncString, msg: SyncString) {
        let resource_name_and_type = format!(
            "{}<{}>",
            self.device_name
                .read()
                .expect("read device name fail")
                .as_ref()
                .expect("device name is none"),
            self.resource_type
                .read()
                .expect("read resource type fail")
                .as_ref()
                .expect("resource type is none")
        );

        let cmd_message_grp_ids: CmdMessageGrpIds =
            serde_json::from_str(&msg).expect("parse cmd message grp ids fail");
        let send = cmd_message_grp_ids.get_cmd_message();
        self.tcp.send(&send.to_string());
        info!("[platform -> {}]: {}", resource_name_and_type, send);

        let recv: CmdMessage = match self.tcp.recv() {
            Some(ret) => serde_json::from_str(&ret).expect("parse cmd message fail"),
            None => {
                if send
                    .cmd
                    .expect("cmd is none")
                    .eq_ignore_ascii_case("sensory_request")
                {
                    CmdMessage::new(
                        Some("sensory_back".to_string()),
                        Some(
                            serde_json::to_value(DEFAULT_NONE_STR)
                                .expect("parse default none str fail"),
                        ),
                    )
                } else {
                    CmdMessage::new(
                        Some("action_back".to_string()),
                        Some(
                            serde_json::to_value(DEFAULT_NONE_STR)
                                .expect("parse default none str fail"),
                        ),
                    )
                }
            }
        };
        info!("[{} -> platform]: {}", resource_name_and_type, recv);

        for grp_id in cmd_message_grp_ids.grp_ids.expect("grp ids is none") {
            publisher::publish(
                &resource_name_and_type,
                Some(grp_id),
                None,
                Arc::new(recv.message.as_ref().expect("message is none").to_string()),
            );
        }
    }
    //question: need callback when tcp is error
    //solution: is tcp a Arc or just a TcpStream?
    //now: tcp handle itself
}

impl Display for ResourceDriver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", type_name::<Self>())
    }
}

impl Subscriber for ResourceDriver {
    fn super_reference(&self) -> &abstract_subscriber::AbstractSubscriber {
        &self.abstract_subscriber
    }

    fn on_message(&self, channel: SyncString, msg: SyncString) {
        match self
            .resource_type
            .read()
            .expect("read resource type fail")
            .as_ref()
            .expect("resource type is none")
        {
            ResourceType::Sensor => {
                if self
                    .sensor_mgr
                    .read()
                    .expect("read sensor mgr fail")
                    .as_ref()
                    .expect("sensor mgr is none")
                    .is_alive()
                {
                    self.on_message_handle(channel, msg);
                }
            }
            ResourceType::Actor => {
                if self
                    .actor_mgr
                    .read()
                    .expect("read actor mgr fail")
                    .as_ref()
                    .expect("actor mgr is none")
                    .is_alive()
                {
                    self.on_message_handle(channel, msg);
                }
            }
            ResourceType::Hybrid => {
                if self
                    .sensor_mgr
                    .read()
                    .expect("read sensor mgr fail")
                    .as_ref()
                    .expect("sensor mgr is none")
                    .is_alive()
                    && self
                        .actor_mgr
                        .read()
                        .expect("read actor mgr fail")
                        .as_ref()
                        .expect("actor mgr is none")
                        .is_alive()
                {
                    self.on_message_handle(channel, msg);
                }
            }
        }
    }
}
