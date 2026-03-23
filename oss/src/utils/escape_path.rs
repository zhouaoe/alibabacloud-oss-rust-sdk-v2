lazy_static::lazy_static! {
    static ref NO_ESCAPE: [bool; 256] = {
        let mut no_escape = [false; 256];
        for (index, item) in no_escape.iter_mut().enumerate() {
            let c = index as u8;
            *item = c.is_ascii_alphanumeric() || c == b'-' || c == b'.' || c == b'_' || c == b'~';
        }
        no_escape
    };
}

/// Escapes a path by replacing characters that need to be escaped with their
/// percent-encoded representation. If `encode_sep` is `true`, the path
/// separator '/' will also be percent-encoded.
pub(crate) fn escape_path(path: &str, encode_sep: bool) -> String {
    path.chars()
        .map(|c| {
            if NO_ESCAPE[c as usize] || (c == '/' && !encode_sep) {
                c.to_string()
            } else {
                format!("%{:02X}", c as u32)
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_path_no_escape() {
        let path = "abc-123";
        let encoded = escape_path(path, true);
        assert_eq!(encoded, "abc-123");
    }

    #[test]
    fn test_escape_path_escape() {
        let path = "abc!@#";
        let encoded = escape_path(path, true);
        assert_eq!(encoded, "abc%21%40%23");
    }

    #[test]
    fn test_escape_path_no_encode_sep() {
        let path = "abc/123";
        let encoded = escape_path(path, false);
        assert_eq!(encoded, "abc/123");
    }

    #[test]
    fn test_escape_path_encode_sep() {
        let path = "abc/123";
        let encoded = escape_path(path, true);
        assert_eq!(encoded, "abc%2F123");
    }
}
