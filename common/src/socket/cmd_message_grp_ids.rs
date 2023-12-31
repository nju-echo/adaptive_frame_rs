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

    ///new_with_cmd_message
    /// will take ownership of cmd_message
    pub fn new_with_cmd_message(cmd: CmdMessage, grp_ids: Option<Vec<i32>>) -> Self {
        Self {
            cmd: cmd.cmd,
            message: cmd.message,
            grp_ids,
        }
    }

    //TODO: whether clone depends on frequency of on message
    pub fn get_cmd_message(&self) -> CmdMessage {
        CmdMessage::new(self.cmd.clone(), self.message.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_and_deserialize() {
        let cmd_message_grp_ids = CmdMessageGrpIds::new(
            Some("test".to_string()),
            Some("test".to_string()),
            Some(vec![1, 2, 3]),
        );

        let json_str = serde_json::to_string(&cmd_message_grp_ids).unwrap();
        println!("{}", json_str);

        let cmd_message_grp_ids: CmdMessageGrpIds = serde_json::from_str(&json_str).unwrap();
        println!("{:?}", cmd_message_grp_ids);

        let value = serde_json::json!({
            "cmd": "test",
            "message": "test",
            "grpIds": [1, 2, 3],
        });
        let cmd_message_grp_ids: CmdMessageGrpIds = serde_json::from_value(value).unwrap();
        println!("{:?}", cmd_message_grp_ids);
    }

    #[test]
    fn test_new_with_cmd_message() {
        let cmd_message = CmdMessage::new(Some("test".to_string()), Some("test".to_string()));
        let cmd_message_grp_ids =
            CmdMessageGrpIds::new_with_cmd_message(cmd_message, Some(vec![1, 2, 3]));
        println!("{:?}", cmd_message_grp_ids);
        println!("{:?}", cmd_message_grp_ids.get_cmd_message());
    }
}
