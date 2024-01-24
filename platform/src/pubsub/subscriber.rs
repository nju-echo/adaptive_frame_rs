use std::fmt::Display;

use common::SyncString;

use crate::pubsub::grp_prio_pair::{GroupId, GrpPrioPair, PrioId};

pub trait Subscriber: Send + Sync + Display {
    ///on message depends on the type of subscriber
    /// it can be overrided by user
    fn on_message(&self, channel: SyncString, msg: SyncString) {
        println!(
            "default on message: subscriber {} receive message from channel {} with message {}",
            self.get_name(),
            channel,
            msg
        );
    }

    /// id is used to identify subscriber
    fn id(&self) -> i32;

    ///get group id and priority id
    fn get_grp_prio_pair(&self, channel: &str) -> Option<GrpPrioPair>;

    ///subscribe to channel
    fn subscribe(&self, channel: &str, group_id: Option<GroupId>, priority_id: Option<PrioId>);

    ///unsubscribe from channel
    fn unsubscribe(&self, channel: &str);

    /// get name
    fn get_name(&self) -> String {
        self.to_string()
    }
}
