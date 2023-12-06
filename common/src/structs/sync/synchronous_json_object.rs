use std::time::Duration;

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

    pub fn block_take(&self) -> Option<Value> {
        self.queue.take()
    }

    pub fn block_take_timeout(&self, timeout: u64) -> Option<Value> {
        self.queue.poll_timeout(Duration::from_millis(timeout))
    }

    pub fn non_block_take(&self) -> Option<Value> {
        self.queue.poll()
    }

    pub fn put(&self, item: Value) {
        self.queue.put(item);
    }

    pub fn size(&self) -> usize {
        self.queue.size()
    }

    pub fn clear(&self) {
        self.queue.clear();
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_block_take() {
        let queue = SynchronousJsonObject::new();
        let item = json!({"name": "test"});
        queue.put(item.clone());
        assert_eq!(queue.block_take().unwrap(), item);
    }

    #[test]
    fn test_block_take_timeout() {
        let queue = SynchronousJsonObject::new();
        let item = json!({"name": "test"});
        queue.put(item.clone());
        assert_eq!(queue.block_take_timeout(100).unwrap(), item);
    }

    #[test]
    fn test_non_block_take() {
        let queue = SynchronousJsonObject::new();
        let item = json!({"name": "test"});
        queue.put(item.clone());
        assert_eq!(queue.non_block_take().unwrap(), item);
    }

    #[test]
    fn test_put() {
        let queue = SynchronousJsonObject::new();
        let item = json!({"name": "test"});
        queue.put(item.clone());
        assert_eq!(queue.block_take().unwrap(), item);
    }

    #[test]
    fn test_size() {
        let queue = SynchronousJsonObject::new();
        let item = json!({"name": "test"});
        queue.put(item.clone());
        assert_eq!(queue.size(), 1);
    }

    #[test]
    fn test_clear() {
        let queue = SynchronousJsonObject::new();
        let item = json!({"name": "test"});
        queue.put(item.clone());
        queue.clear();
        assert_eq!(queue.size(), 0);
    }
}
