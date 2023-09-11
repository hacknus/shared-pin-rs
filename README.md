# Shared pin abstraction for embedded rust

[![crates.io](https://img.shields.io/crates/v/shared-pin.svg)](https://crates.io/crates/shared-pin)
[![Docs](https://docs.rs/shared-pin/badge.svg)](https://docs.rs/shared-pin)
[![Rust](https://github.com/hacknus/shared-pin-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/hacknus/shared-pin-rs/actions/workflows/rust.yml)

This is a simple abstraction for embedded rust applications to share a pin between different functions.
It uses an `Rc<RefCell<Pin>>` to share the `embedded_hal::digital::v2` pin.  

  
Example:
```rust
use shared_pin::SharedPin;
{
    let mut shared_pin_1 = SharedPin::new(pin);
    let mut shared_pin_2 = shared_pin_1.clone();
    let mut shared_pin_3 = shared_pin_1.clone();
}
```

TODO:
- [ ] make thread safe for freeRTOS

