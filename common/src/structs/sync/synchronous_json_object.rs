use serde_json::Value;

use crate::structs::sync::BlockingQueue;

#[derive(Debug)]
struct SynchronousJsonObject {
    queue: BlockingQueue<Value>,
}

impl SynchronousJsonObject {
    pub fn new() -> Self {
        Self {
            queue: BlockingQueue::new(),
        }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            queue: BlockingQueue::new_with_capacity(capacity),
        }
    }

    pub fn blocktake() -> Option<Value> {}
}
