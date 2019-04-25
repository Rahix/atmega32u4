#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Hardware Control"]
    pub uhwcon: UHWCON,
    #[doc = "0x01 - USB Control"]
    pub usbcon: USBCON,
    #[doc = "0x02 - USB Status"]
    pub usbsta: USBSTA,
    #[doc = "0x03 - USB Interrupt"]
    pub usbint: USBINT,
    _reserved0: [u8; 5usize],
    #[doc = "0x09 - USB Device Control"]
    pub udcon: UDCON,
    #[doc = "0x0a - USB Device Interrupt"]
    pub udint: UDINT,
    #[doc = "0x0b - USB Device Interrupt Enable"]
    pub udien: UDIEN,
    #[doc = "0x0c - USB Device Address"]
    pub udaddr: UDADDR,
    #[doc = "0x0d - USB Frame Number Low"]
    pub udfnuml: UDFNUML,
    #[doc = "0x0e - USB Frame Number High"]
    pub udfnumh: UDFNUMH,
    #[doc = "0x0f - USB Frame Number CRC Error Flag"]
    pub udmfn: UDMFN,
    _reserved1: [u8; 1usize],
    #[doc = "0x11 - USB Endpoint Interrupt"]
    pub ueintx: UEINTX,
    #[doc = "0x12 - USB Endpoint Number"]
    pub uenum: UENUM,
    #[doc = "0x13 - USB Endpoint Reset"]
    pub uerst: UERST,
    #[doc = "0x14 - USB Endpoint Control"]
    pub ueconx: UECONX,
    #[doc = "0x15 - USB Endpoint Configuration 0"]
    pub uecfg0x: UECFG0X,
    #[doc = "0x16 - USB Endpoint Configuration 1"]
    pub uecfg1x: UECFG1X,
    #[doc = "0x17 - USB Endpoint Status 0"]
    pub uesta0x: UESTA0X,
    #[doc = "0x18 - USB Endpoint Status 1"]
    pub uesta1x: UESTA1X,
    #[doc = "0x19 - USB Endpoint Interrupt Enable"]
    pub ueienx: UEIENX,
    #[doc = "0x1a - USB Endpoint Data"]
    pub uedatx: UEDATX,
    #[doc = "0x1b - USB Endpoint Byte Count Low"]
    pub uebclx: UEBCLX,
    #[doc = "0x1c - USB Endpoint Byte Count High"]
    pub uebchx: UEBCHX,
    #[doc = "0x1d - USB Endpoint Interrupts"]
    pub ueint: UEINT,
}
#[doc = "USB Hardware Control"]
pub struct UHWCON {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Hardware Control"]
pub mod uhwcon;
#[doc = "USB Control"]
pub struct USBCON {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Control"]
pub mod usbcon;
#[doc = "USB Status"]
pub struct USBSTA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Status"]
pub mod usbsta;
#[doc = "USB Interrupt"]
pub struct USBINT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Interrupt"]
pub mod usbint;
#[doc = "USB Device Control"]
pub struct UDCON {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Device Control"]
pub mod udcon;
#[doc = "USB Device Interrupt"]
pub struct UDINT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Device Interrupt"]
pub mod udint;
#[doc = "USB Device Interrupt Enable"]
pub struct UDIEN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Device Interrupt Enable"]
pub mod udien;
#[doc = "USB Device Address"]
pub struct UDADDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Device Address"]
pub mod udaddr;
#[doc = "USB Frame Number Low"]
pub struct UDFNUML {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Frame Number Low"]
pub mod udfnuml;
#[doc = "USB Frame Number High"]
pub struct UDFNUMH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Frame Number High"]
pub mod udfnumh;
#[doc = "USB Frame Number CRC Error Flag"]
pub struct UDMFN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Frame Number CRC Error Flag"]
pub mod udmfn;
#[doc = "USB Endpoint Interrupt"]
pub struct UEINTX {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Interrupt"]
pub mod ueintx;
#[doc = "USB Endpoint Number"]
pub struct UENUM {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Number"]
pub mod uenum;
#[doc = "USB Endpoint Reset"]
pub struct UERST {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Reset"]
pub mod uerst;
#[doc = "USB Endpoint Control"]
pub struct UECONX {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Control"]
pub mod ueconx;
#[doc = "USB Endpoint Configuration 0"]
pub struct UECFG0X {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Configuration 0"]
pub mod uecfg0x;
#[doc = "USB Endpoint Configuration 1"]
pub struct UECFG1X {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Configuration 1"]
pub mod uecfg1x;
#[doc = "USB Endpoint Status 0"]
pub struct UESTA0X {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Status 0"]
pub mod uesta0x;
#[doc = "USB Endpoint Status 1"]
pub struct UESTA1X {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Status 1"]
pub mod uesta1x;
#[doc = "USB Endpoint Interrupt Enable"]
pub struct UEIENX {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Interrupt Enable"]
pub mod ueienx;
#[doc = "USB Endpoint Data"]
pub struct UEDATX {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Data"]
pub mod uedatx;
#[doc = "USB Endpoint Byte Count Low"]
pub struct UEBCLX {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Byte Count Low"]
pub mod uebclx;
#[doc = "USB Endpoint Byte Count High"]
pub struct UEBCHX {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Byte Count High"]
pub mod uebchx;
#[doc = "USB Endpoint Interrupts"]
pub struct UEINT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Interrupts"]
pub mod ueint;
