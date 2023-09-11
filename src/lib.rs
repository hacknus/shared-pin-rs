//! An abstraction of the embedded_hal::digital::v2 OutputPin and InputPin
//! to be shared between functions using an Rc<RefCell<Pin>>
#![no_std]
#![allow(dead_code)]
#![deny(missing_docs)]
#![deny(warnings)]

extern crate alloc;

use alloc::rc::Rc;
use core::cell::RefCell;

use embedded_hal::digital::v2::{InputPin, OutputPin};

/// SharedPin struct containing the Rc<RefCell<Pin>>
pub struct SharedPin<Pin> {
    pin: Rc<RefCell<Pin>>,
}

impl<Pin> SharedPin<Pin>
{
    /// Create a new SharedPin instance by supplying a pin.
    pub fn new(pin: Pin) -> Self {
        Self {
            pin: Rc::new(RefCell::new(pin)),
        }
    }
}

/// Clone implementation using Rc::clone()
impl<Pin> Clone for SharedPin<Pin> {
    fn clone(&self) -> Self {
        Self {
            pin: Rc::clone(&self.pin),
        }
    }
}


impl<Pin> OutputPin for SharedPin<Pin>
    where
        Pin: OutputPin,
{
    type Error = ();

    /// Borrows the RefCell and calls set_low() on the pin
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.borrow_mut().set_low().map_err(|_e| ())
    }

    /// Borrows the RefCell and calls set_high() on the pin
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.borrow_mut().set_high().map_err(|_e| ())
    }
}

impl<Pin> InputPin for SharedPin<Pin>
    where
        Pin: InputPin,
{
    type Error = ();

    /// Borrows the RefCell and calls is_high() on the pin
    fn is_high(&self) -> Result<bool, Self::Error> {
        self.pin.borrow_mut().is_high().map_err(|_e| ())
    }

    /// Borrows the RefCell and calls is_low() on the pin
    fn is_low(&self) -> Result<bool, Self::Error> {
        self.pin.borrow_mut().is_low().map_err(|_e| ())
    }
}
