use std::time::Duration;

use crate::structs::set_state::SetState;
use crate::structs::sync::BlockingQueue;

#[derive(Debug)]
pub struct SynchronousSetState {
    queue: BlockingQueue<SetState>,
}

impl SynchronousSetState {
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

    pub fn block_take(&self) -> Option<SetState> {
        self.queue.take()
    }

    pub fn block_take_timeout(&self, timeout: u64) -> Option<SetState> {
        self.queue.poll_timeout(Duration::from_millis(timeout))
    }

    pub fn non_block_take(&self) -> Option<SetState> {
        self.queue.poll()
    }

    pub fn put(&self, item: SetState) {
        self.queue.put(item);
    }

    pub fn size(&self) -> usize {
        self.queue.size()
    }

    pub fn clear(&self) {
        self.queue.clear();
    }
}
