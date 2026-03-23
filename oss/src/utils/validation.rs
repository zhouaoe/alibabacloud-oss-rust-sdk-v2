use std::collections::HashSet;
use std::str::FromStr;

use super::parse_range;

pub(crate) fn is_valid_region(region: &str) -> bool {
    if region.is_empty() {
        return false;
    }

    for v in region.chars() {
        if !v.is_ascii_lowercase() && !v.is_ascii_digit() && v != '-' {
            return false;
        }
    }
    true
}

pub(crate) fn is_valid_endpoint(endpoint: &str) -> bool {
    url::Url::from_str(endpoint).is_ok()
}

pub(crate) fn is_valid_bucket_name(bucket_name: &str) -> bool {
    let name_len = bucket_name.len();

    if !(3..=63).contains(&name_len) {
        return false;
    }

    if bucket_name.starts_with('-') || bucket_name.ends_with('-') {
        return false;
    }

    for v in bucket_name.chars() {
        if !v.is_ascii_lowercase() && !v.is_ascii_digit() && v != '-' {
            return false;
        }
    }
    true
}

pub(crate) fn is_valid_object_name(object_name: &str) -> bool {
    !object_name.is_empty()
}

#[allow(unused)]
pub(crate) fn is_valid_range(r: &str) -> bool {
    parse_range(r).is_ok()
}

pub(crate) fn is_valid_method(method: &str) -> bool {
    let supported_methods: HashSet<&str> = ["GET", "PUT", "HEAD", "POST", "DELETE", "OPTIONS"]
        .iter()
        .cloned()
        .collect();
    supported_methods.contains(method)
}

#[allow(unused)]
pub(crate) fn is_valid_copy_directive(value: &str) -> bool {
    let supported_copy_directives: HashSet<&str> = ["COPY", "REPLACE"].iter().cloned().collect();
    supported_copy_directives.contains(&value.to_uppercase().as_str())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_region() {
        assert!(is_valid_region("hangzhou-1"));

        assert!(!is_valid_region(""));
        assert!(!is_valid_region("hangzhou_1"));
    }

    #[test]
    fn test_is_valid_endpoint() {
        assert!(is_valid_endpoint("https://example.com"));
        assert!(is_valid_endpoint("http://example.com"));
        assert!(is_valid_endpoint("ftp://example.com"));
        assert!(is_valid_endpoint("http://localhost"));
        assert!(is_valid_endpoint("http://127.0.0.1"));
        assert!(is_valid_endpoint("http://[::1]"));

        assert!(!is_valid_endpoint("example.com"));
    }

    #[test]
    fn test_is_valid_bucket_name() {
        assert!(is_valid_bucket_name("bucket-name-very-long-123"));

        assert!(!is_valid_bucket_name(""));
        assert!(!is_valid_bucket_name("-bucket"));
        assert!(!is_valid_bucket_name("bucket-"));
        assert!(!is_valid_bucket_name("bucket_"));
    }

    #[test]
    fn test_is_valid_object_name() {
        assert!(is_valid_object_name("object"));

        assert!(!is_valid_object_name(""));
    }

    #[test]
    fn test_is_valid_range() {
        assert!(is_valid_range("bytes=0-100"));

        assert!(!is_valid_range(""));
        assert!(!is_valid_range("bytes=invalid-100"));
    }

    #[test]
    fn test_is_valid_method() {
        assert!(is_valid_method("GET"));
        assert!(is_valid_method("PUT"));
        assert!(is_valid_method("HEAD"));
        assert!(is_valid_method("POST"));
        assert!(is_valid_method("DELETE"));
        assert!(is_valid_method("OPTIONS"));

        assert!(!is_valid_method(""));
        assert!(!is_valid_method("invalid"));
    }

    #[test]
    fn test_is_valid_copy_directive() {
        assert!(is_valid_copy_directive("COPY"));
        assert!(is_valid_copy_directive("REPLACE"));

        assert!(!is_valid_copy_directive(""));
        assert!(!is_valid_copy_directive("invalid"));
    }
}
