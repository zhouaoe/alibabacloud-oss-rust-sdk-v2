#[derive(Debug, PartialEq)]
pub(crate) enum RequestFieldType {
    Header,
    Query,
}

impl std::str::FromStr for RequestFieldType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "header" => Ok(RequestFieldType::Header),
            "query" => Ok(RequestFieldType::Query),
            _ => Err(format!("Invalid field type: {}", s)),
        }
    }
}

impl std::fmt::Display for RequestFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestFieldType::Header => write!(f, "header"),
            RequestFieldType::Query => write!(f, "query"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_header() {
        let field_type: RequestFieldType = "header".parse().unwrap();
        assert_eq!(field_type, RequestFieldType::Header);
    }

    #[test]
    fn test_from_str_query() {
        let field_type: RequestFieldType = "query".parse().unwrap();
        assert_eq!(field_type, RequestFieldType::Query);
    }

    #[test]
    fn test_from_str_invalid() {
        let result: Result<RequestFieldType, String> = "invalid".parse();
        assert_eq!(result, Err("Invalid field type: invalid".to_string()));
    }

    #[test]
    fn test_display_header() {
        let field_type = RequestFieldType::Header;
        assert_eq!(format!("{}", field_type), "header");
    }

    #[test]
    fn test_display_query() {
        let field_type = RequestFieldType::Query;
        assert_eq!(format!("{}", field_type), "query");
    }
}
