// #![deny(missing_docs)]
#![allow(non_camel_case_types)]
#![no_std]
#![feature(const_fn)]
#![feature(try_from)]
#![feature(asm)]

extern crate vcell;
extern crate bare_metal;

pub mod interrupt;

mod peripherals;
pub use peripherals::*;

impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    #[allow(unused_assignments)]
    pub fn take() -> Option<Self> {
        interrupt::free(|_| if unsafe { DEVICE_PERIPHERALS } {
            None
        } else {
            Some(unsafe { Peripherals::steal() })
        })
    }
}
