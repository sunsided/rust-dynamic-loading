mod logger;

pub use logger::PluginLogger;

use std::any::Any;

/// Declare a plugin type and its constructor.
///
/// # Notes
///
/// This works by automatically generating an `extern "C"` function with a
/// pre-defined signature and symbol name. Therefore you will only be able to
/// declare one plugin per library.
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn $crate::ExamplePlugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<dyn $crate::ExamplePlugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}

pub trait Plugin: Any + Send + Sync {
    fn name(&self) -> &'static str;
    fn semantic_version(&self) -> &'static str;
    fn on_plugin_load(&self) {}
    fn on_plugin_unload(&self) {}
}

#[allow(unused_variables)]
pub trait ExamplePlugin: Plugin {
    fn pre_operation(&self, data: &mut u32) {}
    fn operation(&self, data: &mut u32);
    fn post_operation(&self, data: &mut u32) {}
}
