mod plugin_manager;
mod static_logger;

use crate::plugin_manager::PluginManager;
use crate::static_logger::initialize_logger;
use env_logger::Logger;
use log::{error, info};
use std::ffi::OsStr;

/// Executed the main function of the application.
///
/// # Arguments
///
/// * `logger` - A reference to the logger object used for logging messages.
/// ```
fn run_main(logger: &'static Logger) {
    let mut manager = PluginManager::new(logger);

    let plugin_path: &OsStr = OsStr::new("target/debug/deps/libexample_plugin.so");
    match unsafe { manager.load_plugin(plugin_path) } {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to load plugin from {:?}: {}", plugin_path, e);
            return;
        }
    }

    let mut data = 21;
    info!("Data before applying plugins: {}", data);

    manager.pre_operation(&mut data);
    manager.operation(&mut data);
    manager.post_operation(&mut data);

    info!("Data after applying plugins: {}", data);
}

/// This function is the entry point of the application.
///
/// It initializes the logger using the `env_logger` crate, sets the maximum log level based
/// on the logger configuration, and calls the `run_main` function.
fn main() {
    dotenvy::dotenv().ok();
    let logger = initialize_logger();
    run_main(logger.as_static_ref());
}
