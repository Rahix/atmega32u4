#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Interrupt Flag Register"]
    pub tifr1: TIFR1,
    _reserved0: [u8; 56usize],
    #[doc = "0x39 - Timer Interrupt Mask Register"]
    pub timsk1: TIMSK1,
    _reserved1: [u8; 16usize],
    #[doc = "0x4a - Timer/Counter Control Register A"]
    pub tccr1a: TCCR1A,
    #[doc = "0x4b - Timer/Counter Control Register B"]
    pub tccr1b: TCCR1B,
    #[doc = "0x4c - Timer/Counter Control Register C"]
    pub tccr1c: TCCR1C,
}
#[doc = "Timer Interrupt Flag Register"]
pub struct TIFR1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer Interrupt Flag Register"]
pub mod tifr1;
#[doc = "Timer Interrupt Mask Register"]
pub struct TIMSK1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer Interrupt Mask Register"]
pub mod timsk1;
#[doc = "Timer/Counter Control Register A"]
pub struct TCCR1A {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer/Counter Control Register A"]
pub mod tccr1a;
#[doc = "Timer/Counter Control Register B"]
pub struct TCCR1B {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer/Counter Control Register B"]
pub mod tccr1b;
#[doc = "Timer/Counter Control Register C"]
pub struct TCCR1C {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer/Counter Control Register C"]
pub mod tccr1c;
