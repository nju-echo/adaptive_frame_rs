use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum CmdType {
    Reset,
    Start,
    Stop,
    //Pause,
    //Load,
    //Save,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(CmdType::from_str("reset").unwrap(), CmdType::Reset);
        assert_eq!(CmdType::from_str("start").unwrap(), CmdType::Start);
        assert_eq!(CmdType::from_str("stop").unwrap(), CmdType::Stop);
        //assert_eq!(CmdType::from_str("pause").unwrap(), CmdType::Pause);
        //assert_eq!(CmdType::from_str("load").unwrap(), CmdType::Load);
        //assert_eq!(CmdType::from_str("save").unwrap(), CmdType::Save);

        if let Err(e) = CmdType::from_str("reset1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse reset1");
        }

        println!("{}", CmdType::from_str("reset").unwrap());
    }
}
