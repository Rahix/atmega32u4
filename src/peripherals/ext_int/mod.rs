#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - External Interrupt Flag Register"]
    pub eifr: EIFR,
    #[doc = "0x01 - External Interrupt Mask Register"]
    pub eimsk: EIMSK,
    _reserved0: [u8; 43usize],
    #[doc = "0x2d - External Interrupt Control Register A"]
    pub eicra: EICRA,
    #[doc = "0x2e - External Interrupt Control Register B"]
    pub eicrb: EICRB,
}
#[doc = "External Interrupt Flag Register"]
pub struct EIFR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "External Interrupt Flag Register"]
pub mod eifr;
#[doc = "External Interrupt Mask Register"]
pub struct EIMSK {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "External Interrupt Mask Register"]
pub mod eimsk;
#[doc = "External Interrupt Control Register A"]
pub struct EICRA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "External Interrupt Control Register A"]
pub mod eicra;
#[doc = "External Interrupt Control Register B"]
pub struct EICRB {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "External Interrupt Control Register B"]
pub mod eicrb;
