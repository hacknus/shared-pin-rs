# Shared pin abstraction for embedded rust

[![crates.io](https://img.shields.io/crates/v/shared-pin.svg)](https://crates.io/crates/shared-pin)
[![Docs](https://docs.rs/shared-pin/badge.svg)](https://docs.rs/shared-pin)
[![Rust](https://github.com/hacknus/shared-pin-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/hacknus/shared-pin-rs/actions/workflows/rust.yml)

This is a simple abstraction for embedded rust applications to share a pin between different functions.
It uses an `Rc<RefCell<Pin>>` to share the `embedded_hal::digital::v2` pin.  
Should be able to be passed on to any function that expects an OutputPin or an InputPin

  
Example:

```rust
use shared_pin::SharedPin;
use embedded_hal::digital::v2::OutputPin;

pub fn do_something_with_the_cloned_pin<PIN>(pin: PIN)
    where PIN: OuputPin
{
    pin.set_high();
    // ...
}

{
    let mut shared_output_pin_1 = SharedPin::new(output_pin);
    let mut shared_output_pin_2 = shared_output_pin_1.clone();
    let mut shared_output_pin_3 = shared_output_pin_1.clone();
    do_something_with_the_cloned_pin(shared_output_pin_2);
    shared_output_pin_3.set_low();
    
    let mut shared_input_pin_1 = SharedPin::new(input_pin);
    let mut shared_input_pin_2 = shared_input_pin_1.clone();
    let mut shared_input_pin_3 = shared_input_pin_1.clone();
    
    if shared_input_pin_3.is_low() {
        // ...
    }
}
```

TODO:
- [ ] make thread safe for freeRTOS

