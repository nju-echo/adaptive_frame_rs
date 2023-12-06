use serde::{Deserialize, Serialize};

use crate::socket::cmd_message::CmdMessage;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CmdMessageGrpIds {
    pub cmd: Option<String>,
    pub message: Option<String>,
    #[serde(rename = "grpIds")]
    pub grp_ids: Option<Vec<i32>>,
}

impl CmdMessageGrpIds {
    pub fn new(cmd: Option<String>, message: Option<String>, grp_ids: Option<Vec<i32>>) -> Self {
        Self {
            cmd,
            message,
            grp_ids,
        }
    }

    pub fn new_with_cmd_message(cmd: CmdMessage, grp_ids: Option<Vec<i32>>) -> Self {
        Self {
            cmd: cmd.cmd,
            message: cmd.message,
            grp_ids,
        }
    }

    //TODO:shuold return a Result or Option
    pub fn get_cmd_message(&self) -> CmdMessage {
        CmdMessage::new(self.cmd.clone(), self.message.clone())
    }
}
