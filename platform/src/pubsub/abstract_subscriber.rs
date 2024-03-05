// notion: while java has cyclic references with garbage collection, rust has not
// so we need to use Weak<RwLock<Subscriber>> instead of Rc<RwLock<Subscriber>> for subscriber

use std::any::type_name;
use std::fmt::Display;
use std::sync::{Arc, RwLock};

use dashmap::DashMap;
use once_cell::sync::Lazy;

use crate::pubsub::channel;
use crate::pubsub::channel::ChannelName;
use crate::pubsub::grp_prio_pair::{GroupId, GrpPrioPair, PrioId};
use crate::pubsub::subscriber::Subscriber;

pub type SubscriberId = i32;
pub type SharedSubscriber = Arc<dyn Subscriber>;

/// SUBSCRIBER_OBJS is a static variable that stores all subscribers.
/// It is a vector of Arc<dyn Subscriber>.
/// outside can use index to get subscriber.
static SUBSCRIBER_OBJS: Lazy<RwLock<Vec<SharedSubscriber>>> = Lazy::new(|| RwLock::new(Vec::new()));

#[derive(Debug)]
pub struct AbstractSubscriber {
    ///channels stores all channels that this subscriber subscribes to.
    channels: DashMap<ChannelName, GrpPrioPair>,
    id: i32,
}

///display for AbstractSubscriber
impl Display for AbstractSubscriber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", type_name::<Self>())
    }
}

///impl Subscriber for AbstractSubscriber
impl Subscriber for AbstractSubscriber {
    fn super_reference(&self) -> &AbstractSubscriber {
        self
    }

    fn id(&self) -> i32 {
        self.id
    }

    fn get_grp_prio_pair(&self, channel: &str) -> Option<GrpPrioPair> {
        match self.channels.get(channel) {
            Some(grp_prio_pair) => Some(grp_prio_pair.clone()),
            None => None,
        }
    }

    fn subscribe(&self, channel: &str, group_id: Option<GroupId>, priority_id: Option<PrioId>) {
        let grp_prio_pair_now =
            channel::get_channel(channel).add_subscriber(self.id(), group_id, priority_id);
        self.channels.insert(channel.to_string(), grp_prio_pair_now);
    }

    fn unsubscribe(&self, channel: &str) {
        self.channels.remove(channel);
        channel::get_channel(channel).remove_subscriber(self.id());
    }
}

///impl AbstractSubscriber
impl AbstractSubscriber {
    ///used for add subscriber
    pub fn new(id: i32) -> Self {
        Self {
            channels: DashMap::new(),
            id,
        }
    }

    ///used outside
    pub fn add_to_subscriber_objs() -> Arc<Self> {
        let mut subscriber_objs = SUBSCRIBER_OBJS.write().expect("get subscriber objs failed");
        let subscriber = Arc::new(Self::new(subscriber_objs.len() as i32));
        subscriber_objs.push(subscriber.clone());
        subscriber
    }
}

///static function
pub fn get_objs() -> &'static RwLock<Vec<SharedSubscriber>> {
    &SUBSCRIBER_OBJS
}

pub fn get_subscriber(id: i32) -> Option<SharedSubscriber> {
    let subscriber_objs = SUBSCRIBER_OBJS.read().expect("get subscriber objs failed");
    subscriber_objs.get(id as usize).map(|x| x.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abstract_subscriber() {
        let subscriber = AbstractSubscriber::new(0);
        assert_eq!(subscriber.id(), 0);
        //assert_eq!(subscriber.to_string(), "pubsub::abstract_subscriber::AbstractSubscriber");
        assert_eq!(subscriber.get_grp_prio_pair("test"), None);
        subscriber.subscribe("test", None, None);
        assert_eq!(
            subscriber.get_grp_prio_pair("test"),
            Some(GrpPrioPair::new(0, 0))
        );
        subscriber.unsubscribe("test");
        assert_eq!(subscriber.get_grp_prio_pair("test"), None);
    }
}
