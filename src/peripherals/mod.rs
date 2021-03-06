use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 8;
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
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
#[doc = "8-Bit Timer/Counter0 with PWM"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        53 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "8-Bit Timer/Counter0 with PWM"]
pub mod timer0;
#[doc = "16-Bit Timer/Counter1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer1::RegisterBlock {
        54 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer1::RegisterBlock;
    fn deref(&self) -> &timer1::RegisterBlock {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "16-Bit Timer/Counter1"]
pub mod timer1;
#[doc = "16-Bit Timer/Counter3"]
pub struct TIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER3 {}
impl TIMER3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer3::RegisterBlock {
        56 as *const _
    }
}
impl Deref for TIMER3 {
    type Target = timer3::RegisterBlock;
    fn deref(&self) -> &timer3::RegisterBlock {
        unsafe { &*TIMER3::ptr() }
    }
}
#[doc = "16-Bit Timer/Counter3"]
pub mod timer3;
#[doc = "10-bit High Speed Timer/Counter4"]
pub struct TIMER4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER4 {}
impl TIMER4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer4::RegisterBlock {
        57 as *const _
    }
}
impl Deref for TIMER4 {
    type Target = timer4::RegisterBlock;
    fn deref(&self) -> &timer4::RegisterBlock {
        unsafe { &*TIMER4::ptr() }
    }
}
#[doc = "10-bit High Speed Timer/Counter4"]
pub mod timer4;
#[doc = "External Interrupts"]
pub struct EXT_INT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXT_INT {}
impl EXT_INT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ext_int::RegisterBlock {
        60 as *const _
    }
}
impl Deref for EXT_INT {
    type Target = ext_int::RegisterBlock;
    fn deref(&self) -> &ext_int::RegisterBlock {
        unsafe { &*EXT_INT::ptr() }
    }
}
#[doc = "External Interrupts"]
pub mod ext_int;
#[doc = "USB Controller"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb::RegisterBlock {
        215 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &usb::RegisterBlock {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB Controller"]
pub mod usb;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
pub(crate) static mut DEVICE_PERIPHERALS: bool = false;
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
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER3"]
    pub TIMER3: TIMER3,
    #[doc = "TIMER4"]
    pub TIMER4: TIMER4,
    #[doc = "EXT_INT"]
    pub EXT_INT: EXT_INT,
    #[doc = "USB"]
    pub USB: USB,
}
impl Peripherals {
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PORTB: PORTB {
                _marker: PhantomData,
            },
            PORTC: PORTC {
                _marker: PhantomData,
            },
            PORTD: PORTD {
                _marker: PhantomData,
            },
            PORTE: PORTE {
                _marker: PhantomData,
            },
            PORTF: PORTF {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER3: TIMER3 {
                _marker: PhantomData,
            },
            TIMER4: TIMER4 {
                _marker: PhantomData,
            },
            EXT_INT: EXT_INT {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
        }
    }
}
