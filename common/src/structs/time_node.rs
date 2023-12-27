use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimeNode {
    pub time: u64,
    //TDOO: can be a better data structure to fasten the remove operation
    pub app_names: Vec<Arc<String>>,
}

impl TimeNode {
    pub fn new(time: u64, app_grpid: Arc<String>) -> Self {
        Self {
            time,
            app_names: vec![app_grpid],
        }
    }
}

impl<'a> fmt::Display for TimeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{:?}", self.time, self.app_names)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let time_node = TimeNode::new(1, Arc::new("test".to_string()));
        assert_eq!(time_node.time, 1);
        assert_eq!(time_node.app_names[0], Arc::new("test".to_string()));
    }

    #[test]
    fn test_display() {
        let time_node = TimeNode::new(1, Arc::new("test".to_string()));
        println!("{}", time_node.app_names[0]);
        println!("{}", time_node);
        //assert_eq!(time_node.to_string(), "time: 1, app_names: [\"test\"]");
    }
}
