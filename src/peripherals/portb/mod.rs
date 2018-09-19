#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port B Input"]
    pub pin: PIN,
    #[doc = "0x01 - Port B Direction"]
    pub ddr: DDR,
    #[doc = "0x02 - Port B Output"]
    pub port: PORT,
}
#[doc = "Port B Input"]
pub struct PIN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port B Input"]
pub mod pin;
#[doc = "Port B Direction"]
pub struct DDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port B Direction"]
pub mod ddr;
#[doc = "Port B Output"]
pub struct PORT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port B Output"]
pub mod port;
