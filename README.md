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
[2023-11-26T13:47:12Z DEBUG app::plugin_manager] Loading plugin from "target/debug/deps/libexample_plugin.so"
[2023-11-26T13:47:12Z DEBUG app::plugin_manager] Loaded plugin: TestPlugin 0.1.0+unit-test
[2023-11-26T13:47:12Z INFO  app] Data before applying plugins: 21
[2023-11-26T13:47:12Z DEBUG app::plugin_manager] Firing pre_operation hooks
[2023-11-26T13:47:12Z TRACE app::plugin_manager] Firing pre_operation for TestPlugin 0.1.0+unit-test
[2023-11-26T13:47:12Z DEBUG app::plugin_manager] Firing operation hooks
[2023-11-26T13:47:12Z TRACE app::plugin_manager] Firing operation for TestPlugin 0.1.0+unit-test
[2023-11-26T13:47:12Z DEBUG app::plugin_manager] Firing post_operation hooks
[2023-11-26T13:47:12Z TRACE app::plugin_manager] Firing post_operation for TestPlugin 0.1.0+unit-test
[2023-11-26T13:47:12Z INFO  app] Data after applying plugins: 42
[2023-11-26T13:47:12Z DEBUG app::plugin_manager] Unloading plugins
[2023-11-26T13:47:12Z TRACE app::plugin_manager] Firing on_plugin_unload for TestPlugin 0.1.0+unit-test
```

## Further reading

This toy project is inspired by Michael F. Bryan's [Dynamic Loading & Plugins] post.

[Dynamic Loading & Plugins]: https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
