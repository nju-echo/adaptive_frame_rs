use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetState {
    pub state: bool,
}

impl SetState {
    pub fn new() -> SetState {
        SetState { state: false }
    }

    pub fn new_with_state(state: bool) -> SetState {
        SetState { state }
    }

    pub fn get(&self) -> bool {
        self.state
    }

    pub fn set(&mut self, state: bool) {
        self.state = state;
    }
}

impl FromStr for SetState {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "true" => Ok(SetState::new_with_state(true)),
            _ => Ok(SetState::new_with_state(false)),
        }
        //should not reach here
        //Err("Invalid state")
    }
}

impl fmt::Display for SetState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(
            SetState::from_str("TrUe").unwrap(),
            SetState::new_with_state(true)
        );
        assert_eq!(
            SetState::from_str("false").unwrap(),
            SetState::new_with_state(false)
        );

        println!("{}", SetState::from_str("true").unwrap());
        println!("{}", SetState::from_str("yes").unwrap());
    }
}
