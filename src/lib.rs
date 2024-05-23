#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

/// Contains some common loggers.
pub mod common;

#[derive(Clone, Debug, PartialEq, Copy, PartialOrd, Eq, Ord)]
pub enum ULogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl From<&ULogLevel> for &'static str {
    fn from(value: &ULogLevel) -> Self {
        match value {
            ULogLevel::Debug => "DEBUG",
            ULogLevel::Info => "INFO",
            ULogLevel::Warning => "WARN",
            ULogLevel::Error => "ERROR",
            ULogLevel::Critical => "CRITICAL",
        }
    }
}

impl core::fmt::Display for ULogLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<ULogLevel> for &'static str {
    fn from(value: ULogLevel) -> Self {
        <&'static str as From<&ULogLevel>>::from(&value)
    }
}

impl ULogLevel {
    /// Converts the level name to an uppercase string.
    pub fn as_str(&self) -> &'static str {
        self.into()
    }

    /// Converts the level name to a short, 3-character uppercase string.
    pub fn as_short_str(&self) -> &'static str {
        match self {
            ULogLevel::Debug => "DBG",
            ULogLevel::Info => "INF",
            ULogLevel::Warning => "WRN",
            ULogLevel::Error => "ERR",
            ULogLevel::Critical => "CRT",
        }
    }

    /// A list of all possible log levels, in ascending order; useful for testing.
    pub fn all_levels() -> [ULogLevel; 5] {
        [
            ULogLevel::Debug,
            ULogLevel::Info,
            ULogLevel::Warning,
            ULogLevel::Error,
            ULogLevel::Critical,
        ]
    }
}

/// Contains data to be used when logging.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct ULogData {
    pub level: ULogLevel,
    pub line: u32,
    pub file: &'static str,
}

impl ULogData {
    pub fn new(level: ULogLevel, line: u32, file: &'static str) -> Self {
        Self { level, line, file }
    }
}

/// A trait that all loggers should implement; [`log_str`](ULog::log_str) and [`log_format`](ULog::log_format)
/// will be called by the different macros
/// to respectively log a static string or a value implementing [`Debug`](core::fmt::Debug).
///
/// [`log_begin`](ULog::log_begin) will be called at the beginning of any logging statement, and can be used to print a header.
/// [`log_end`](ULog::log_end) will be called once the logging statement is done, and can be interpreted to send a newline
/// and/or flush the stream.
///
/// Implementors are free to ignore the value passed to `log_format` (for instance if the formatting utilities are too costly to include).
pub trait ULog {
    /// Logs a string of characters, alongside the given `level`.
    fn log_str(&self, log_data: &ULogData, string: &str);

    /// Optionally logs a key-value pair, where the value implements [`Debug`](core::fmt::Debug).
    fn log_format<T: core::fmt::Debug>(&self, log_data: &ULogData, key: &str, value: &T);

    /// Begins a logging statement, called once before a chain of `log_str` and `log_format`.
    fn log_begin(&self, log_data: &ULogData);

    /// Ends a logging statement, called once after a chain of `log_str` and `log_format`.
    fn log_end(&self, log_data: &ULogData);

    /// A shortcut for [`ChainLogger::new(self, other)`](common::ChainLogger::new);
    /// constructs a logger that forwards statements to both `self` and `other`.
    fn chain<Other: ULog>(self, other: Other) -> common::ChainLogger<Self, Other>
    where
        Self: Sized,
    {
        common::ChainLogger::new(self, other)
    }

    /// A shortcut for [`MinLevelLogger::new(self, min_level)`](common::MinLogger::new);
    /// wraps the logger so that it only interprets logging statements with a level above `min_level`.
    fn min_level(self, min_level: ULogLevel) -> common::MinLevelLogger<Self>
    where
        Self: Sized,
    {
        common::MinLevelLogger::new(self, min_level)
    }
}

impl<Logger: ULog> ULog for &Logger {
    #[inline(always)]
    fn log_str(&self, log_data: &ULogData, string: &str) {
        <Logger as ULog>::log_str(*self, log_data, string)
    }

    #[inline(always)]
    fn log_format<T: core::fmt::Debug>(&self, log_data: &ULogData, key: &str, value: &T) {
        <Logger as ULog>::log_format(*self, log_data, key, value)
    }

    #[inline(always)]
    fn log_begin(&self, log_data: &ULogData) {
        <Logger as ULog>::log_begin(*self, log_data)
    }

    #[inline(always)]
    fn log_end(&self, log_data: &ULogData) {
        <Logger as ULog>::log_end(*self, log_data)
    }
}

