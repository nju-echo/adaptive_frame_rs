//todo: if a subscriber subscriber a channel with different group id without remove, it can be a error
// it depends on outside setting
// so inside we do not modify it until we have a better idea

use std::fmt::Display;

use dashmap::mapref::one::Ref;
use dashmap::{DashMap, DashSet};
use once_cell::sync::Lazy;

use crate::pubsub::abstract_subscriber::SubscriberId;
use crate::pubsub::grp_prio_pair::{GroupId, GrpPrioPair, PrioId};

/// #notion
/// in rust default Eq and Hash is based on content of struct
/// in java equals and hashcode is based on address of object
/// #solution
/// use index to represent object

/// type

pub type SubscriberSet = DashSet<SubscriberId>;
pub type ChannelName = String;

pub struct Channel {
    subscribers: DashMap<GroupId, DashMap<PrioId, SubscriberSet>>,
    channel_base_name: ChannelName,
}

/// static
static CHANNEL_OBJS: Lazy<DashMap<ChannelName, Channel>> = Lazy::new(|| DashMap::new());

pub static DEFAULT_GRP_ID: i32 = 0;
pub static DEFAULT_PRIO_ID: i32 = 0;

/// const
pub const SENSOR_SUFFIX: &str = "<SENSOR>";
pub const ACTOR_SUFFIX: &str = "<ACTOR>";
pub const SENSOR_REQUEST_SUFFIX: &str = "<SENSOR_REQUEST>";
pub const ACTOR_REQUEST_SUFFIX: &str = "<ACTOR_REQUEST>";

impl Channel {
    /// public function
    /// should not be used out of pubsub module

    pub fn new(channel_base_name: ChannelName) -> Option<Ref<'static, ChannelName, Channel>> {
        let channel = Self {
            subscribers: DashMap::new(),
            channel_base_name: channel_base_name.clone(),
        };
        CHANNEL_OBJS.insert(channel_base_name.clone(), channel);
        CHANNEL_OBJS.get(&channel_base_name)
    }

    ///getter
    pub fn get_name(&self) -> &str {
        &self.channel_base_name
    }

    pub fn get_subscribers(&self) -> &DashMap<GroupId, DashMap<PrioId, SubscriberSet>> {
        &self.subscribers
    }

    pub fn get_group_subscribers(
        &self,
        group_id: i32,
    ) -> Option<Ref<GroupId, DashMap<PrioId, SubscriberSet>>> {
        self.subscribers.get(&group_id)
    }

    ///get grp_prio_pair
    pub fn get_grp_prio_pair(&self, subscriber: SubscriberId) -> Option<GrpPrioPair> {
        for grp in self.subscribers.iter() {
            for prio in grp.value().iter() {
                if prio.value().contains(&subscriber) {
                    return Some(GrpPrioPair::new(*grp.key(), *prio.key()));
                }
            }
        }
        None
    }

    /// remove subscriber
    pub fn remove_subscriber(&self, subscriber: SubscriberId) {
        let grp_prio_pair_now = self.get_grp_prio_pair(subscriber);
        if let Some(grp_prio_pair_now) = grp_prio_pair_now {
            let group_id = grp_prio_pair_now.grp_id;
            let priority_id = grp_prio_pair_now.priority_id;
            self.subscribers
                .get(&group_id)
                .expect("get group failed")
                .get(&priority_id)
                .expect("get priority failed")
                .remove(&subscriber);
            if self
                .subscribers
                .get(&group_id)
                .expect("get group failed")
                .get(&priority_id)
                .expect("get priority failed")
                .is_empty()
            {
                self.subscribers
                    .get(&group_id)
                    .expect("get group failed")
                    .remove(&priority_id);
            }
            if self
                .subscribers
                .get(&group_id)
                .expect("get group failed")
                .is_empty()
            {
                self.subscribers.remove(&group_id);
            }
        }
    }

    /// add subscriber
    /// if group id is none, generate new group id
    /// if prio id is none, use DEFAULT_PRIO_ID
    pub fn add_subscriber(
        &self,
        subscriber: SubscriberId,
        group_id: Option<GroupId>,
        prio_id: Option<PrioId>,
    ) -> GrpPrioPair {
        let group_id = match group_id {
            Some(group_id) => group_id,
            None => self.gen_new_group_id(),
        };
        let prio_id = match prio_id {
            Some(prio_id) => prio_id,
            None => DEFAULT_PRIO_ID,
        };

        if !self.subscribers.contains_key(&group_id) {
            self.subscribers.insert(group_id, DashMap::new());
        }
        let group = self.subscribers.get(&group_id).expect("get group failed");

        if !group.contains_key(&prio_id) {
            group.insert(prio_id, DashSet::new());
        }
        let prio = group.get(&prio_id).expect("get prio failed");
        prio.insert(subscriber);

        GrpPrioPair::new(group_id, prio_id)
    }

    /// private function

    /// generate new group id
    /// if no subscriber, return DEFAULT_GRP_ID
    /// else return max group id + 1
    fn gen_new_group_id(&self) -> GroupId {
        let mut group_id = DEFAULT_GRP_ID;

        if self.subscribers.is_empty() {
            return group_id;
        }

        self.subscribers.iter().for_each(|group| {
            if group.key() > &group_id {
                group_id = *group.key();
            }
        });

        group_id + 1
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "channel name: {}", self.channel_base_name)
    }
}

