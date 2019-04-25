#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Interrupt Flag Register"]
    pub tifr: TIFR,
    _reserved0: [u8; 56usize],
    #[doc = "0x39 - Timer Interrupt Mask Register"]
    pub timsk: TIMSK,
    _reserved1: [u8; 75usize],
    #[doc = "0x85 - Timer/Counter Register"]
    pub tcnt: TCNT,
    #[doc = "0x86 - Timer/Counter High Byte"]
    pub tch: TCH,
    #[doc = "0x87 - Timer/Counter Control Register A"]
    pub tccr_a: TCCR_A,
    #[doc = "0x88 - Timer/Counter Control Register B"]
    pub tccr_b: TCCR_B,
    #[doc = "0x89 - Timer/Counter Control Register C"]
    pub tccr_c: TCCR_C,
    #[doc = "0x8a - Timer/Counter Control Register D"]
    pub tccr_d: TCCR_D,
    #[doc = "0x8b - Timer/Counter Control Register E"]
    pub tccr_e: TCCR_E,
    _reserved2: [u8; 10usize],
    #[doc = "0x96 - Output Compare Register A"]
    pub ocr_a: OCR_A,
    #[doc = "0x97 - Output Compare Register B"]
    pub ocr_b: OCR_B,
    #[doc = "0x98 - Output Compare Register B"]
    pub ocr_c: OCR_C,
    #[doc = "0x99 - Output Compare Register B"]
    pub ocr_d: OCR_D,
    _reserved3: [u8; 1usize],
    #[doc = "0x9b - Dead Time Value"]
    pub dt: DT,
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
#[doc = "Timer/Counter Register"]
pub struct TCNT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer/Counter Register"]
pub mod tcnt;
#[doc = "Timer/Counter High Byte"]
pub struct TCH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer/Counter High Byte"]
pub mod tch;
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
#[doc = "Timer/Counter Control Register D"]
pub struct TCCR_D {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer/Counter Control Register D"]
pub mod tccr_d;
#[doc = "Timer/Counter Control Register E"]
pub struct TCCR_E {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer/Counter Control Register E"]
pub mod tccr_e;
#[doc = "Output Compare Register A"]
pub struct OCR_A {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Output Compare Register A"]
pub mod ocr_a;
#[doc = "Output Compare Register B"]
pub struct OCR_B {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Output Compare Register B"]
pub mod ocr_b;
#[doc = "Output Compare Register B"]
pub struct OCR_C {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Output Compare Register B"]
pub mod ocr_c;
#[doc = "Output Compare Register B"]
pub struct OCR_D {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Output Compare Register B"]
pub mod ocr_d;
#[doc = "Dead Time Value"]
pub struct DT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Dead Time Value"]
pub mod dt;
