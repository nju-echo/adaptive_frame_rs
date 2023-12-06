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
