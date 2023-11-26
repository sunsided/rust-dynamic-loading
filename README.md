# Dynamic Loading: Plugins in Rust

Ensure both the application and the plugin are built:

```shell
cargo build -p app -p example-plugin
```

Then run the application from the repository root

```shell
cargo run
```

This should print output like the following:

```
[2023-11-26T16:30:06Z DEBUG app::plugin_manager] Loading plugin from "target/debug/deps/libexample_plugin.so"
[2023-11-26T16:30:06Z DEBUG app::plugin_manager] Initializing logger for loaded library
[2023-11-26T16:30:06Z DEBUG example_plugin] Logger initialized
[2023-11-26T16:30:06Z DEBUG app::plugin_manager] Loaded plugin: TestPlugin 0.1.0+integration-test
[2023-11-26T16:30:06Z INFO  example_plugin] Plugin loaded
[2023-11-26T16:30:06Z INFO  app] Data before applying plugins: 21
[2023-11-26T16:30:06Z DEBUG app::plugin_manager] Firing pre_operation hooks
[2023-11-26T16:30:06Z TRACE app::plugin_manager] Firing pre_operation for TestPlugin 0.1.0+integration-test
[2023-11-26T16:30:06Z INFO  example_plugin] Added 1 to input
[2023-11-26T16:30:06Z DEBUG app::plugin_manager] Firing operation hooks
[2023-11-26T16:30:06Z TRACE app::plugin_manager] Firing operation for TestPlugin 0.1.0+integration-test
[2023-11-26T16:30:06Z INFO  example_plugin] Multiplied input by 2
[2023-11-26T16:30:06Z DEBUG app::plugin_manager] Firing post_operation hooks
[2023-11-26T16:30:06Z TRACE app::plugin_manager] Firing post_operation for TestPlugin 0.1.0+integration-test
[2023-11-26T16:30:06Z INFO  example_plugin] Subtracted 2 from input
[2023-11-26T16:30:06Z INFO  app] Data after applying plugins: 42
[2023-11-26T16:30:06Z DEBUG app::plugin_manager] Unloading plugins
[2023-11-26T16:30:06Z TRACE app::plugin_manager] Firing on_plugin_unload for TestPlugin 0.1.0+integration-test
[2023-11-26T16:30:06Z INFO  example_plugin] Plugin unloaded
[2023-11-26T16:30:06Z TRACE app::plugin_manager] Unloading logger
```

This assumes the environment provided in the [.env](.env) file:

```.dotenv
RUST_LOG=app=trace,example_plugin=debug
```

## Logging in the Plugins

In order to achieve logging in the dynamically loaded libraries, a `declare_logger_plugin!`
macro is provided. It exports the `_logger_create` used to initialize the logger.
If this symbol is found, an instance to the logger and the lowest log level is passed,
which is then set up as the global logger within the shared library.

For this to work, a `&'static dyn Log` reference needs to be available, which is a bit
of a hackery in `main`: A `StaticLogger` type takes ownership of the `Box<env_logger::Logger>`
and _pretends_ to be able to provide a `&'static dyn Log` instance. This assumes that this logger
is always created first and destroyed last.

## Further reading

This toy project is inspired by Michael F. Bryan's [Dynamic Loading & Plugins] post.

[Dynamic Loading & Plugins]: https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
