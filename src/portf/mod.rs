#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port F Input"]
    pub pin: PIN,
    #[doc = "0x01 - Port F Direction"]
    pub ddr: DDR,
    #[doc = "0x02 - Port F Output"]
    pub port: PORT,
}
#[doc = "Port F Input"]
pub struct PIN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port F Input"]
pub mod pin;
#[doc = "Port F Direction"]
pub struct DDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port F Direction"]
pub mod ddr;
#[doc = "Port F Output"]
pub struct PORT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port F Output"]
pub mod port;
