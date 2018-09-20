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
    _reserved2: [u8; 1usize],
    #[doc = "0x4e - Timer/Counter Register Low Byte"]
    pub tcnt_l: TCNT_L,
    #[doc = "0x4f - Timer/Counter Register High Byte"]
    pub tcnt_h: TCNT_H,
    #[doc = "0x50 - Input Capture Register Low Byte"]
    pub icr_l: ICR_L,
    #[doc = "0x51 - Input Capture Register High Byte"]
    pub icr_h: ICR_H,
    #[doc = "0x52 - Output Compare Register A Low Byte"]
    pub ocr_a_l: OCR_A_L,
    #[doc = "0x53 - Output Compare Register A High Byte"]
    pub ocr_a_h: OCR_A_H,
    #[doc = "0x54 - Output Compare Register B Low Byte"]
    pub ocr_b_l: OCR_B_L,
    #[doc = "0x55 - Output Compare Register B High Byte"]
    pub ocr_b_h: OCR_B_H,
    #[doc = "0x56 - Output Compare Register C Low Byte"]
    pub ocr_c_l: OCR_C_L,
    #[doc = "0x57 - Output Compare Register C High Byte"]
    pub ocr_c_h: OCR_C_H,
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
#[doc = "Timer/Counter Register Low Byte"]
pub struct TCNT_L {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer/Counter Register Low Byte"]
pub mod tcnt_l;
#[doc = "Timer/Counter Register High Byte"]
pub struct TCNT_H {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer/Counter Register High Byte"]
pub mod tcnt_h;
#[doc = "Input Capture Register Low Byte"]
pub struct ICR_L {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Input Capture Register Low Byte"]
pub mod icr_l;
#[doc = "Input Capture Register High Byte"]
pub struct ICR_H {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Input Capture Register High Byte"]
pub mod icr_h;
#[doc = "Output Compare Register A Low Byte"]
pub struct OCR_A_L {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Output Compare Register A Low Byte"]
pub mod ocr_a_l;
#[doc = "Output Compare Register A High Byte"]
pub struct OCR_A_H {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Output Compare Register A High Byte"]
pub mod ocr_a_h;
#[doc = "Output Compare Register B Low Byte"]
pub struct OCR_B_L {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Output Compare Register B Low Byte"]
pub mod ocr_b_l;
#[doc = "Output Compare Register B High Byte"]
pub struct OCR_B_H {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Output Compare Register B High Byte"]
pub mod ocr_b_h;
#[doc = "Output Compare Register C Low Byte"]
pub struct OCR_C_L {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Output Compare Register C Low Byte"]
pub mod ocr_c_l;
#[doc = "Output Compare Register C High Byte"]
pub struct OCR_C_H {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Output Compare Register C High Byte"]
pub mod ocr_c_h;
