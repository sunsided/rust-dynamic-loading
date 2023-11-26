/// Registers a logger plugin for this crate.
///
/// # Usage
///
/// This macro should be called without any arguments to generate the necessary code for registering a logger plugin. This will create an `extern "C"` function
#[macro_export]
macro_rules! declare_logger_plugin {
    () => {
        #[no_mangle]
        pub extern "C" fn _logger_create(
            log: &'static dyn log::Log,
            filter: log::LevelFilter,
        ) -> *mut dyn plugin_traits::PluginLogger {
            // make sure the constructor is the correct type.
            let constructor: fn(
                &'static dyn log::Log,
                filter: log::LevelFilter,
            ) -> PluginLoggerImpl = |log, filter| PluginLoggerImpl::new(log, filter);

            let object = constructor(log, filter);
            let boxed: Box<dyn plugin_traits::PluginLogger> = Box::new(object);
            Box::into_raw(boxed)
        }

        struct PluginLoggerImpl {}

        impl PluginLoggerImpl {
            pub fn new(log: &'static dyn log::Log, filter: log::LevelFilter) -> Self {
                if log::set_logger(log).is_ok() {
                    log::set_max_level(filter);
                    log::debug!("Logger initialized");
                } else {
                    eprintln!("Failed to set logger in plugin");
                }
                Self {}
            }

            pub fn unset(&self) {
                log::set_boxed_logger(Box::new(crate::NullLogger)).ok();
                log::set_max_level(log::Level::Error.to_level_filter());
            }
        }

        impl plugin_traits::PluginLogger for PluginLoggerImpl {
            fn unset(&self) {
                self.unset();
            }
        }

        impl Drop for PluginLoggerImpl {
            fn drop(&mut self) {
                self.unset();
            }
        }

        struct NullLogger;

        impl log::Log for NullLogger {
            fn enabled(&self, _metadata: &log::Metadata) -> bool {
                false
            }

            fn log(&self, _record: &log::Record) {
                // This is intentionally left blank.
            }

            fn flush(&self) {}
        }
    };
}

pub trait PluginLogger {
    fn unset(&self);
}
