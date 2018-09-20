#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer0 Interrupt Flag Register"]
    pub tifr0: TIFR0,
    _reserved0: [u8; 14usize],
    #[doc = "0x0f - Timer0 Control Register A"]
    pub tccr0a: TCCR0A,
    #[doc = "0x10 - Timer0 Control Register B"]
    pub tccr0b: TCCR0B,
    #[doc = "0x11 - Timer0 Register"]
    pub tcnt0: TCNT0,
    #[doc = "0x12 - Timer0 Output Compare Register A"]
    pub ocr0a: OCR0A,
    #[doc = "0x13 - Timer0 Output Compare Register B"]
    pub ocr0b: OCR0B,
    _reserved1: [u8; 37usize],
    #[doc = "0x39 - Timer0 Interrupt Mask Register"]
    pub timsk0: TIMSK0,
}
#[doc = "Timer0 Interrupt Flag Register"]
pub struct TIFR0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer0 Interrupt Flag Register"]
pub mod tifr0;
#[doc = "Timer0 Control Register A"]
pub struct TCCR0A {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer0 Control Register A"]
pub mod tccr0a;
#[doc = "Timer0 Control Register B"]
pub struct TCCR0B {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer0 Control Register B"]
pub mod tccr0b;
#[doc = "Timer0 Register"]
pub struct TCNT0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer0 Register"]
pub mod tcnt0;
#[doc = "Timer0 Output Compare Register A"]
pub struct OCR0A {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer0 Output Compare Register A"]
pub mod ocr0a;
#[doc = "Timer0 Output Compare Register B"]
pub struct OCR0B {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer0 Output Compare Register B"]
pub mod ocr0b;
#[doc = "Timer0 Interrupt Mask Register"]
pub struct TIMSK0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer0 Interrupt Mask Register"]
pub mod timsk0;
