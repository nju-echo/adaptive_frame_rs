use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CmdMessage {
    pub cmd: Option<String>,
    pub message: Option<Value>,
}

impl CmdMessage {
    pub fn new(cmd: Option<String>, message: Option<Value>) -> Self {
        Self { cmd, message }
    }
}

impl Display for CmdMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //if cmd is none, print message only
        //if message is none, print cmd only
        //if both are none, print nothing
        //todo: how to avoid 二次转义
        /*match (&self.cmd, &self.message) {
            (Some(cmd), Some(message)) => write!(f, "{{\"cmd\":\"{}\" , \"message\": \"{}\"}}", cmd, message),
            (Some(cmd), None) => write!(f, "{{\"cmd\":\"{}\"}}", cmd),
            (None, Some(message)) => write!(f, "{{\"message\": \"{}\"}}", message),
            (None, None) => write!(f, ""),
        }*/
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
