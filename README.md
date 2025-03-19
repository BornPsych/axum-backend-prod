## Dev (REPL)

> NOTE: Install baocn with `cargo install bacon`.

```sh
# Terminal 1 - To run the server.
bacon run -- main.rs

# Terminal 2 - To run the quick_dev.
bacon test tests quick_dev.rs -- -q -- --nocapture
```


## Dev

```sh
# Terminal 1 - To run the server.
cargo run

# Terminal 2 - To run the tests.
cargo run --example quick_dev
```
