#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Interrupt Flag Register"]
    pub tifr: TIFR,
    _reserved0: [u8; 56usize],
    #[doc = "0x39 - Timer Interrupt Mask Register"]
    pub timsk: TIMSK,
    _reserved1: [u8; 16usize],
    #[doc = "0x4a - Timer/Counter Control Register A"]
    pub tccr_a: TCCR_A,
    #[doc = "0x4b - Timer/Counter Control Register B"]
    pub tccr_b: TCCR_B,
    #[doc = "0x4c - Timer/Counter Control Register C"]
    pub tccr_c: TCCR_C,
}
#[doc = "Timer Interrupt Flag Register"]
pub struct TIFR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer Interrupt Flag Register"]
pub mod tifr;
#[doc = "Timer Interrupt Mask Register"]
pub struct TIMSK {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer Interrupt Mask Register"]
pub mod timsk;
#[doc = "Timer/Counter Control Register A"]
pub struct TCCR_A {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer/Counter Control Register A"]
pub mod tccr_a;
#[doc = "Timer/Counter Control Register B"]
pub struct TCCR_B {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer/Counter Control Register B"]
pub mod tccr_b;
#[doc = "Timer/Counter Control Register C"]
pub struct TCCR_C {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer/Counter Control Register C"]
pub mod tccr_c;
