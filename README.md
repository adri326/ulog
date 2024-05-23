# `ulog`

A tiny rust logging library, meant for embedded devices with a very small space budget:
- no allocations
- no global variables
- no synchronization
- no std
- optional formatting
- logging levels, helper macros, helper structs

## When (not) to use it

This library is especially useful when:

- you don't want a global logger (otherwise see [rust-lang/log](https://github.com/rust-lang/log))
- you don't want allocations or synchronization (otherwise see [slog-rs/slog](https://github.com/slog-rs/slog))
- you want to easily opt out of any kind of formatting

Typical scenarios for using this library include embedded programs with very limited resources,
or rust functions called through FFI that may not have access to an allocator or global, mutable variables.

## Example

To start using this library, simply implement `ULog` on the struct responsible with logging:

```rust
use ulog::{ULog, ULogData};

struct MyLogger;

impl ULog for MyLogger {
    fn log_begin(&self, log_data: &ULogData) {
        print!("[MyLogger {}]", log_data.level.as_short_str());
    }

    fn log_str(&self, _log_data: &ULogData, string: &str) {
        print!(" {string}");
    }

    fn log_format<T: std::fmt::Debug>(&self, _log_data: &ULogData, key: &str, value: &T) {
        print!(" {key} => {value:?}");
    }

    fn log_end(&self, _log_data: &ULogData) {
        println!("");
    }
}

fn main() {
    let logger = MyLogger;

    ulog::info!(logger, "Hello, this is an info!");

    ulog::error!(logger, "Whoops, something went wrong!", "error_code" => 42);
}
```

You can also pass `&impl ULog` in a library or function:

```rust
use ulog::ULog;

fn fibonacci(n: u32, logger: &impl ULog) -> u32 {
    ulog::info!(logger, "fibonacci", "step" => n);

    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1, logger) + fibonacci(n - 2, logger)
    }
}
```
