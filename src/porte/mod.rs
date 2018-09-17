#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port E Input"]
    pub pin: PIN,
    #[doc = "0x01 - Port E Direction"]
    pub ddr: DDR,
    #[doc = "0x02 - Port E Output"]
    pub port: PORT,
}
#[doc = "Port E Input"]
pub struct PIN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port E Input"]
pub mod pin;
#[doc = "Port E Direction"]
pub struct DDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port E Direction"]
pub mod ddr;
#[doc = "Port E Output"]
pub struct PORT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port E Output"]
pub mod port;
