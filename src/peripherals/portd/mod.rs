#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port D Input Pins Address"]
    pub pin: PIN,
    #[doc = "0x01 - Port D Data Direction Register"]
    pub ddr: DDR,
    #[doc = "0x02 - Port D Output/Data Register"]
    pub port: PORT,
}
#[doc = "Port D Input Pins Address"]
pub struct PIN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port D Input Pins Address"]
pub mod pin;
#[doc = "Port D Data Direction Register"]
pub struct DDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port D Data Direction Register"]
pub mod ddr;
#[doc = "Port D Output/Data Register"]
pub struct PORT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port D Output/Data Register"]
pub mod port;
