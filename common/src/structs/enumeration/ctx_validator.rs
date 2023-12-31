use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Display, EnumString, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum CtxValidator {
    EccImd,
    EccGeas,
    PccImd,
    PccGeas,
    ConCImd,
    ConCGeas,
    Infuse,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(
            CtxValidator::from_str("eccimd").unwrap(),
            CtxValidator::EccImd
        );
        assert_eq!(
            CtxValidator::from_str("eccgeas").unwrap(),
            CtxValidator::EccGeas
        );
        assert_eq!(
            CtxValidator::from_str("pccimd").unwrap(),
            CtxValidator::PccImd
        );
        assert_eq!(
            CtxValidator::from_str("pccgeas").unwrap(),
            CtxValidator::PccGeas
        );
        assert_eq!(
            CtxValidator::from_str("concimd").unwrap(),
            CtxValidator::ConCImd
        );
        assert_eq!(
            CtxValidator::from_str("concgeas").unwrap(),
            CtxValidator::ConCGeas
        );
        assert_eq!(
            CtxValidator::from_str("infuse").unwrap(),
            CtxValidator::Infuse
        );

        if let Err(e) = CtxValidator::from_str("eccimd1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse eccimd1");
        }

        println!("{}", CtxValidator::from_str("eccimd").unwrap());
    }
}
