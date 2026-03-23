use lazy_static::lazy_static;
use regex::Regex;

/// Represents the type of endpoint for accessing OSS.
#[derive(Debug, PartialEq, Eq)]
pub enum EndpointType {
    /// Access OSS over the public network, oss-\[region\].aliyuncs.com
    Public,
    /// Access OSS over the internal network,
    /// oss-\[region\]-internal.aliyuncs.com
    Internal,
    /// Access OSS over the global acceleration endpoint,
    /// oss-accelerate.aliyuncs.com
    Accelerate,
    /// Access OSS over the acceleration endpoint outside the Chinese mainland,
    /// oss-accelerate-overseas.aliyuncs.com
    #[allow(unused)]
    AccelerateOverseas,
    /// Access OSS over the dual stack endpoint that support both IPv4 and IPv6,
    /// \[region\].oss.aliyuncs.com
    DualStack,
}

lazy_static! {
    static ref SCHEME_PATTERN: Regex = Regex::new(r"^([^:]+)://").expect("Invalid regex pattern");
}

/// Adds the scheme (http or https) to the endpoint if it doesn't already have
/// one.
///
/// # Arguments
///
/// * `endpoint` - The endpoint URL.
/// * `disable_ssl` - A flag indicating whether SSL is disabled.
///
/// # Returns
///
/// The endpoint URL with the scheme added.
pub(crate) fn add_endpoint_scheme(endpoint: &str, disable_ssl: bool) -> String {
    if !endpoint.is_empty() && !SCHEME_PATTERN.is_match(endpoint) {
        let scheme = if disable_ssl { "http" } else { "https" };
        format!("{}://{}", scheme, endpoint)
    } else {
        endpoint.to_string()
    }
}

/// Generates the endpoint URL based on the region, SSL settings, and endpoint
/// type.
///
/// # Arguments
///
/// * `region` - The region where the OSS is located.
/// * `disable_ssl` - A flag indicating whether SSL is disabled.
/// * `endpoint_type` - The type of endpoint to generate.
///
/// # Returns
///
/// The generated endpoint URL.
pub(crate) fn endpoint_from_region(
    region: &str,
    disable_ssl: bool,
    endpoint_type: EndpointType,
) -> String {
    let scheme = if disable_ssl { "http" } else { "https" };
    let endpoint = match endpoint_type {
        EndpointType::Internal => format!("oss-{}-internal.aliyuncs.com", region),
        EndpointType::DualStack => format!("{}.oss.aliyuncs.com", region),
        EndpointType::Accelerate => "oss-accelerate.aliyuncs.com".to_string(),
        EndpointType::AccelerateOverseas => "oss-accelerate-overseas.aliyuncs.com".to_string(),
        _ => format!("oss-{}.aliyuncs.com", region),
    };
    format!("{}://{}", scheme, endpoint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_endpoint_scheme() {
        assert_eq!("", add_endpoint_scheme("", true));
        assert_eq!("", add_endpoint_scheme("", false));
        assert_eq!("https://123", add_endpoint_scheme("123", false));
        assert_eq!("http://123", add_endpoint_scheme("123", true));
        assert_eq!("http://123", add_endpoint_scheme("http://123", false));
        assert_eq!("ftp://123", add_endpoint_scheme("ftp://123", false));
    }

    #[test]
    fn test_endpoint_from_region() {
        // EndpointPublic
        assert_eq!(
            "https://oss-.aliyuncs.com",
            endpoint_from_region("", false, EndpointType::Public)
        );
        assert_eq!(
            "http://oss-.aliyuncs.com",
            endpoint_from_region("", true, EndpointType::Public)
        );
        assert_eq!(
            "https://oss-cn-hangzhou.aliyuncs.com",
            endpoint_from_region("cn-hangzhou", false, EndpointType::Public)
        );
        assert_eq!(
            "http://oss-cn-hangzhou.aliyuncs.com",
            endpoint_from_region("cn-hangzhou", true, EndpointType::Public)
        );

        // EndpointInternal
        assert_eq!(
            "https://oss--internal.aliyuncs.com",
            endpoint_from_region("", false, EndpointType::Internal)
        );
        assert_eq!(
            "http://oss--internal.aliyuncs.com",
            endpoint_from_region("", true, EndpointType::Internal)
        );
        assert_eq!(
            "https://oss-cn-hangzhou-internal.aliyuncs.com",
            endpoint_from_region("cn-hangzhou", false, EndpointType::Internal)
        );
        assert_eq!(
            "http://oss-cn-hangzhou-internal.aliyuncs.com",
            endpoint_from_region("cn-hangzhou", true, EndpointType::Internal)
        );

        // EndpointAccelerate
        assert_eq!(
            "https://oss-accelerate.aliyuncs.com",
            endpoint_from_region("", false, EndpointType::Accelerate)
        );
        assert_eq!(
            "http://oss-accelerate.aliyuncs.com",
            endpoint_from_region("", true, EndpointType::Accelerate)
        );
        assert_eq!(
            "https://oss-accelerate.aliyuncs.com",
            endpoint_from_region("cn-hangzhou", false, EndpointType::Accelerate)
        );
        assert_eq!(
            "http://oss-accelerate.aliyuncs.com",
            endpoint_from_region("cn-hangzhou", true, EndpointType::Accelerate)
        );

        // EndpointAccelerateOverseas
        assert_eq!(
            "https://oss-accelerate-overseas.aliyuncs.com",
            endpoint_from_region("", false, EndpointType::AccelerateOverseas)
        );
        assert_eq!(
            "http://oss-accelerate-overseas.aliyuncs.com",
            endpoint_from_region("", true, EndpointType::AccelerateOverseas)
        );
        assert_eq!(
            "https://oss-accelerate-overseas.aliyuncs.com",
            endpoint_from_region("cn-hangzhou", false, EndpointType::AccelerateOverseas)
        );
        assert_eq!(
            "http://oss-accelerate-overseas.aliyuncs.com",
            endpoint_from_region("cn-hangzhou", true, EndpointType::AccelerateOverseas)
        );

        // EndpointDualStack
        assert_eq!(
            "https://.oss.aliyuncs.com",
            endpoint_from_region("", false, EndpointType::DualStack)
        );
        assert_eq!(
            "http://.oss.aliyuncs.com",
            endpoint_from_region("", true, EndpointType::DualStack)
        );
        assert_eq!(
            "https://cn-hangzhou.oss.aliyuncs.com",
            endpoint_from_region("cn-hangzhou", false, EndpointType::DualStack)
        );
        assert_eq!(
            "http://cn-hangzhou.oss.aliyuncs.com",
            endpoint_from_region("cn-hangzhou", true, EndpointType::DualStack)
        );
    }
}
