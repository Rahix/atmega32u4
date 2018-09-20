#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port C Input Pins Address"]
    pub pin: PIN,
    #[doc = "0x01 - Port C Data Direction Register"]
    pub ddr: DDR,
    #[doc = "0x02 - Port C Output/Data Register"]
    pub port: PORT,
}
#[doc = "Port C Input Pins Address"]
pub struct PIN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port C Input Pins Address"]
pub mod pin;
#[doc = "Port C Data Direction Register"]
pub struct DDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port C Data Direction Register"]
pub mod ddr;
#[doc = "Port C Output/Data Register"]
pub struct PORT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port C Output/Data Register"]
pub mod port;
