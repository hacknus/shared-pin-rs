# Shared pin abstraction for embedded rust

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

