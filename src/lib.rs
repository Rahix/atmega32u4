# ! [ doc = "Peripheral access API for ARDUINO_LEONARDO microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api" ] # ! [ deny ( missing_docs ) ]  # ! [ allow ( non_camel_case_types ) ] # ! [ no_std ] # ! [ feature ( const_fn ) ] # ! [ feature ( try_from ) ]extern crate bare_metal ;
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
}
impl Peripherals {
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PORTB: PORTB { _marker: PhantomData },
            PORTC: PORTC { _marker: PhantomData },
            PORTD: PORTD { _marker: PhantomData },
        }
    }
}
