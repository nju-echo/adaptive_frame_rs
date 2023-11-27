use std::time::Duration;

use crate::structs::sync::BlockingQueue;

#[derive(Debug)]
pub struct SynchronousString {
    queue: BlockingQueue<String>,
}

impl SynchronousString {
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

    pub fn blocktake(&self) -> Option<String> {
        self.queue.take()
    }

    pub fn block_take_timeout(&self, timeout: u64) -> Option<String> {
        self.queue.poll_timeout(Duration::from_millis(timeout))
    }

    pub fn non_block_take(&self) -> Option<String> {
        self.queue.poll()
    }

    pub fn put(&self, item: String) {
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
    use super::*;

    #[test]
    fn test_block_take() {
        let queue = SynchronousString::new();
        let item = "test".to_string();
        queue.put(item.clone());
        assert_eq!(queue.blocktake().unwrap(), item);
    }

    #[test]
    fn test_block_take_timeout() {
        let queue = SynchronousString::new();
        let item = "test".to_string();
        queue.put(item.clone());
        assert_eq!(queue.block_take_timeout(1000).unwrap(), item);
    }

    #[test]
    fn test_non_block_take() {
        let queue = SynchronousString::new();
        let item = "test".to_string();
        queue.put(item.clone());
        assert_eq!(queue.non_block_take().unwrap(), item);
    }

    #[test]
    fn test_put() {
        let queue = SynchronousString::new();
        let item = "test".to_string();
        queue.put(item.clone());
        assert_eq!(queue.blocktake().unwrap(), item);
    }

    #[test]
    fn test_size() {
        let queue = SynchronousString::new();
        let item = "test".to_string();
        queue.put(item.clone());
        assert_eq!(queue.size(), 1);
    }

    #[test]
    fn test_clear() {
        let queue = SynchronousString::new();
        let item = "test".to_string();
        queue.put(item.clone());
        queue.clear();
        assert_eq!(queue.size(), 0);
    }
}
