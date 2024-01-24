use std::any::type_name;
use std::fmt::Display;
use std::sync::{Arc, RwLock};
use std::thread;

use common::SyncString;
use platform::pubsub::grp_prio_pair::{GroupId, GrpPrioPair, PrioId};
use platform::pubsub::subscriber::Subscriber;
use platform::pubsub::*;

struct TestSubscriber1 {
    abstract_subscriber: abstract_subscriber::AbstractSubscriber,
}

impl TestSubscriber1 {
    pub fn new() -> Arc<RwLock<Self>> {
        let subscriber_objs = abstract_subscriber::get_objs();
        let mut subscriber_objs = subscriber_objs.write().expect("get subscriber objs failed");
        let subscriber = Arc::new(RwLock::new(Self {
            abstract_subscriber: abstract_subscriber::AbstractSubscriber::new(
                subscriber_objs.len() as i32,
            ),
        }));
        subscriber_objs.push(subscriber.clone());
        subscriber
    }
}

impl Display for TestSubscriber1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", type_name::<Self>())
    }
}

impl Subscriber for TestSubscriber1 {
    fn id(&self) -> i32 {
        self.abstract_subscriber.id()
    }

    fn get_grp_prio_pair(&self, channel: &str) -> Option<GrpPrioPair> {
        self.abstract_subscriber.get_grp_prio_pair(channel)
    }

    fn subscribe(&self, channel: &str, group_id: Option<GroupId>, priority_id: Option<PrioId>) {
        self.abstract_subscriber
            .subscribe(channel, group_id, priority_id)
    }

    fn unsubscribe(&self, channel: &str) {
        self.abstract_subscriber.unsubscribe(channel)
    }

    fn on_message(&self, channel: SyncString, msg: SyncString) {
        println!(
            "{} received message: {} from channel: {}",
            self.get_name(),
            msg,
            channel
        );
    }
}

struct TestSubscriber2 {
    abstract_subscriber: abstract_subscriber::AbstractSubscriber,
}

impl TestSubscriber2 {
    pub fn new() -> Arc<RwLock<Self>> {
        let subscriber_objs = abstract_subscriber::get_objs();
        let mut subscriber_objs = subscriber_objs.write().expect("get subscriber objs failed");
        let subscriber = Arc::new(RwLock::new(Self {
            abstract_subscriber: abstract_subscriber::AbstractSubscriber::new(
                subscriber_objs.len() as i32,
            ),
        }));
        subscriber_objs.push(subscriber.clone());
        subscriber
    }
}

impl Display for TestSubscriber2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", type_name::<Self>())
    }
}

impl Subscriber for TestSubscriber2 {
    fn id(&self) -> i32 {
        self.abstract_subscriber.id()
    }

    fn get_grp_prio_pair(&self, channel: &str) -> Option<GrpPrioPair> {
        self.abstract_subscriber.get_grp_prio_pair(channel)
    }

    fn subscribe(&self, channel: &str, group_id: Option<GroupId>, priority_id: Option<PrioId>) {
        self.abstract_subscriber
            .subscribe(channel, group_id, priority_id)
    }

    fn unsubscribe(&self, channel: &str) {
        self.abstract_subscriber.unsubscribe(channel)
    }

    fn on_message(&self, channel: SyncString, msg: SyncString) {
        println!(
            "{} received message: {} from channel: {}",
            self.get_name(),
            msg,
            channel
        );
    }
}

struct TestSubscriber3 {
    abstract_subscriber: abstract_subscriber::AbstractSubscriber,
}

impl TestSubscriber3 {
    pub fn new() -> Arc<RwLock<Self>> {
        let subscriber_objs = abstract_subscriber::get_objs();
        let mut subscriber_objs = subscriber_objs.write().expect("get subscriber objs failed");
        let subscriber = Arc::new(RwLock::new(Self {
            abstract_subscriber: abstract_subscriber::AbstractSubscriber::new(
                subscriber_objs.len() as i32,
            ),
        }));
        subscriber_objs.push(subscriber.clone());
        subscriber
    }
}

impl Display for TestSubscriber3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", type_name::<Self>())
    }
}

impl Subscriber for TestSubscriber3 {
    fn id(&self) -> i32 {
        self.abstract_subscriber.id()
    }

    fn get_grp_prio_pair(&self, channel: &str) -> Option<GrpPrioPair> {
        self.abstract_subscriber.get_grp_prio_pair(channel)
    }

    fn subscribe(&self, channel: &str, group_id: Option<GroupId>, priority_id: Option<PrioId>) {
        self.abstract_subscriber
            .subscribe(channel, group_id, priority_id)
    }

    fn unsubscribe(&self, channel: &str) {
        self.abstract_subscriber.unsubscribe(channel)
    }

    fn on_message(&self, channel: SyncString, msg: SyncString) {
        println!(
            "{} received message: {} from channel: {}",
            self.get_name(),
            msg,
            channel
        );
    }
}

struct TestSubscriber4 {
    abstract_subscriber: abstract_subscriber::AbstractSubscriber,
}

