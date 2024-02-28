use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CmdMessage {
    pub cmd: Option<String>,
    pub message: Option<String>,
}

impl CmdMessage {
    pub fn new(cmd: Option<String>, message: Option<String>) -> Self {
        Self { cmd, message }
    }
}

impl Display for CmdMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
