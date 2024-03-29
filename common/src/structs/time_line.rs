use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::sync::{Arc, Condvar, Mutex};

use crate::structs::time_node::TimeNode;

//TODO: add an optimistic lock or unsafe to make sure for ownership

/// TimeLine is a struct to store the time nodes of all apps
/// its purpose is to provide a way to find the time node of a specific time with O(logn) time complexity

pub type SyncCondTimeLine = Arc<(Mutex<TimeLine>, Condvar)>;
pub type FrequencyType = u32;
pub type TimeType = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeLine {
    app_name_to_freq: HashMap<Arc<String>, u32>,
    time_nodes_map: BTreeMap<u64, TimeNode>,
}

impl Default for TimeLine {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeLine {
    pub fn new() -> Self {
        Self {
            time_nodes_map: BTreeMap::new(),
            app_name_to_freq: HashMap::new(),
        }
    }

    ///getter
    pub fn get_app_name_to_freq(&self) -> &HashMap<Arc<String>, u32> {
        &self.app_name_to_freq
    }

    //todo: may be need to modify
    pub fn get_nodes(&self) -> &BTreeMap<u64, TimeNode> {
        &self.time_nodes_map
    }

    pub fn get_time_node_mut(&mut self, time: u64) -> Option<&mut TimeNode> {
        self.time_nodes_map.get_mut(&time)
    }

    ///size
    pub fn size(&self) -> usize {
        self.app_name_to_freq.len()
    }

    ///insert
    pub fn insert(&mut self, time: u64, app_name: Arc<String>) {
        if let Some(time_node) = self.time_nodes_map.get_mut(&time) {
            time_node.app_names.push(app_name);
        } else {
            let time_node = TimeNode::new(time, app_name);
            self.time_nodes_map.insert(time, time_node);
        }
    }

    ///delete
    pub fn delete(&mut self, time: u64, app_name: Arc<String>) {
        if let Some(time_node) = self.time_nodes_map.get_mut(&time) {
            let index = time_node
                .app_names
                .iter()
                .position(|x| x.as_str() == app_name.as_str());
            if let Some(index) = index {
                //TODO: swap_remove or remove depends on the order of the app_names
                time_node.app_names.remove(index);
            }
            //if the time node is empty, remove it
            if time_node.app_names.is_empty() {
                self.time_nodes_map.remove(&time);
            }
        }
    }

    ///insert app_name with freq
    pub fn insert_with_freq(&mut self, app_name: Arc<String>, freq: u32) {
        if self.app_name_to_freq.contains_key(&app_name) {
            self.delete_with_freq(app_name.clone(), self.app_name_to_freq[&app_name]);
        }

        let sleep_time: f64 = 1000.0 / freq as f64;
        for i in 1..=freq {
            let time = (i as f64 * sleep_time).round() as u64;
            self.insert(time, app_name.clone());
        }
        self.app_name_to_freq.insert(app_name.clone(), freq);
    }

    ///delete app_name with freq
    pub fn delete_with_freq(&mut self, app_name: Arc<String>, freq: u32) {
        if self.app_name_to_freq.contains_key(&app_name) {
            //TODO: determine whether the freq is the same
            assert_eq!(self.app_name_to_freq[&app_name], freq);
            let sleep_time: f64 = 1000.0 / freq as f64;
            for i in 1..=freq {
                let time = (i as f64 * sleep_time).round() as u64;
                self.delete(time, app_name.clone());
            }
            self.app_name_to_freq.remove(&app_name);
        }
    }
}

impl<'a> fmt::Display for TimeLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "app_name_to_freq: {:?}\n time_nodes_map: {}",
            self.app_name_to_freq,
            self.time_nodes_map
                .values()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" -> ")
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    use super::*;

    #[test]
    fn test_time_line() {
        let mut time_line = TimeLine::new();
        time_line.insert(1, Arc::new("test1".to_string()));
        time_line.insert(1, Arc::new("test2".to_string()));
        time_line.insert(2, Arc::new("test3".to_string()));
        println!("{}", time_line);
        println!("{}", time_line.size());
        println!("{:?}", time_line.get_nodes());
        println!("{:?}", time_line.get_app_name_to_freq());
        println!("{:?}", time_line.get_time_node_mut(1));
        time_line.delete(1, Arc::new("test1".to_string()));
        println!("{}", time_line);
        time_line.delete(1, Arc::new("test2".to_string()));
        println!("{}", time_line);
        time_line.delete(1, Arc::new("test3".to_string()));
        println!("{}", time_line);
        time_line.insert_with_freq(Arc::new("test3".to_string()), 2);
        println!("{}", time_line);
        time_line.insert_with_freq(Arc::new("test3".to_string()), 3);
        println!("{}", time_line);
        time_line.delete_with_freq(Arc::new("test3".to_string()), 3);
        println!("{}", time_line);
    }

    #[test]
    fn test__() {
        let my_str: &'static str = "Hello, World!";

        // 将字符串切片包装在 Arc 中
        let arc_str: Arc<&str> = Arc::new(my_str);

        // 在多个线程中共享 Arc<&str>
        let thread_handle = {
            let arc_str = Arc::clone(&arc_str);
            thread::spawn(move || {
                println!("Thread 1: {}", *arc_str);
            })
        };

        // 在主线程中使用 Arc<&str>
        println!("Main Thread: {}", *arc_str);

        // 等待线程完成
        thread_handle.join().unwrap();
    }
}
