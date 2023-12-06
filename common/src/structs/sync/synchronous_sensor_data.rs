use std::time::Duration;

use crate::structs::sensor_data::SensorData;
use crate::structs::sync::BlockingQueue;

#[derive(Debug)]
pub struct SynchronousSensorData {
    queue: BlockingQueue<SensorData>,
}

impl SynchronousSensorData {
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

    pub fn block_take(&self) -> Option<SensorData> {
        self.queue.take()
    }

    pub fn block_take_timeout(&self, timeout: u64) -> Option<SensorData> {
        self.queue.poll_timeout(Duration::from_millis(timeout))
    }

    pub fn non_block_take(&self) -> Option<SensorData> {
        self.queue.poll()
    }

    pub fn put(&self, item: SensorData) {
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

    use crate::structs::enumeration::sensor_data_type::SensorDataType;
    use crate::structs::sensor_data::SensorData;

    use super::*;

    #[test]
    fn test_block_take() {
        let queue = SynchronousSensorData::new();
        let item = SensorData::new(
            SensorDataType::IncResult,
            vec!["test".to_string()],
            vec![json!("test")],
        );
        queue.put(item.clone());
        assert_eq!(queue.block_take().unwrap(), item);
    }

    #[test]
    fn test_block_take_timeout() {
        let queue = SynchronousSensorData::new();
        let item = SensorData::new(
            SensorDataType::IncResult,
            vec!["test".to_string()],
            vec![json!("test")],
        );
        queue.put(item.clone());
        assert_eq!(queue.block_take_timeout(1000).unwrap(), item);
    }
}
