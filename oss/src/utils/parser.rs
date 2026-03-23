use std::result::Result;

use http::HeaderMap;

use crate::utils::HTTPRange;
use crate::{HTTP_HEADER_CONTENT_LENGTH, HTTP_HEADER_CONTENT_RANGE};

/// Parses the offset and size from the headers and returns a tuple containing
/// the offset and size.
///
/// # Arguments
///
/// * `headers` - The `HeaderMap` containing the headers.
///
/// # Returns
///
/// Returns a `Result` containing a tuple `(i64, i64)` representing the offset
/// and size if successful, or a `Box<dyn std::error::Error>` if an error
/// occurred during parsing.
///
/// # Examples
///
/// ```ignore
/// # use http::HeaderMap;
/// # use alibabacloud_oss_sdk_rust_v2::utils::parser::parse_offset_and_size_from_headers;
/// #
/// let mut headers = HeaderMap::new();
/// headers.insert("Content-Length", "12345".parse().unwrap());
///
/// let result = parse_offset_and_size_from_headers(&headers).unwrap();
/// assert_eq!(result, (0, 12345));
/// ```
#[allow(unused)]
pub(crate) fn parse_offset_and_size_from_headers(
    headers: &HeaderMap,
) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let mut size = -1;

    if let Some(content_length_value) = headers.get(HTTP_HEADER_CONTENT_LENGTH) {
        match content_length_value.to_str()?.parse() {
            Ok(parsed_size) => size = parsed_size,
            Err(_) => return Err("Failed to parse Content-Length".into()),
        }
    }

    if let Some(content_range_value) = headers.get(HTTP_HEADER_CONTENT_RANGE) {
        if let Ok(content_range_str) = content_range_value.to_str() {
            if !content_range_str.starts_with("bytes ") {
                return Err("Content-Range does not start with 'bytes '".into());
            }

            let dash_pos = content_range_str.find('-').ok_or("Invalid Content-Range")?;

            // start offset
            let offset = content_range_str[6..dash_pos].parse().unwrap_or(-1);
            if offset == -1 {
                return Err("Failed to parse starting offset".into());
            }

            // total size
            let slash_pos = content_range_str.find('/').ok_or("Invalid Content-Range")?;
            let total_size_str = &content_range_str[slash_pos + 1..];
            if total_size_str != "*" {
                size = total_size_str.parse().unwrap_or(-1);
                if size == -1 {
                    return Err("Failed to parse total size".into());
                }
            }
            return Ok((offset, size));
        } else {
            return Err("Invalid Content-Range header".into());
        }
    }

    Ok((0, size))
}

/// Parses the content range string and returns the corresponding start, end,
/// and total values.
///
/// # Arguments
///
/// * `s` - The content range string to parse.
///
/// # Returns
///
/// Returns a `Result` containing a tuple `(i64, i64, i64)` representing the
/// start, end, and total values if successful, or a `Box<dyn std::error::Error
/// >` if an error occurred during parsing. (total is -1 when the value is "*")
///
/// # Examples
///
/// ```ignore
/// # use alibabacloud_oss_sdk_rust_v2::utils::parse_content_range;
/// #
/// let range = "bytes 0-499/1000";
/// let result = parse_content_range(range).unwrap();
/// assert_eq!(result, (0, 499, 1000));
/// ```
#[allow(unused)]
pub(crate) fn parse_content_range(s: &str) -> Result<(i64, i64, i64), Box<dyn std::error::Error>> {
    if !s.starts_with("bytes ") {
        return Err("invalid content range".into());
    }

    let slash_pos = s.find('/').ok_or("invalid content range")?;
    let dash_pos = s.find('-').ok_or("invalid content range")?;

    if slash_pos < dash_pos {
        return Err("invalid content range".into());
    }

    // from
    let from: i64 = s[6..dash_pos]
        .parse()
        .map_err(|_| "invalid content range")?;

    // to
    let to: i64 = s[dash_pos + 1..slash_pos]
        .parse()
        .map_err(|_| "invalid content range")?;

    // total
    let total = if &s[slash_pos + 1..] == "*" {
        -1
    } else {
        s[slash_pos + 1..]
            .parse()
            .map_err(|_| "invalid content range")?
    };

    Ok((from, to, total))
}

/// Parses the range header string and returns the corresponding `HTTPRange`
/// struct.
///
/// # Arguments
///
/// * `s` - The range header string to parse.
///
/// # Returns
///
/// Returns a `Result` containing the parsed `HTTPRange` if successful, or a
/// `Box<dyn std::error::Error >` if an error occurred during parsing.
///
/// # Examples
///
/// ```ignore
/// # use alibabacloud_oss_sdk_rust_v2::utils::parse_range;
/// #
/// let range = "bytes=0-499";
/// let result = parse_range(range).unwrap();
/// assert_eq!(result.offset, 0);
/// assert_eq!(result.count, 500);
/// ```
#[allow(unused)]
pub(crate) fn parse_range(s: &str) -> Result<HTTPRange, Box<dyn std::error::Error>> {
    const PREAMBLE: &str = "bytes=";

    if !s.starts_with(PREAMBLE) {
        return Err(format!("range: header invalid: doesn't start with {}", PREAMBLE).into());
    }
    let s = &s[PREAMBLE.len()..];
    if s.contains(',') {
        return Err("range: header invalid: contains multiple ranges which isn't supported".into());
    }
    let dash_pos = s
        .find('-')
        .ok_or("range: header invalid: contains no '-'")?;
    let (start, end) = s.split_at(dash_pos);
    let start = start.trim();
    let end = end[1..].trim(); // skip the '-'

    let mut offset = 0;
    let mut count = 0;

    if !start.is_empty() {
        offset = start.parse::<i64>()?;
        if offset < 0 {
            return Err("range: header invalid: bad start".into());
        }
    }

    if !end.is_empty() {
        let e = end.parse::<i64>()?;
        if e < 0 {
            return Err("range: header invalid: bad end".into());
        }
        count = e - offset + 1;
    }

    Ok(HTTPRange { offset, count })
}

