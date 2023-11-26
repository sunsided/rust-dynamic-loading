mod plugin_manager;

use crate::plugin_manager::PluginManager;
use env_logger::Builder;
use log::{error, info};
use std::ffi::OsStr;

fn main() {
    dotenvy::dotenv().ok();

    Builder::new().parse_default_env().init();

    let mut manager = PluginManager::new();

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
