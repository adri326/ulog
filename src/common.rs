use super::{ULog, ULogData, ULogLevel};

/// A logger that does not log anything, useful for conditionally turning off logging.
pub struct StubLogger;

impl ULog for StubLogger {
    #[inline(always)]
    fn log_str(&self, _log_data: &ULogData, _string: &str) {
        // Noop
    }

    #[inline(always)]
    fn log_format<T: core::fmt::Debug>(&self, _log_data: &ULogData, _key: &str, _value: &T) {
        // Noop
    }

    #[inline(always)]
    fn log_begin(&self, _log_data: &ULogData) {
        // Noop
    }

    #[inline(always)]
    fn log_end(&self, _log_data: &ULogData) {
        // Noop
    }
}

/// Chains or composes two or more loggers together, forwarding any logging statements to all of them.
/// Can be quickly constructed by calling [`ULog::chain`].
#[derive(Debug, Clone)]
pub struct ChainLogger<Parent, Current> {
    parent: Parent,
    current: Current,
}

impl<Parent: ULog, Current: ULog> ChainLogger<Parent, Current> {
    pub fn new(parent: Parent, current: Current) -> Self {
        Self { parent, current }
    }

    pub fn into_inner(self) -> (Parent, Current) {
        (self.parent, self.current)
    }
}

impl<Parent: ULog, Current: ULog> ULog for ChainLogger<Parent, Current> {
    fn log_str(&self, log_data: &ULogData, string: &str) {
        self.parent.log_str(log_data, string);
        self.current.log_str(log_data, string);
    }

    fn log_format<T: core::fmt::Debug>(&self, log_data: &ULogData, key: &str, value: &T) {
        self.parent.log_format(log_data, key, value);
        self.current.log_format(log_data, key, value);
    }

    fn log_begin(&self, log_data: &ULogData) {
        self.parent.log_begin(log_data);
        self.current.log_begin(log_data);
    }

    fn log_end(&self, log_data: &ULogData) {
        self.parent.log_end(log_data);
        self.current.log_end(log_data);
    }
}

/// Restricts the logs going to the wrapped logger to be above a minimum level threshold.
/// Can be quickly constructed using [`ULog::min_level`]
#[derive(Debug, Clone)]
pub struct MinLevelLogger<Logger> {
    logger: Logger,
    min_level: ULogLevel,
}

impl<Logger: ULog> MinLevelLogger<Logger> {
    pub fn new(logger: Logger, min_level: ULogLevel) -> Self {
        Self { logger, min_level }
    }

    pub fn min_level(&self) -> ULogLevel {
        self.min_level
    }

    pub fn into_inner(self) -> Logger {
        self.logger
    }
}

impl<Logger: ULog> ULog for MinLevelLogger<Logger> {
    fn log_str(&self, log_data: &ULogData, string: &str) {
        if log_data.level >= self.min_level {
            self.logger.log_str(log_data, string);
        }
    }

    fn log_format<T: core::fmt::Debug>(&self, log_data: &ULogData, key: &str, value: &T) {
        if log_data.level >= self.min_level {
            self.logger.log_format(log_data, key, value);
        }
    }

    fn log_begin(&self, log_data: &ULogData) {
        if log_data.level >= self.min_level {
            self.logger.log_begin(log_data);
        }
    }

    fn log_end(&self, log_data: &ULogData) {
        if log_data.level >= self.min_level {
            self.logger.log_end(log_data);
        }
    }
}
