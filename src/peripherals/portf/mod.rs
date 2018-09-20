#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port F Input Pins Address"]
    pub pin: PIN,
    #[doc = "0x01 - Port F Data Direction Register"]
    pub ddr: DDR,
    #[doc = "0x02 - Port F Output/Data Register"]
    pub port: PORT,
}
#[doc = "Port F Input Pins Address"]
pub struct PIN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port F Input Pins Address"]
pub mod pin;
#[doc = "Port F Data Direction Register"]
pub struct DDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port F Data Direction Register"]
pub mod ddr;
#[doc = "Port F Output/Data Register"]
pub struct PORT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port F Output/Data Register"]
pub mod port;
