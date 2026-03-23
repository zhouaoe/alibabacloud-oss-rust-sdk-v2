#[derive(Debug, PartialEq)]
pub(crate) enum ResultFieldType {
    Header,
}

impl std::str::FromStr for ResultFieldType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "header" => Ok(ResultFieldType::Header),
            _ => Err(format!("Invalid field type: {}", s)),
        }
    }
}

impl std::fmt::Display for ResultFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultFieldType::Header => write!(f, "header"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_header() {
        let field_type: ResultFieldType = "header".parse().unwrap();
        assert_eq!(field_type, ResultFieldType::Header);
    }

    #[test]
    fn test_from_str_invalid() {
        let result: Result<ResultFieldType, String> = "invalid".parse();
        assert_eq!(result, Err("Invalid field type: invalid".to_string()));
    }

    #[test]
    fn test_display_header() {
        let field_type = ResultFieldType::Header;
        assert_eq!(format!("{}", field_type), "header");
    }
}
