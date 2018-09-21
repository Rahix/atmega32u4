//! Low level access to ATmega32U4 registers
#![no_std]
#![feature(asm, const_fn, try_from)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]

extern crate vcell;
extern crate bare_metal;

pub mod interrupt;

mod peripherals;
pub use peripherals::*;

impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        interrupt::free(|_| if unsafe { DEVICE_PERIPHERALS } {
            None
        } else {
            Some(unsafe { Peripherals::steal() })
        })
    }
}
