#![ doc = "Peripheral access API for ARDUINO_LEONARDO microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api" ]
#![ deny ( missing_docs ) ]
#![ allow ( non_camel_case_types ) ]
#![ no_std ]
#![ feature ( const_fn ) ]
#![ feature ( try_from ) ]
#![feature(asm)]

extern crate vcell ;
use core::ops::Deref;
use core::marker::PhantomData;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 8;
#[doc = "PORTB"]
pub struct PORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTB {}
impl PORTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portb::RegisterBlock {
        35 as *const _
    }
}
impl Deref for PORTB {
    type Target = portb::RegisterBlock;
    fn deref(&self) -> &portb::RegisterBlock {
        unsafe { &*PORTB::ptr() }
    }
}
#[doc = "PORTB"]
pub mod portb;
#[doc = "PORTC"]
pub struct PORTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTC {}
impl PORTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portc::RegisterBlock {
        38 as *const _
    }
}
impl Deref for PORTC {
    type Target = portc::RegisterBlock;
    fn deref(&self) -> &portc::RegisterBlock {
        unsafe { &*PORTC::ptr() }
    }
}
#[doc = "PORTC"]
pub mod portc;
#[doc = "PORTD"]
pub struct PORTD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTD {}
impl PORTD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portd::RegisterBlock {
        41 as *const _
    }
}
impl Deref for PORTD {
    type Target = portd::RegisterBlock;
    fn deref(&self) -> &portd::RegisterBlock {
        unsafe { &*PORTD::ptr() }
    }
}
#[doc = "PORTD"]
pub mod portd;
#[doc = "PORTE"]
pub struct PORTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTE {}
impl PORTE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const porte::RegisterBlock {
        44 as *const _
    }
}
impl Deref for PORTE {
    type Target = porte::RegisterBlock;
    fn deref(&self) -> &porte::RegisterBlock {
        unsafe { &*PORTE::ptr() }
    }
}
#[doc = "PORTE"]
pub mod porte;
#[doc = "PORTF"]
pub struct PORTF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTF {}
impl PORTF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portf::RegisterBlock {
        47 as *const _
    }
}
impl Deref for PORTF {
    type Target = portf::RegisterBlock;
    fn deref(&self) -> &portf::RegisterBlock {
        unsafe { &*PORTF::ptr() }
    }
}
#[doc = "PORTF"]
pub mod portf;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PORTB"]
    pub PORTB: PORTB,
    #[doc = "PORTC"]
    pub PORTC: PORTC,
    #[doc = "PORTD"]
    pub PORTD: PORTD,
    #[doc = "PORTE"]
    pub PORTE: PORTE,
    #[doc = "PORTF"]
    pub PORTF: PORTF,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    #[allow(unused_assignments)]
    pub fn take() -> Option<Self> {
        let mut sreg: u8 = 0;
        unsafe { asm!(
            "in $0,0x35\n\tcli"
            : "=r"(sreg)
            :
            :
            : "volatile"
        ); }
        let result = if unsafe { DEVICE_PERIPHERALS } {
            None
        } else {
            Some(unsafe { Peripherals::steal() })
        };
        unsafe { asm!(
            "out 0x35,$0"
            :
            : "r"(sreg)
            :
            : "volatile"
        ); }
        result
    }

    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PORTB: PORTB { _marker: PhantomData },
            PORTC: PORTC { _marker: PhantomData },
            PORTD: PORTD { _marker: PhantomData },
            PORTE: PORTE { _marker: PhantomData },
            PORTF: PORTF { _marker: PhantomData },
        }
    }
}
