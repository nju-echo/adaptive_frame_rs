use std::sync::Arc;
use std::thread;

use dashmap::DashMap;

use common::SyncString;

use crate::pubsub::channel::SubscriberSet;
use crate::pubsub::grp_prio_pair::{GroupId, PrioId};
use crate::pubsub::{abstract_subscriber, channel};

/// publish
/// publish message to channel
/// if group_id is specified, publish to this group
/// if group_id is not specified, publish to all groups
/// if prio_id is specified, publish to this priority and below
/// if prio_id is not specified, publish to all priorities
pub fn publish(channel: &str, group_id: Option<GroupId>, prio_id: Option<PrioId>, msg: SyncString) {
    let prio_id = prio_id.unwrap_or(PrioId::MAX);
    let channel_name = Arc::new(channel.to_string());
    let channel = channel::get_channel(channel);
    let subscribers = channel.get_subscribers();

    if let Some(group_id) = group_id {
        //if group_id is specified, publish to this group
        publish_to_group(channel_name, group_id, prio_id, msg, subscribers);
    } else {
        //if group_id is not specified, publish to all groups
        for group in subscribers.iter() {
            publish_to_group(
                channel_name.clone(),
                *group.key(),
                prio_id,
                msg.clone(),
                subscribers,
            );
        }
    }
}

/// publish to specified group and priority
fn publish_to_group(
    channel_name: SyncString,
    group_id: GroupId,
    prio_id: PrioId,
    msg: SyncString,
    subscribers: &DashMap<GroupId, DashMap<PrioId, SubscriberSet>>,
) {
    let grp = subscribers.get(&group_id);
    if let Some(grp) = grp {
        let mut max_prio = PrioId::MIN;
        for prio in grp.iter() {
            if prio.key() > &prio_id {
                continue;
            }
            max_prio = max_prio.max(*prio.key());
        }

        if max_prio > PrioId::MIN {
            let prio = grp.get(&max_prio).expect("get priority failed");
            for subscriber_id in prio.iter() {
                let subscriber = abstract_subscriber::get_subscriber(*subscriber_id)
                    .expect("get subscriber failed");
                let channel_name_in = channel_name.clone();
                let msg_in = msg.clone();
                thread::spawn(move || {
                    subscriber
                        .read()
                        .expect("read subscriber failed")
                        .on_message(channel_name_in, msg_in);
                });
            }
        }
    }
}
