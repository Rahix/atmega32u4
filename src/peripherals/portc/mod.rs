#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port C Input"]
    pub pin: PIN,
    #[doc = "0x01 - Port C Direction"]
    pub ddr: DDR,
    #[doc = "0x02 - Port C Output"]
    pub port: PORT,
}
#[doc = "Port C Input"]
pub struct PIN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port C Input"]
pub mod pin;
#[doc = "Port C Direction"]
pub struct DDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port C Direction"]
pub mod ddr;
#[doc = "Port C Output"]
pub struct PORT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port C Output"]
pub mod port;
