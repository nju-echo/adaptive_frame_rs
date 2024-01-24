/// pubsub is a module that implements publish/subscribe pattern.
/// It is used to transfer message between different modules in platform.
/// # usage
/// It is used by crate: platform
/// it contains channel, grp_prio_pair, publisher, subscriber and abstract_subscriber
/// ## channel
/// channel is a struct that stores subscribers.
/// it manage subscribers by group and priority.
/// ## grp_prio_pair
/// grp_prio_pair is a struct that stores group id and priority id.
/// ## publisher
/// publisher is a module that provides publish function.
/// one can use it to publish message to subscribers through channel.
/// ## subscriber
/// subscriber is a module that provides subscribe trait
/// trait provides on_message, id, get_grp_prio_pair, subscribe, unsubscribe and get_name.
/// ## abstract_subscriber
/// abstract_subscriber is a struct that implements subscribe trait.
/// it is an abstract struct that can be used to implement concrete subscriber.
pub mod abstract_subscriber;
pub mod channel;
pub mod grp_prio_pair;
pub mod publisher;
pub mod subscriber;

//todo: 维护一下代码风格的一致性，修改一下部分函数和结构体的命名，参数和返回值
