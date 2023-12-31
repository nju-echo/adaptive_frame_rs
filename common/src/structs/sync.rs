use std::fmt::Debug;
use std::sync::{Condvar, Mutex};
use std::time::Duration;

use concurrent_queue::ConcurrentQueue;

pub mod synchronous_json_object;
pub mod synchronous_sensor_data;
pub mod synchronous_set_state;
pub mod synchronous_string;

#[derive(Debug)]
pub struct BlockingQueue<T>
where
    T: Debug,
{
    queue: ConcurrentQueue<T>,
    lock: Mutex<()>,
    condvar: Condvar,
}

//impl use condvar.
//also can use flume::{receiver, sender}
//how to express a interrupt exception
impl<T: Debug> BlockingQueue<T> {
    /// Create a new blocking queue with unbounded capacity
    pub fn new() -> Self {
        Self {
            queue: ConcurrentQueue::unbounded(),
            lock: Mutex::new(()),
            condvar: Condvar::new(),
        }
    }

    /// Create a new blocking queue with bounded capacity
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            queue: ConcurrentQueue::bounded(capacity),
            lock: Mutex::new(()),
            condvar: Condvar::new(),
        }
    }

    //similar producer-consumer pattern
    ///put an item into the queue
    /// if the queue is full, block until the queue is not full
    /// # Panics
    /// if the queue is closed or mutex is poisoned, panic
    pub fn put(&self, item: T) {
        let mut lock = self.lock.lock().unwrap();

        //if queue is closed, panic
        if self.queue.is_closed() {
            panic!("queue is closed");
        }

        while self.queue.is_full() {
            lock = self.condvar.wait(lock).unwrap();
        }

        //if the queue is not full, push the item into the queue
        self.queue.push(item).unwrap();

        //notify all to avoid deadlock
        self.condvar.notify_all();
    }

    ///get an item from the queue
    /// if the queue is empty, block until the queue is not empty
    /// never return null
    /// # Panics
    /// if the queue is closed or mutex is poisoned, panic
    pub fn take(&self) -> Option<T> {
        let mut lock = self.lock.lock().unwrap();

        //if queue is closed, panic
        if self.queue.is_closed() {
            panic!("queue is closed");
        }

        while self.queue.is_empty() {
            lock = self.condvar.wait(lock).unwrap();
        }

        //if the queue is not empty, pop an item from the queue
        let item = self.queue.pop().unwrap();

        //notify all to avoid deadlock
        self.condvar.notify_all();

        Some(item)
    }

    ///get an item from the queue
    /// if the queue is empty, block until the queue is not empty or timeout
    /// # Panics
    /// if the queue is closed or mutex is poisoned, panic
    pub fn poll_timeout(&self, timeout: Duration) -> Option<T> {
        let mut lock = self.lock.lock().unwrap();

        //if queue is closed, panic
        if self.queue.is_closed() {
            panic!("queue is closed");
        }

        while self.queue.is_empty() {
            let (new_lock, result) = self.condvar.wait_timeout(lock, timeout).unwrap();
            lock = new_lock;
            if result.timed_out() {
                return None;
            }
        }

        //if the queue is not empty, pop an item from the queue
        let item = self.queue.pop().unwrap();

        //notify all to avoid deadlock
        self.condvar.notify_all();

        Some(item)
    }

    ///get an item from the queue
    /// if the queue is empty, return None
    /// # Panics
    /// if the queue is closed or mutex is poisoned, panic
    pub fn poll(&self) -> Option<T> {
        let _lock = self.lock.lock().unwrap();

        //if queue is closed, panic
        if self.queue.is_closed() {
            panic!("queue is closed");
        }

        if self.queue.is_empty() {
            return None;
        }

        //if the queue is not empty, pop an item from the queue
        let item = self.queue.pop().unwrap();

        //notify all to avoid deadlock
        self.condvar.notify_all();

        Some(item)
    }

    pub fn size(&self) -> usize {
        let _lock = self.lock.lock().unwrap();
        self.queue.len()
    }

    pub fn clear(&self) {
        let _lock = self.lock.lock().unwrap();

        //if queue is closed, panic
        if self.queue.is_closed() {
            panic!("queue is closed");
        }

        while !self.queue.is_empty() {
            self.queue.pop().unwrap();
        }

        self.condvar.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    use concurrent_queue::{PopError, PushError};

    use super::*;

    #[test]
    fn test_concurrent_queue_one_thread() {
        let queue = ConcurrentQueue::unbounded();
        queue.push(1).unwrap();
        queue.push(2).unwrap();
        queue.push(3).unwrap();
        assert_eq!(queue.pop(), Ok(1));
        assert_eq!(queue.pop(), Ok(2));
        assert_eq!(queue.pop(), Ok(3));
        assert_eq!(queue.pop(), Err(PopError::Empty));

        let queue = ConcurrentQueue::bounded(2);
        queue.push(1).unwrap();
        queue.push(2).unwrap();
        assert_eq!(queue.push(3), Err(PushError::Full(3)));
        assert_eq!(queue.pop(), Ok(1));
        assert_eq!(queue.pop(), Ok(2));
        assert_eq!(queue.pop(), Err(PopError::Empty));
        assert_eq!(queue.capacity(), Some(2));
        assert_eq!(queue.len(), 0);
        queue.close();
        assert_eq!(queue.pop(), Err(PopError::Closed));
    }

    #[test]
    fn test_concurrent_queue_mutiple_threads() {
        let queue = ConcurrentQueue::unbounded();
        let queue = Arc::new(queue);

        let mut handles = vec![];
        for i in 0..10 {
            let queue2 = queue.clone();
            let handle = std::thread::spawn(move || {
                queue2.push(i).unwrap();
                queue2.push(i).unwrap();
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("{:?}", queue);
        while let Ok(i) = queue.pop() {
            println!("{}", i);
        }
    }

    #[test]
    fn test_blocking_queue_one_thread() {
        let queue = BlockingQueue::new();
        queue.put(1);
        queue.put(2);
        queue.put(3);
        assert_eq!(queue.take(), Some(1));
        assert_eq!(queue.poll(), Some(2));
        assert_eq!(queue.poll_timeout(Duration::from_millis(100)), Some(3));
        assert_eq!(queue.poll(), None);

        let queue = BlockingQueue::new_with_capacity(2);
        queue.put(1);
        queue.put(2);
        //assert_eq!(queue.put(3), ());
        assert_eq!(queue.take(), Some(1));
        assert_eq!(queue.take(), Some(2));
        assert_eq!(queue.poll_timeout(Duration::from_millis(100)), None);
    }

    #[test]
    fn test_blocking_queue_mutiple_threads() {
        let queue = BlockingQueue::new_with_capacity(10);
        let queue = Arc::new(queue);

        let mut handles = vec![];
        for i in 0..10 {
            let queue2 = queue.clone();
            let handle = std::thread::spawn(move || loop {
                queue2.put(i);
            });
            handles.push(handle);

            let queue2 = queue.clone();
            let _ = std::thread::spawn(move || loop {
                println!("{:?}", queue2.take());
            });

            let queue2 = queue.clone();
            let _ = std::thread::spawn(move || loop {
                println!("{:?}", queue2.poll_timeout(Duration::from_millis(100)));
            });
        }

        thread::sleep(Duration::from_millis(100));

        println!("{:?}", queue);
        queue.clear();
        println!("{:?}", queue);
    }

    //need some more test for blocking queue
}
