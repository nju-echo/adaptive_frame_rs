use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimeNode<'a> {
    pub time: u64,
    //TDOO: can be a better data structure to fasten the remove operation
    pub app_names: Vec<&'a str>,
}

impl<'a> TimeNode<'a> {
    pub fn new(time: u64, app_grpid: &'a str) -> Self {
        Self {
            time,
            app_names: vec![app_grpid],
        }
    }
}

impl<'a> fmt::Display for TimeNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{:?}", self.time, self.app_names)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let time_node = TimeNode::new(1, "test");
        assert_eq!(time_node.time, 1);
        assert_eq!(time_node.app_names, vec!["test"]);
    }

    #[test]
    fn test_display() {
        let time_node = TimeNode::new(1, "test");
        println!("{}", time_node.app_names[0]);
        println!("{}", time_node);
        //assert_eq!(time_node.to_string(), "time: 1, app_names: [\"test\"]");
    }
}