///below is static function

// getter is used rarely
// so its efficiency is not important

/// get channel by channel base name
/// if channel not exist, create it
pub fn get_channel(channel_base_name: &str) -> Ref<'static, String, Channel> {
    match CHANNEL_OBJS.get(channel_base_name) {
        Some(channel) => channel,
        None => Channel::new(channel_base_name.to_string()).expect("create channel failed"),
    }
}

/// get channel name with suffix
fn get_channel_name_with_suffix(base_name: &str, suffix: &str) -> ChannelName {
    let channel_name = format!("{}{}", base_name, suffix);
    get_channel(&channel_name).get_name().to_string()
}

/// get sensor
pub fn get_sensor(sensor_name: &str) -> ChannelName {
    get_channel_name_with_suffix(sensor_name, SENSOR_SUFFIX)
}

///get actor
pub fn get_actor(actor_name: &str) -> ChannelName {
    get_channel_name_with_suffix(actor_name, ACTOR_SUFFIX)
}

///get sensor request
pub fn get_sensor_request(sensor_name: &str) -> ChannelName {
    get_channel_name_with_suffix(sensor_name, SENSOR_REQUEST_SUFFIX)
}

///get actor request
pub fn get_actor_request(actor_name: &str) -> ChannelName {
    get_channel_name_with_suffix(actor_name, ACTOR_REQUEST_SUFFIX)
}

///get Channel objs
pub fn get_objs() -> &'static DashMap<ChannelName, Channel> {
    &CHANNEL_OBJS
}

///get grp id and prio id
pub fn get_grp_prio_pair(channel: &str, subscriber: SubscriberId) -> Option<GrpPrioPair> {
    get_channel(channel).get_grp_prio_pair(subscriber)
}

pub fn get_grp_prio_pair_with_channel(
    channel: &Channel,
    subscriber: SubscriberId,
) -> Option<GrpPrioPair> {
    channel.get_grp_prio_pair(subscriber)
}

#[cfg(test)]
mod tests {
    use crate::pubsub::channel::{
        get_actor, get_actor_request, get_channel, get_grp_prio_pair,
        get_grp_prio_pair_with_channel, get_sensor, get_sensor_request,
    };

    #[test]
    fn test_channel() {
        let channel = get_channel("test_channel");
        println!("{}", channel.value());
    }

    #[test]
    fn test_sensor() {
        let sensor = get_sensor("test_sensor");
        println!("{}", sensor);
    }

    #[test]
    fn test_actor() {
        let actor = get_actor("test_actor");
        println!("{}", actor);
    }

    #[test]
    fn test_sensor_request() {
        let sensor_request = get_sensor_request("test_sensor_request");
        println!("{}", sensor_request);
    }

    #[test]
    fn test_actor_request() {
        let actor_request = get_actor_request("test_actor_request");
        println!("{}", actor_request);
    }

    #[test]
    fn test_get_grp_prio_pair() {
        let channel = get_channel("test_get_grp_prio_pair");
        let grp_prio_pair = channel.add_subscriber(1, None, None);
        let grp_prio_pair_now = get_grp_prio_pair("test_get_grp_prio_pair", 1).unwrap();
        println!("{}", grp_prio_pair);
        assert_eq!(grp_prio_pair, grp_prio_pair_now);
    }

    #[test]
    fn test_get_grp_prio_pair_with_channel() {
        let channel = get_channel("test_get_grp_prio_pair_with_channel");
        let grp_prio_pair = channel.add_subscriber(1, None, None);
        let grp_prio_pair_now = get_grp_prio_pair_with_channel(&channel, 1).unwrap();
        println!("{}", grp_prio_pair);
        assert_eq!(grp_prio_pair, grp_prio_pair_now);
    }
}