impl TestSubscriber4 {
    pub fn new() -> Arc<RwLock<Self>> {
        let subscriber_objs = abstract_subscriber::get_objs();
        let mut subscriber_objs = subscriber_objs.write().expect("get subscriber objs failed");
        let subscriber = Arc::new(RwLock::new(Self {
            abstract_subscriber: abstract_subscriber::AbstractSubscriber::new(
                subscriber_objs.len() as i32,
            ),
        }));
        subscriber_objs.push(subscriber.clone());
        subscriber
    }
}

impl Display for TestSubscriber4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", type_name::<Self>())
    }
}

impl Subscriber for TestSubscriber4 {
    fn id(&self) -> i32 {
        self.abstract_subscriber.id()
    }

    fn get_grp_prio_pair(&self, channel: &str) -> Option<GrpPrioPair> {
        self.abstract_subscriber.get_grp_prio_pair(channel)
    }

    fn subscribe(&self, channel: &str, group_id: Option<GroupId>, priority_id: Option<PrioId>) {
        self.abstract_subscriber
            .subscribe(channel, group_id, priority_id)
    }

    fn unsubscribe(&self, channel: &str) {
        self.abstract_subscriber.unsubscribe(channel)
    }

    fn on_message(&self, channel: SyncString, msg: SyncString) {
        println!(
            "{} received message: {} from channel: {}",
            self.get_name(),
            msg,
            channel
        );
        let grp_prio_pair_now = self.get_grp_prio_pair(&channel);
        println!(
            "{} grp_prio_pair_now: {:?}",
            self.get_name(),
            grp_prio_pair_now
        );
        publisher::publish(
            &channel,
            grp_prio_pair_now.map(|grp_prio_pair| grp_prio_pair.grp_id),
            grp_prio_pair_now.map(|grp_prio_pair| grp_prio_pair.priority_id - 1),
            Arc::new(String::from("hello")),
        );
    }
}

#[test]
fn test_pubsub_simple() {
    let channel_1 = channel::get_channel("channel_1");
    let subscriber_1 = TestSubscriber1::new();
    subscriber_1
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_1", None, None);
    let subscriber_2 = TestSubscriber2::new();
    subscriber_2
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_1", None, None);
    let subscriber_3 = TestSubscriber3::new();
    let pair = channel_1
        .get_grp_prio_pair(subscriber_2.read().expect("read subscriber failed").id())
        .expect("get grp prio pair failed");
    subscriber_3
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_1", Some(pair.grp_id), Some(pair.priority_id + 1));

    publisher::publish("channel_1", None, None, Arc::new(String::from("hello")));

    thread::sleep(std::time::Duration::from_secs(1));
}

#[test]
fn test_pubsub_hard() {
    let channel_1 = channel::get_channel("channel_1");
    let subscriber_1 = TestSubscriber1::new();
    subscriber_1
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_1", None, None);
    let subscriber_2 = TestSubscriber2::new();
    subscriber_2
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_1", None, None);
    let subscriber_3 = TestSubscriber3::new();
    let pair = channel_1
        .get_grp_prio_pair(subscriber_2.read().expect("read subscriber failed").id())
        .expect("get grp prio pair failed");
    subscriber_3
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_1", Some(pair.grp_id), Some(pair.priority_id + 1));
    let subscriber_4 = TestSubscriber4::new();
    let pair = channel_1
        .get_grp_prio_pair(subscriber_3.read().expect("read subscriber failed").id())
        .expect("get grp prio pair failed");
    subscriber_4
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_1", Some(pair.grp_id), Some(pair.priority_id));

    thread::spawn(|| {
        publisher::publish(
            "channel_1",
            None,
            None,
            Arc::new(String::from("hello from son thread")),
        );
    });
    publisher::publish("channel_1", None, None, Arc::new(String::from("hello")));

    thread::sleep(std::time::Duration::from_secs(1));
}

#[test]
fn test_pub_with_mutil_channel() {
    let channel_1 = channel::get_channel("channel_1");
    let subscriber_1 = TestSubscriber1::new();
    subscriber_1
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_1", None, None);
    let subscriber_2 = TestSubscriber2::new();
    subscriber_2
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_1", None, None);
    let subscriber_3 = TestSubscriber3::new();
    let pair = channel_1
        .get_grp_prio_pair(subscriber_2.read().expect("read subscriber failed").id())
        .expect("get grp prio pair failed");
    subscriber_3
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_1", Some(pair.grp_id), Some(pair.priority_id + 1));
    let subscriber_4 = TestSubscriber4::new();
    let pair = channel_1
        .get_grp_prio_pair(subscriber_3.read().expect("read subscriber failed").id())
        .expect("get grp prio pair failed");
    subscriber_4
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_1", Some(pair.grp_id), Some(pair.priority_id));

    let _channel_2 = channel::get_channel("channel_2");
    subscriber_1
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_2", None, None);
    subscriber_2
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_2", None, None);
    subscriber_3
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_2", None, None);
    subscriber_4
        .read()
        .expect("read subscriber failed")
        .subscribe("channel_2", None, None);

    thread::spawn(|| {
        publisher::publish(
            "channel_1",
            None,
            None,
            Arc::new(String::from("hello from son thread")),
        );
    });

    publisher::publish("channel_2", None, None, Arc::new(String::from("hello")));

    thread::sleep(std::time::Duration::from_secs(1));
}
