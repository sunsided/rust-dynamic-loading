use log::info;
use plugin_traits::{declare_logger_plugin, declare_plugin, ExamplePlugin, Plugin};

// Export the logging wrapper.
declare_logger_plugin!();

// Export the actual plugin.
declare_plugin!(MyExamplePlugin, MyExamplePlugin::default);

#[derive(Debug, Default)]
struct MyExamplePlugin {}

impl Plugin for MyExamplePlugin {
    fn name(&self) -> &'static str {
        "TestPlugin"
    }

    fn semantic_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn on_plugin_load(&self) {
        info!("Plugin loaded");
    }

    fn on_plugin_unload(&self) {
        info!("Plugin unloaded");
    }
}

impl ExamplePlugin for MyExamplePlugin {
    fn pre_operation(&self, data: &mut u32) {
        info!("Added 1 to input");
        *data += 1;
    }

    fn operation(&self, data: &mut u32) {
        info!("Multiplied input by 2");
        *data *= 2;
    }

    fn post_operation(&self, data: &mut u32) {
        info!("Subtracted 2 from input");
        *data -= 2;
    }
}
