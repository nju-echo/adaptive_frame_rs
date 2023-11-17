#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CmdType {
    Reset,
    Start,
    Stop,
    Pause,
    Load,
    Save,
}

impl CmdType {
    pub fn from_string(cmd_type_str: &str) -> Result<CmdType, String> {
        match cmd_type_str.to_lowercase().as_str() {
            "reset" => Ok(CmdType::Reset),
            "start" => Ok(CmdType::Start),
            "stop" => Ok(CmdType::Stop),
            "pause" => Ok(CmdType::Pause),
            "load" => Ok(CmdType::Load),
            "save" => Ok(CmdType::Save),
            other => Err(format!("No constant with text {} found", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(CmdType::from_string("Reset").unwrap(), CmdType::Reset);
        assert_eq!(CmdType::from_string("Start").unwrap(), CmdType::Start);
        assert_eq!(CmdType::from_string("Stop").unwrap(), CmdType::Stop);
        assert_eq!(CmdType::from_string("Pause").unwrap(), CmdType::Pause);
        assert_eq!(CmdType::from_string("Load").unwrap(), CmdType::Load);
        assert_eq!(CmdType::from_string("Save").unwrap(), CmdType::Save);
        if let Err(e) = CmdType::from_string("Reset1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse Reset1");
        }
    }
}
