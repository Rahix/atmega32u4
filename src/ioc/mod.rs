#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port C direction control"]
    pub ddrc: DDRC,
    #[doc = "0x01 - Port C values"]
    pub portc: PORTC,
}
#[doc = "Port C direction control"]
pub struct DDRC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port C direction control"]
pub mod ddrc;
#[doc = "Port C values"]
pub struct PORTC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port C values"]
pub mod portc;
