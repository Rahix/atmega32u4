#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Interrupt Flag Register"]
    pub tifr: TIFR,
    _reserved0: [u8; 14usize],
    #[doc = "0x0f - Timer/Counter Control Register A"]
    pub tccr_a: TCCR_A,
    #[doc = "0x10 - Timer/Counter Control Register B"]
    pub tccr_b: TCCR_B,
    #[doc = "0x11 - Timer/Counter Register"]
    pub tcnt: TCNT,
    #[doc = "0x12 - Timer Output Compare Register A"]
    pub ocr_a: OCR_A,
    #[doc = "0x13 - Timer Output Compare Register B"]
    pub ocr_b: OCR_B,
    _reserved1: [u8; 37usize],
    #[doc = "0x39 - Timer Interrupt Mask Register"]
    pub timsk: TIMSK,
}
#[doc = "Timer Interrupt Flag Register"]
pub struct TIFR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer Interrupt Flag Register"]
pub mod tifr;
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
#[doc = "Timer/Counter Register"]
pub struct TCNT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer/Counter Register"]
pub mod tcnt;
#[doc = "Timer Output Compare Register A"]
pub struct OCR_A {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer Output Compare Register A"]
pub mod ocr_a;
#[doc = "Timer Output Compare Register B"]
pub struct OCR_B {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer Output Compare Register B"]
pub mod ocr_b;
#[doc = "Timer Interrupt Mask Register"]
pub struct TIMSK {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer Interrupt Mask Register"]
pub mod timsk;
