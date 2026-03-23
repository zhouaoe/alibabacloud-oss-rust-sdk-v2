/// Represents the log levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Default)]
pub enum LogLevel {
    /// No logging.
    Off,
    /// Error level logging.
    Error,
    /// Warning level logging.
    Warn,
    /// Information level logging.
    #[default]
    Info,
    /// Debug level logging.
    Debug,
}

impl From<&str> for LogLevel {
    /// Converts a string to a `LogLevel`.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to convert.
    ///
    /// # Examples
    ///
    /// ```
    /// # use alibabacloud_oss_sdk_rust_v2::log::LogLevel;
    /// #
    /// let level: LogLevel = "error".into();
    /// assert_eq!(level, LogLevel::Error);
    /// ```
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "error" | "err" => LogLevel::Error,
            "warning" | "warn" => LogLevel::Warn,
            "info" | "inf" => LogLevel::Info,
            "debug" | "dbg" => LogLevel::Debug,
            _ => LogLevel::Off,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_from_str() {
        assert_eq!(LogLevel::from("error"), LogLevel::Error);
        assert_eq!(LogLevel::from("warn"), LogLevel::Warn);
        assert_eq!(LogLevel::from("info"), LogLevel::Info);
        assert_eq!(LogLevel::from("debug"), LogLevel::Debug);
        assert_eq!(LogLevel::from("invalid"), LogLevel::Off);
    }
}
