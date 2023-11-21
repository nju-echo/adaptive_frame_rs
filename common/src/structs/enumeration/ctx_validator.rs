#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CtxValidator {
    EccImd,
    EccGeas,
    PccImd,
    PccGeas,
    ConCImd,
    ConCGeas,
    Infuse,
}

impl CtxValidator {
    pub fn from_string(type_str: &str) -> Result<CtxValidator, String> {
        match type_str.to_lowercase().as_str() {
            "ecc_imd" => Ok(CtxValidator::EccImd),
            "ecc_geas" => Ok(CtxValidator::EccGeas),
            "pcc_imd" => Ok(CtxValidator::PccImd),
            "pcc_geas" => Ok(CtxValidator::PccGeas),
            "conc_imd" => Ok(CtxValidator::ConCImd),
            "conc_geas" => Ok(CtxValidator::ConCGeas),
            "infuse" => Ok(CtxValidator::Infuse),
            other => Err(format!("No constant with text {} found", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(
            CtxValidator::from_string("Ecc_imd").unwrap(),
            CtxValidator::EccImd
        );
        assert_eq!(
            CtxValidator::from_string("ecC_geas").unwrap(),
            CtxValidator::EccGeas
        );
        assert_eq!(
            CtxValidator::from_string("pcc_imd").unwrap(),
            CtxValidator::PccImd
        );
        assert_eq!(
            CtxValidator::from_string("pcc_geas").unwrap(),
            CtxValidator::PccGeas
        );
        assert_eq!(
            CtxValidator::from_string("conC_imd").unwrap(),
            CtxValidator::ConCImd
        );
        assert_eq!(
            CtxValidator::from_string("conc_Geas").unwrap(),
            CtxValidator::ConCGeas
        );
        assert_eq!(
            CtxValidator::from_string("infuse").unwrap(),
            CtxValidator::Infuse
        );
        if let Err(e) = CtxValidator::from_string("ecc_imd1") {
            println!("{}", e);
        } else {
            panic!("Should not be able to parse ecc_imd1");
        }
    }
}
