use std::fmt::{Display, Formatter};
use std::net::TcpStream;
use std::sync::{Arc, RwLock, Weak};

use log::{info, trace};

use common::socket::cmd_message::CmdMessage;
use common::socket::tcp::TCP;
use common::structs::enumeration::resource_type::{ResourceType, RwLockOptionResourceType};
use common::structs::resource_config::ResourceConfig;
use common::SyncString;

use crate::pubsub::abstract_subscriber;
use crate::pubsub::abstract_subscriber::SubscriberId;
use crate::pubsub::channel::get_sensor_request;
use crate::pubsub::grp_prio_pair::{GroupId, GrpPrioPair, PrioId};
use crate::pubsub::subscriber::Subscriber;
use crate::resource::actor_mgr::RwLockOptionSyncActorMgr;
use crate::resource::res_mgr_thread::RES_MGR_THREAD;
use crate::resource::resource_driver::device_driver_tcp::{DeviceDriverTCP, RwlockDeviceDriverTCP};
use crate::resource::sensor_mgr::{RwLockOptionSyncSensorMgr, SensorMgr, SyncSensorMgr};

pub mod device_driver_tcp;

pub type SyncResourceDriver = Arc<ResourceDriver>;
pub type WeakResourceDriver = Weak<ResourceDriver>;
pub type SyncResourceName = Arc<String>;
pub type RwLockOptionSyncResourceName = RwLock<Option<SyncResourceName>>;

/// resource_driver is a job thread that res_mgr_thread can use to perform resource operations.
//todo: add lock inside instead of outside to avoid field config
pub struct ResourceDriver {
    // inherit from AbstractSubscriber
    abstract_subscriber: abstract_subscriber::AbstractSubscriber,
    //TODO: how to define a struct in a thread?
    tcp: RwlockDeviceDriverTCP,
    sensor_mgr: RwLockOptionSyncSensorMgr,
    actor_mgr: RwLockOptionSyncActorMgr,
    resource_type: RwLockOptionResourceType,
    device_name: RwLockOptionSyncResourceName,
}

impl ResourceDriver {
    pub fn new(stream: TcpStream, id: SubscriberId) -> Self {
        let abstract_subscriber = abstract_subscriber::AbstractSubscriber::new(id);
        let tcp = RwLock::new(DeviceDriverTCP::new(stream, false));

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

        trace!(
            "New resource connection: {} add to subscriber objs success",
            resource_driver
                .tcp
                .read()
                .expect("get tcp fail")
                .abstract_tcp
                .get_socket()
                .peer_addr()
                .unwrap()
        );
        resource_driver
    }

    //todo: transfer Arc<Rwlock<ResourceDriver>> to this function
    pub fn run(driver: SyncResourceDriver) {
        if let Some(msg_from_client) = driver.tcp.write().expect("get tcp fail").recv() {
            //todo: should handle cmd message when err?
            let cmd_message = serde_json::from_str::<CmdMessage>(&msg_from_client)
                .expect("parse cmd message fail");
            info!("[wrapper -> platform]: {}", cmd_message);

            let joo = serde_json::to_string(&cmd_message.message).expect("parse cmd message fail");
            trace!("[wrapper -> platform]: {}", joo);

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

                match resource_type {
                    ResourceType::Sensor => {
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
                        } else {
                            let sensor_mgr: Arc<SensorMgr> = Arc::new(
                                serde_json::from_str(&joo).expect("parse sensor mgr fail"),
                            );
                            driver
                                .sensor_mgr
                                .write()
                                .expect("get write sensor mgr fail")
                                .replace(sensor_mgr.clone());
                            RES_MGR_THREAD
                                .get_sensor_mgrs()
                                .insert(device_name.clone(), sensor_mgr);
                        }
                        driver.abstract_subscriber.subscribe(
                            &get_sensor_request(&device_name),
                            None,
                            None,
                        );
                    }
                    ResourceType::Actor => {}
                    ResourceType::Hybrid => {}
                }
            }

            todo!()
        }
    }

    //question: need callback when tcp is error
    //solution: is tcp a Arc or just a TcpStream?
    //now: tcp handle itself
}

impl Display for ResourceDriver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Subscriber for ResourceDriver {
    fn on_message(&self, channel: SyncString, msg: SyncString) {
        todo!()
    }

    fn id(&self) -> i32 {
        todo!()
    }

    fn get_grp_prio_pair(&self, channel: &str) -> Option<GrpPrioPair> {
        todo!()
    }

    fn subscribe(&self, channel: &str, group_id: Option<GroupId>, priority_id: Option<PrioId>) {
        todo!()
    }

    fn unsubscribe(&self, channel: &str) {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }
}
