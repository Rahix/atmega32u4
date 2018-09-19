// #![deny(missing_docs)]
#![allow(non_camel_case_types)]
#![no_std]
#![feature(const_fn)]
#![feature(try_from)]
#![feature(asm)]

extern crate vcell;
extern crate bare_metal;

mod peripherals;

pub use peripherals::*;

impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    #[allow(unused_assignments)]
    pub fn take() -> Option<Self> {
        let mut sreg: u8 = 0;
        unsafe {
            asm!(
            "in $0,0x35\n\tcli"
            : "=r"(sreg)
            :
            :
            : "volatile"
        );
        }
        let result = if unsafe { DEVICE_PERIPHERALS } {
            None
        } else {
            Some(unsafe { Peripherals::steal() })
        };
        unsafe {
            asm!(
            "out 0x35,$0"
            :
            : "r"(sreg)
            :
            : "volatile"
        );
        }
        result
    }
}
