use plugin_traits::{ExamplePlugin, Plugin};

struct TestPlugin {}

impl Plugin for TestPlugin {
    fn name(&self) -> &'static str {
        "TestPlugin"
    }

    fn semantic_version(&self) -> &'static str {
        "0.1.0+unit-test"
    }
}

impl ExamplePlugin for TestPlugin {
    fn pre_operation(&self, data: &mut u32) {
        *data += 1;
    }

    fn operation(&self, data: &mut u32) {
        *data *= 2;
    }

    fn post_operation(&self, data: &mut u32) {
        *data -= 1;
    }
}
