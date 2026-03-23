mod level;
mod logger;
mod output;
mod printer;

pub use self::level::*;
pub use self::logger::*;
pub use self::output::*;
pub use self::printer::*;

/// The `Logger` trait defines the behavior of a logger.
pub trait Logger {
    /// Logs a debug message with the specified format.
    fn debug(&self, format: &str);

    /// Logs an info message with the specified format.
    fn info(&self, format: &str);

    /// Logs a warning message with the specified format.
    fn warn(&self, format: &str);

    /// Logs an error message with the specified format.
    fn error(&self, format: &str);

    /// Returns the log level of the logger.
    fn level(&self) -> LogLevel;
}
