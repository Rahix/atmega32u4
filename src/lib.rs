# ! [ doc = "Peripheral access API for ARDUINO_LEONARDO microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api" ] # ! [ deny ( missing_docs ) ] # ! [ deny ( warnings ) ] # ! [ allow ( non_camel_case_types ) ] # ! [ no_std ] # ! [ feature ( const_fn ) ] # ! [ feature ( try_from ) ]extern crate bare_metal ;
extern crate vcell ;
use core::ops::Deref;
use core::marker::PhantomData;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 8;
#[doc = "IOC"]
pub struct IOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOC {}
impl IOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ioc::RegisterBlock {
        39 as *const _
    }
}
impl Deref for IOC {
    type Target = ioc::RegisterBlock;
    fn deref(&self) -> &ioc::RegisterBlock {
        unsafe { &*IOC::ptr() }
    }
}
#[doc = "IOC"]
pub mod ioc;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "IOC"]
    pub IOC: IOC,
}
impl Peripherals {
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals { IOC: IOC { _marker: PhantomData } }
    }
}
