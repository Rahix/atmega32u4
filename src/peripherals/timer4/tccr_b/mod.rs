#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::TCCR_B {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct PWM_XR {
    bits: bool,
}
impl PWM_XR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DTPSR {
    bits: u8,
}
impl DTPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSR {
    #[doc = "No clock source (Timer/Counter stopped)"]
    STOPPED,
    #[doc = "clk/1 (No Prescaler)"]
    CLK,
    #[doc = "clk/2"]
    CLK_2,
    #[doc = "clk/4"]
    CLK_4,
    #[doc = "clk/8"]
    CLK_8,
    #[doc = "clk/16"]
    CLK_16,
    #[doc = "clk/32"]
    CLK_32,
    #[doc = "clk/64"]
    CLK_64,
    #[doc = "clk/128"]
    CLK_128,
    #[doc = "clk/256"]
    CLK_256,
    #[doc = "clk/512"]
    CLK_512,
    #[doc = "clk/1024"]
    CLK_1024,
    #[doc = "clk/2048"]
    CLK_2048,
    #[doc = "clk/4096"]
    CLK_4096,
    #[doc = "clk/8192"]
    CLK_8192,
    #[doc = "clk/16384"]
    CLK_16384,
}
impl CSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSR::STOPPED => 0,
            CSR::CLK => 1,
            CSR::CLK_2 => 2,
            CSR::CLK_4 => 3,
            CSR::CLK_8 => 4,
            CSR::CLK_16 => 5,
            CSR::CLK_32 => 6,
            CSR::CLK_64 => 7,
            CSR::CLK_128 => 8,
            CSR::CLK_256 => 9,
            CSR::CLK_512 => 10,
            CSR::CLK_1024 => 11,
            CSR::CLK_2048 => 12,
            CSR::CLK_4096 => 13,
            CSR::CLK_8192 => 14,
            CSR::CLK_16384 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSR {
        match value {
            0 => CSR::STOPPED,
            1 => CSR::CLK,
            2 => CSR::CLK_2,
            3 => CSR::CLK_4,
            4 => CSR::CLK_8,
            5 => CSR::CLK_16,
            6 => CSR::CLK_32,
            7 => CSR::CLK_64,
            8 => CSR::CLK_128,
            9 => CSR::CLK_256,
            10 => CSR::CLK_512,
            11 => CSR::CLK_1024,
            12 => CSR::CLK_2048,
            13 => CSR::CLK_4096,
            14 => CSR::CLK_8192,
            15 => CSR::CLK_16384,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline]
    pub fn is_stopped(&self) -> bool {
        *self == CSR::STOPPED
    }
    #[doc = "Checks if the value of the field is `CLK`"]
    #[inline]
    pub fn is_clk(&self) -> bool {
        *self == CSR::CLK
    }
    #[doc = "Checks if the value of the field is `CLK_2`"]
    #[inline]
    pub fn is_clk_2(&self) -> bool {
        *self == CSR::CLK_2
    }
    #[doc = "Checks if the value of the field is `CLK_4`"]
    #[inline]
    pub fn is_clk_4(&self) -> bool {
        *self == CSR::CLK_4
    }
    #[doc = "Checks if the value of the field is `CLK_8`"]
    #[inline]
    pub fn is_clk_8(&self) -> bool {
        *self == CSR::CLK_8
    }
    #[doc = "Checks if the value of the field is `CLK_16`"]
    #[inline]
    pub fn is_clk_16(&self) -> bool {
        *self == CSR::CLK_16
    }
    #[doc = "Checks if the value of the field is `CLK_32`"]
    #[inline]
    pub fn is_clk_32(&self) -> bool {
        *self == CSR::CLK_32
    }
    #[doc = "Checks if the value of the field is `CLK_64`"]
    #[inline]
    pub fn is_clk_64(&self) -> bool {
        *self == CSR::CLK_64
    }
    #[doc = "Checks if the value of the field is `CLK_128`"]
    #[inline]
    pub fn is_clk_128(&self) -> bool {
        *self == CSR::CLK_128
    }
    #[doc = "Checks if the value of the field is `CLK_256`"]
    #[inline]
    pub fn is_clk_256(&self) -> bool {
        *self == CSR::CLK_256
    }
    #[doc = "Checks if the value of the field is `CLK_512`"]
    #[inline]
    pub fn is_clk_512(&self) -> bool {
        *self == CSR::CLK_512
    }
    #[doc = "Checks if the value of the field is `CLK_1024`"]
    #[inline]
    pub fn is_clk_1024(&self) -> bool {
        *self == CSR::CLK_1024
    }
    #[doc = "Checks if the value of the field is `CLK_2048`"]
    #[inline]
    pub fn is_clk_2048(&self) -> bool {
        *self == CSR::CLK_2048
    }
    #[doc = "Checks if the value of the field is `CLK_4096`"]
    #[inline]
    pub fn is_clk_4096(&self) -> bool {
        *self == CSR::CLK_4096
    }
    #[doc = "Checks if the value of the field is `CLK_8192`"]
    #[inline]
    pub fn is_clk_8192(&self) -> bool {
        *self == CSR::CLK_8192
    }
    #[doc = "Checks if the value of the field is `CLK_16384`"]
    #[inline]
    pub fn is_clk_16384(&self) -> bool {
        *self == CSR::CLK_16384
    }
}
#[doc = r" Proxy"]
pub struct _PWM_XW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_XW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTPSW<'a> {
    w: &'a mut W,
}
impl<'a> _DTPSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CS`"]
pub enum CSW {
    #[doc = "No clock source (Timer/Counter stopped)"]
    STOPPED,
    #[doc = "clk/1 (No Prescaler)"]
    CLK,
    #[doc = "clk/2"]
    CLK_2,
    #[doc = "clk/4"]
    CLK_4,
    #[doc = "clk/8"]
    CLK_8,
    #[doc = "clk/16"]
    CLK_16,
    #[doc = "clk/32"]
    CLK_32,
    #[doc = "clk/64"]
    CLK_64,
    #[doc = "clk/128"]
    CLK_128,
    #[doc = "clk/256"]
    CLK_256,
    #[doc = "clk/512"]
    CLK_512,
    #[doc = "clk/1024"]
    CLK_1024,
    #[doc = "clk/2048"]
    CLK_2048,
    #[doc = "clk/4096"]
    CLK_4096,
    #[doc = "clk/8192"]
    CLK_8192,
    #[doc = "clk/16384"]
    CLK_16384,
}
impl CSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSW::STOPPED => 0,
            CSW::CLK => 1,
            CSW::CLK_2 => 2,
            CSW::CLK_4 => 3,
            CSW::CLK_8 => 4,
            CSW::CLK_16 => 5,
            CSW::CLK_32 => 6,
            CSW::CLK_64 => 7,
            CSW::CLK_128 => 8,
            CSW::CLK_256 => 9,
            CSW::CLK_512 => 10,
            CSW::CLK_1024 => 11,
            CSW::CLK_2048 => 12,
            CSW::CLK_4096 => 13,
            CSW::CLK_8192 => 14,
            CSW::CLK_16384 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSW<'a> {
    w: &'a mut W,
}
impl<'a> _CSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline]
    pub fn stopped(self) -> &'a mut W {
        self.variant(CSW::STOPPED)
    }
    #[doc = "clk/1 (No Prescaler)"]
    #[inline]
    pub fn clk(self) -> &'a mut W {
        self.variant(CSW::CLK)
    }
    #[doc = "clk/2"]
    #[inline]
    pub fn clk_2(self) -> &'a mut W {
        self.variant(CSW::CLK_2)
    }
    #[doc = "clk/4"]
    #[inline]
    pub fn clk_4(self) -> &'a mut W {
        self.variant(CSW::CLK_4)
    }
    #[doc = "clk/8"]
    #[inline]
    pub fn clk_8(self) -> &'a mut W {
        self.variant(CSW::CLK_8)
    }
    #[doc = "clk/16"]
    #[inline]
    pub fn clk_16(self) -> &'a mut W {
        self.variant(CSW::CLK_16)
    }
    #[doc = "clk/32"]
    #[inline]
    pub fn clk_32(self) -> &'a mut W {
        self.variant(CSW::CLK_32)
    }
    #[doc = "clk/64"]
    #[inline]
    pub fn clk_64(self) -> &'a mut W {
        self.variant(CSW::CLK_64)
    }
    #[doc = "clk/128"]
    #[inline]
    pub fn clk_128(self) -> &'a mut W {
        self.variant(CSW::CLK_128)
    }
    #[doc = "clk/256"]
    #[inline]
    pub fn clk_256(self) -> &'a mut W {
        self.variant(CSW::CLK_256)
    }
    #[doc = "clk/512"]
    #[inline]
    pub fn clk_512(self) -> &'a mut W {
        self.variant(CSW::CLK_512)
    }
    #[doc = "clk/1024"]
    #[inline]
    pub fn clk_1024(self) -> &'a mut W {
        self.variant(CSW::CLK_1024)
    }
    #[doc = "clk/2048"]
    #[inline]
    pub fn clk_2048(self) -> &'a mut W {
        self.variant(CSW::CLK_2048)
    }
    #[doc = "clk/4096"]
    #[inline]
    pub fn clk_4096(self) -> &'a mut W {
        self.variant(CSW::CLK_4096)
    }
    #[doc = "clk/8192"]
    #[inline]
    pub fn clk_8192(self) -> &'a mut W {
        self.variant(CSW::CLK_8192)
    }
    #[doc = "clk/16384"]
    #[inline]
    pub fn clk_16384(self) -> &'a mut W {
        self.variant(CSW::CLK_16384)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PSRW<'a> {
    w: &'a mut W,
}
impl<'a> _PSRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 7 - PWM Inversion Mode"]
    #[inline]
    pub fn pwm_x(&self) -> PWM_XR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        PWM_XR { bits }
    }
    #[doc = "Bits 4:5 - Dead Time Prescaler"]
    #[inline]
    pub fn dtps(&self) -> DTPSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        DTPSR { bits }
    }
    #[doc = "Bits 0:3 - Clock Source"]
    #[inline]
    pub fn cs(&self) -> CSR {
        CSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - PWM Inversion Mode"]
    #[inline]
    pub fn pwm_x(&mut self) -> _PWM_XW {
        _PWM_XW { w: self }
    }
    #[doc = "Bits 4:5 - Dead Time Prescaler"]
    #[inline]
    pub fn dtps(&mut self) -> _DTPSW {
        _DTPSW { w: self }
    }
    #[doc = "Bits 0:3 - Clock Source"]
    #[inline]
    pub fn cs(&mut self) -> _CSW {
        _CSW { w: self }
    }
    #[doc = "Bit 6 - Prescaler Reset"]
    #[inline]
    pub fn psr(&mut self) -> _PSRW {
        _PSRW { w: self }
    }
}
