use env_logger::Logger;
use std::ops::Deref;

/// Initializes the logger.
///
/// Construct a new logger using the default environment variables and build it.
/// Convert the logger into a `StaticLogger` using the `from` method of the `StaticLogger`.
/// Set the maximum log level of the logger.
/// Set the global logger using the `set_logger` method of the `log` crate.
/// If the logger is successfully set, set the maximum log level using the `set_max_level` method of the `log` crate.
///
/// # Return Value
///
/// Returns a `StaticLogger` instance.
///
/// # Safety
///
/// This function uses several `unsafe` operations under the hood through
/// the `StaticLogger` methods, including dereferencing raw pointers and
/// creating static references. These operations are potentially dangerous
/// and have several safety requirements:
///
/// - The raw pointer in `StaticLogger` must be valid at all the times. It should
///   always point to a live `Logger` object. Any operation that may potentially
///   invalidate the pointer (like moving the `Logger` it points to) should be
///   strictly avoided.
///
/// - The static reference created by `as_static_ref` method is assumed to be
///   valid for the 'static lifetime. This implies that the `Logger` object must
///   not be moved or deallocated while references to it exist. Ensure to maintain
///   the invariant that the `Logger` outlives all its references.
///
/// Improper use of this function can lead to undefined behavior, including dangling
/// pointers and data races. Use with care and ensure that the 'static logger is
/// properly managed.
pub fn initialize_logger() -> StaticLogger {
    let logger = env_logger::Builder::new().parse_default_env().build();
    let logger = StaticLogger::from(logger);

    let max_level = logger.filter();
    let r = log::set_logger(logger.as_static_ref());
    if r.is_ok() {
        log::set_max_level(max_level);
    }
    logger
}

/// The `StaticLogger` struct is a wrapper around a raw pointer to a `Logger` instance.
/// It provides a way to access the logger instance in a static context.
///
/// # Safety
/// The use of a raw pointer requires caution as it bypasses Rust's ownership and borrowing rules.
/// The caller must ensure the safety and correctness of using the logger instance.
pub struct StaticLogger(*mut Logger);

impl StaticLogger {
    pub fn as_static_ref(&self) -> &'static Logger {
        unsafe { &*self.0 }
    }
}

impl From<Logger> for StaticLogger {
    fn from(value: Logger) -> Self {
        Box::new(value).into()
    }
}

impl From<Box<Logger>> for StaticLogger {
    fn from(value: Box<Logger>) -> Self {
        Self(Box::into_raw(value))
    }
}

impl AsRef<Logger> for StaticLogger {
    fn as_ref(&self) -> &Logger {
        unsafe { &*self.0 }
    }
}

impl Deref for StaticLogger {
    type Target = Logger;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Drop for StaticLogger {
    fn drop(&mut self) {
        let _box = unsafe { Box::from_raw(self.0) };
    }
}
