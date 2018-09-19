#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port D Input"]
    pub pin: PIN,
    #[doc = "0x01 - Port D Direction"]
    pub ddr: DDR,
    #[doc = "0x02 - Port D Output"]
    pub port: PORT,
}
#[doc = "Port D Input"]
pub struct PIN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port D Input"]
pub mod pin;
#[doc = "Port D Direction"]
pub struct DDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port D Direction"]
pub mod ddr;
#[doc = "Port D Output"]
pub struct PORT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port D Output"]
pub mod port;