#[cfg(test)]
mod tests {
    use http::HeaderValue;

    use super::*;

    #[test]
    fn test_parse_offset_and_size_without_content_range() {
        // no content range
        let mut headers = HeaderMap::new();
        headers.insert(
            HTTP_HEADER_CONTENT_LENGTH,
            HeaderValue::from_static("12345"),
        );
        let result = parse_offset_and_size_from_headers(&headers).unwrap();
        assert_eq!(result, (0, 12345));

        // has content range
        let mut headers = HeaderMap::new();
        headers.insert(
            HTTP_HEADER_CONTENT_LENGTH,
            HeaderValue::from_static("12345"),
        );
        headers.insert(
            HTTP_HEADER_CONTENT_RANGE,
            HeaderValue::from_static("bytes 12-34/56"),
        );
        let result = parse_offset_and_size_from_headers(&headers).unwrap();
        assert_eq!(result, (12, 56));

        // wildcard total-size
        let mut headers = HeaderMap::new();
        headers.insert(
            HTTP_HEADER_CONTENT_RANGE,
            HeaderValue::from_static("bytes 12-34/*"),
        );
        let result = parse_offset_and_size_from_headers(&headers).unwrap();
        assert_eq!(result, (12, -1));

        // invalid Content-Length
        let mut headers = HeaderMap::new();
        headers.insert(
            HTTP_HEADER_CONTENT_LENGTH,
            HeaderValue::from_static("invalid"),
        );
        let result = parse_offset_and_size_from_headers(&headers);
        assert!(result.is_err());

        // invalid Content-Range header
        let mut headers = HeaderMap::new();
        headers.insert(
            HTTP_HEADER_CONTENT_RANGE,
            HeaderValue::from_static("byte 12-34/56"),
        );
        let result = parse_offset_and_size_from_headers(&headers);
        assert!(result.is_err());

        // Content-Range no dash
        let mut headers = HeaderMap::new();
        headers.insert(
            HTTP_HEADER_CONTENT_RANGE,
            HeaderValue::from_static("bytes 12/56"),
        );
        let result = parse_offset_and_size_from_headers(&headers);
        assert!(result.is_err());

        // Content-Range no slash
        let mut headers = HeaderMap::new();
        headers.insert(
            HTTP_HEADER_CONTENT_RANGE,
            HeaderValue::from_static("bytes 12-34"),
        );
        let result = parse_offset_and_size_from_headers(&headers);
        assert!(result.is_err());

        // Content-Range invalid `from`
        let mut headers = HeaderMap::new();
        headers.insert(
            HTTP_HEADER_CONTENT_RANGE,
            HeaderValue::from_static("bytes invalid-34/56"),
        );
        let result = parse_offset_and_size_from_headers(&headers);
        assert!(result.is_err());

        // Content-Range invalid total-size
        let mut headers = HeaderMap::new();
        headers.insert(
            HTTP_HEADER_CONTENT_RANGE,
            HeaderValue::from_static("bytes 12-34/invalid"),
        );
        let result = parse_offset_and_size_from_headers(&headers);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_content_range() {
        // valid
        let result = parse_content_range("bytes 0-499/1000");
        assert_eq!(result.unwrap(), (0, 499, 1000));

        // wildcard
        let result = parse_content_range("bytes 0-499/*");
        assert_eq!(result.unwrap(), (0, 499, -1));

        // invalid header
        let result = parse_content_range("byte 0-499/1000");
        assert!(result.is_err());

        // invalid `from`
        let result = parse_content_range("bytes invalid-499/1000");
        assert!(result.is_err());

        // invalid `to`
        let result = parse_content_range("bytes 0-invalid/1000");
        assert!(result.is_err());

        // no `to`
        let result = parse_content_range("bytes 0/499-1000");
        assert!(result.is_err());

        // no total
        let result = parse_content_range("bytes 0-499/");
        assert!(result.is_err());

        // no dash
        let result = parse_content_range("bytes 499/1000");
        assert!(result.is_err());

        // no slash
        let result = parse_content_range("bytes 0-499");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_range() {
        // valid
        let result = parse_range("bytes=0-499").unwrap();
        assert_eq!(result.offset, 0);
        assert_eq!(result.count, 500);

        // empty start
        let result = parse_range("bytes=-500").unwrap();
        assert_eq!(result.offset, 0);
        assert_eq!(result.count, 501);

        // empty end
        let result = parse_range("bytes=100-").unwrap();
        assert_eq!(result.offset, 100);
        assert_eq!(result.count, 0);

        // invalid header
        let result = parse_range("invalid");
        assert!(result.is_err());

        // multiple ranges
        let result = parse_range("bytes=0-499,500-999");
        assert!(result.is_err());

        // invalid start
        let result = parse_range("bytes=invalid-100");
        assert!(result.is_err());

        // invalid end
        let result = parse_range("bytes=0-invalid");
        assert!(result.is_err());

        // minus start
        let result = parse_range("bytes=-1-100");
        assert!(result.is_err());

        // minus end
        let result = parse_range("bytes=100--1");
        assert!(result.is_err());
    }
}
