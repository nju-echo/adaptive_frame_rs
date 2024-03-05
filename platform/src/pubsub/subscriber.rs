use std::fmt::Display;

use common::SyncString;

use crate::pubsub::abstract_subscriber::AbstractSubscriber;
use crate::pubsub::grp_prio_pair::{GroupId, GrpPrioPair, PrioId};

pub trait Subscriber: Send + Sync + Display {
    fn super_reference(&self) -> &AbstractSubscriber;

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
    fn id(&self) -> i32 {
        self.super_reference().id()
    }

    ///get group id and priority id
    fn get_grp_prio_pair(&self, channel: &str) -> Option<GrpPrioPair> {
        self.super_reference().get_grp_prio_pair(channel)
    }

    ///subscribe to channel
    /// if group id is none, generate new group id
    /// if prio id is none, use DEFAULT_PRIO_ID
    fn subscribe(&self, channel: &str, group_id: Option<GroupId>, priority_id: Option<PrioId>) {
        self.super_reference()
            .subscribe(channel, group_id, priority_id);
    }

    ///unsubscribe from channel
    fn unsubscribe(&self, channel: &str) {
        self.super_reference().unsubscribe(channel);
    }

    /// get name
    fn get_name(&self) -> String {
        self.to_string()
    }
}
