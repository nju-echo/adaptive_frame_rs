use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use crate::resource::sensor_mgr::SyncSensorMgr;

pub type RwLockOptionValueThread = RwLock<Option<ValueThread>>;

pub type SyncMutexStop = Arc<Mutex<bool>>;

/// notion: value thread is a thread that get value from sensor
/// its mode can be seen as a producer
/// struct type:
/// {
///   shared_data: Arc<???>
///   thread: Option<thread::JoinHandle<()>>
/// }
#[derive(Debug)]
pub struct ValueThread {
    should_stop: SyncMutexStop,
    stopped: SyncMutexStop,
    //sensor_mgr_weak: WeakSensorMgr,
    thread: Option<thread::JoinHandle<()>>,
}

impl ValueThread {
    /// new a value thread
    /// it also run the thread
    pub fn new(sensor_mgr: SyncSensorMgr) -> Self {
        let should_stop = Arc::new(Mutex::new(false));
        let stopped = Arc::new(Mutex::new(true));

        let join_handle = thread::spawn(move || todo!());

        Self {
            should_stop,
            stopped,
            //      sensor_mgr_weak: Arc::downgrade(&sensor_mgr),
            thread: Some(join_handle),
        }
    }

    /// how thread run
    fn run(should_stop: SyncMutexStop, stopped: SyncMutexStop, sensor_mgr: SyncSensorMgr) {
        todo!()
    }

    pub fn stop_thread(&self, sensor_mgr: SyncSensorMgr) {
        todo!()
    }
}
