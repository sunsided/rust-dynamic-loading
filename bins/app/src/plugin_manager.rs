use env_logger::Logger;
use libloading::{Library, Symbol};
use log::{debug, trace};
use plugin_traits::ExamplePlugin;
use std::ffi::OsStr;

pub struct PluginManager {
    logger: &'static Logger,
    plugins: Vec<(
        Box<dyn ExamplePlugin>,
        Option<Box<dyn plugin_traits::PluginLogger>>,
    )>,
    loaded_libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new(logger: &'static Logger) -> PluginManager {
        PluginManager {
            logger,
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }

    pub unsafe fn load_plugin<P: AsRef<OsStr>>(
        &mut self,
        filename: P,
    ) -> Result<(), LoadPluginError> {
        type LoggerCreate = unsafe fn(
            log: &'static dyn log::Log,
            filter: log::LevelFilter,
        ) -> *mut dyn plugin_traits::PluginLogger;
        type PluginCreate = unsafe fn() -> *mut dyn ExamplePlugin;

        debug!("Loading plugin from {:?}", filename.as_ref());
        let lib = Library::new(filename.as_ref()).map_err(LoadPluginError::FailedToLoad)?;

        // We need to keep the library around otherwise our plugin's vtable will
        // point to garbage. We do this little dance to make sure the library
        // doesn't end up getting moved.
        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();

        // Attempt to load the logger.
        let logger = match lib.get::<Symbol<LoggerCreate>>(b"_logger_create") {
            Ok(constructor) => {
                debug!("Initializing logger for loaded library");
                let logger = constructor(self.logger, self.logger.filter());
                let logger = Box::from_raw(logger);
                Some(logger)
            }
            Err(_) => None,
        };

        // Load the plugin.
        let constructor: Symbol<PluginCreate> = lib
            .get(b"_plugin_create")
            .map_err(LoadPluginError::PluginCreateNotFound)?;
        let boxed_raw = constructor();

        let plugin = Box::from_raw(boxed_raw);
        debug!(
            "Loaded plugin: {} {}",
            plugin.name(),
            plugin.semantic_version()
        );
        plugin.on_plugin_load();
        self.plugins.push((plugin, logger));

        Ok(())
    }

    /// Iterate over the plugins, running their `pre_send()` hook.
    pub fn pre_operation(&mut self, data: &mut u32) {
        debug!("Firing pre_operation hooks");

        for (plugin, _) in &mut self.plugins {
            trace!(
                "Firing pre_operation for {} {}",
                plugin.name(),
                plugin.semantic_version()
            );
            plugin.pre_operation(data);
        }
    }

    /// Iterate over the plugins, running their `operation()` hook.
    pub fn operation(&mut self, data: &mut u32) {
        debug!("Firing operation hooks");

        for (plugin, _) in &mut self.plugins {
            trace!(
                "Firing operation for {} {}",
                plugin.name(),
                plugin.semantic_version()
            );
            plugin.operation(data);
        }
    }

    /// Iterate over the plugins, running their `post_operation()` hook.
    pub fn post_operation(&mut self, data: &mut u32) {
        debug!("Firing post_operation hooks");

        for (plugin, _) in &mut self.plugins {
            trace!(
                "Firing post_operation for {} {}",
                plugin.name(),
                plugin.semantic_version()
            );
            plugin.post_operation(data);
        }
    }

    /// Unload all plugins and loaded plugin libraries, making sure to fire
    /// their `on_plugin_unload()` methods so they can do any necessary cleanup.
    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for (plugin, logger) in self.plugins.drain(..) {
            trace!(
                "Firing on_plugin_unload for {} {}",
                plugin.name(),
                plugin.semantic_version()
            );
            plugin.on_plugin_unload();

            if let Some(logger) = logger {
                trace!("Unloading logger");
                logger.unset();
            }
        }

        for lib in self.loaded_libraries.drain(..) {
            drop(lib);
        }
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
            self.unload();
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoadPluginError {
    #[error("Unable to load the plugin: {0}")]
    FailedToLoad(libloading::Error),

    #[error("The `_plugin_create` symbol wasn't found.")]
    PluginCreateNotFound(libloading::Error),
}