#[macro_export]
macro_rules! ulog {
    ( $level:expr, $logger:expr, $str:expr $(,)? ) => {{
        let log_data = $crate::ULogData::new($level, line!(), file!());

        $crate::ULog::log_begin(&$logger, &log_data);
        $crate::ULog::log_str(&$logger, &log_data, $str);
        $crate::ULog::log_end(&$logger, &log_data);
    }};

    ( $level:expr, $logger:expr, $str:expr, $($name:tt => $value:expr),+ $(,)? ) => {{
        let log_data = $crate::ULogData::new($level, line!(), file!());

        $crate::ULog::log_begin(&$logger, &log_data);
        $crate::ULog::log_str(&$logger, &log_data, $str);
        $(
            $crate::ULog::log_format(&$logger, &log_data, $name, &$value);
        )+
        $crate::ULog::log_end(&$logger, &log_data);
    }}
}

#[macro_export]
macro_rules! debug {
    ( $logger:expr, $str:expr $(, $($name:tt => $value:expr),* $(,)? )? ) => {
        $crate::ulog!($crate::ULogLevel::Debug, $logger, $str, $( $( $name => $value ),* )?)
    }
}

#[macro_export]
macro_rules! info {
    ( $logger:expr, $str:expr $(, $($name:tt => $value:expr),* $(,)? )? ) => {
        $crate::ulog!($crate::ULogLevel::Info, $logger, $str, $( $( $name => $value ),* )?)
    }
}

#[macro_export]
macro_rules! warn {
    ( $logger:expr, $str:expr $(, $($name:tt => $value:expr),* $(,)? )? ) => {
        $crate::ulog!($crate::ULogLevel::Warning, $logger, $str, $( $( $name => $value ),* )?)
    }
}

#[macro_export]
macro_rules! error {
    ( $logger:expr, $str:expr $(, $($name:tt => $value:expr),* $(,)? )? ) => {
        $crate::ulog!($crate::ULogLevel::Error, $logger, $str, $( $( $name => $value ),* )?)
    }
}

#[macro_export]
macro_rules! critical {
    ( $logger:expr, $str:expr $(, $($name:tt => $value:expr),* $(,)? )? ) => {
        $crate::ulog!($crate::ULogLevel::Critical, $logger, $str, $( $( $name => $value ),* )?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn test_level_order() {
        let levels = ULogLevel::all_levels();

        for i in 0..(levels.len() - 1) {
            for j in (i + 1)..levels.len() {
                assert!(levels[i] < levels[j]);
            }
        }
    }

    #[derive(Default)]
    struct TestLogger {
        logs: RefCell<Vec<(ULogLevel, String)>>,
    }

    impl ULog for TestLogger {
        fn log_str(&self, log_data: &ULogData, data: &str) {
            self.logs
                .borrow_mut()
                .push((log_data.level, data.to_string()));
        }

        fn log_format<T: core::fmt::Debug>(&self, log_data: &ULogData, name: &str, value: &T) {
            self.logs
                .borrow_mut()
                .push((log_data.level, format!("{name} => {:?}", value)));
        }

        fn log_begin(&self, log_data: &ULogData) {
            self.logs
                .borrow_mut()
                .push((log_data.level, String::from("__BEGIN__")));
        }

        fn log_end(&self, log_data: &ULogData) {
            self.logs
                .borrow_mut()
                .push((log_data.level, String::from("__END__")));
        }
    }

    #[test]
    fn test_ulog_macro() {
        let logger = TestLogger::default();

        ulog!(ULogLevel::Debug, logger, "Hello");
        ulog!(ULogLevel::Error, logger, "world", "value" => 32);

        assert_eq!(
            &logger.logs.into_inner()[..],
            &[
                (ULogLevel::Debug, String::from("__BEGIN__")),
                (ULogLevel::Debug, String::from("Hello")),
                (ULogLevel::Debug, String::from("__END__")),
                (ULogLevel::Error, String::from("__BEGIN__")),
                (ULogLevel::Error, String::from("world")),
                (ULogLevel::Error, String::from("value => 32")),
                (ULogLevel::Error, String::from("__END__")),
            ]
        );
    }

    #[test]
    fn test_info_macro() {
        let logger = TestLogger::default();

        info!(logger, "Hello");
        info!(logger, "world", "value" => 32);

        assert_eq!(
            &logger.logs.into_inner()[..],
            &[
                (ULogLevel::Info, String::from("__BEGIN__")),
                (ULogLevel::Info, String::from("Hello")),
                (ULogLevel::Info, String::from("__END__")),
                (ULogLevel::Info, String::from("__BEGIN__")),
                (ULogLevel::Info, String::from("world")),
                (ULogLevel::Info, String::from("value => 32")),
                (ULogLevel::Info, String::from("__END__")),
            ]
        );
    }

    #[test]
    fn test_min_level() {
        let logger = TestLogger::default().min_level(ULogLevel::Warning);

        for level in ULogLevel::all_levels() {
            ulog!(level, logger, "Hello");
        }

        assert!(logger
            .into_inner()
            .logs
            .into_inner()
            .iter()
            .all(|log| log.0 >= ULogLevel::Warning));
    }
}
