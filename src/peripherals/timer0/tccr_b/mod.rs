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
pub struct WGM2R {
    bits: bool,
}
impl WGM2R {
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
#[doc = "Possible values of the field `CS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSR {
    #[doc = "No clock source (Timer/Counter stopped)"]
    STOPPED,
    #[doc = "clkIO/1 (No prescaling)"]
    IO,
    #[doc = "clkIO/8 (From prescaler)"]
    IO_8,
    #[doc = "clkIO/64 (From prescaler)"]
    IO_64,
    #[doc = "clkIO/256 (From prescaler)"]
    IO_256,
    #[doc = "clkIO/1024 (From prescaler)"]
    IO_1024,
    #[doc = "External clock source on T0 pin. Clock on falling edge."]
    EXT_FALLING,
    #[doc = "External clock source on T0 pin. Clock on rising edge."]
    EXT_RISING,
}
impl CSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSR::STOPPED => 0,
            CSR::IO => 1,
            CSR::IO_8 => 2,
            CSR::IO_64 => 3,
            CSR::IO_256 => 4,
            CSR::IO_1024 => 5,
            CSR::EXT_FALLING => 6,
            CSR::EXT_RISING => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSR {
        match value {
            0 => CSR::STOPPED,
            1 => CSR::IO,
            2 => CSR::IO_8,
            3 => CSR::IO_64,
            4 => CSR::IO_256,
            5 => CSR::IO_1024,
            6 => CSR::EXT_FALLING,
            7 => CSR::EXT_RISING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline]
    pub fn is_stopped(&self) -> bool {
        *self == CSR::STOPPED
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline]
    pub fn is_io(&self) -> bool {
        *self == CSR::IO
    }
    #[doc = "Checks if the value of the field is `IO_8`"]
    #[inline]
    pub fn is_io_8(&self) -> bool {
        *self == CSR::IO_8
    }
    #[doc = "Checks if the value of the field is `IO_64`"]
    #[inline]
    pub fn is_io_64(&self) -> bool {
        *self == CSR::IO_64
    }
    #[doc = "Checks if the value of the field is `IO_256`"]
    #[inline]
    pub fn is_io_256(&self) -> bool {
        *self == CSR::IO_256
    }
    #[doc = "Checks if the value of the field is `IO_1024`"]
    #[inline]
    pub fn is_io_1024(&self) -> bool {
        *self == CSR::IO_1024
    }
    #[doc = "Checks if the value of the field is `EXT_FALLING`"]
    #[inline]
    pub fn is_ext_falling(&self) -> bool {
        *self == CSR::EXT_FALLING
    }
    #[doc = "Checks if the value of the field is `EXT_RISING`"]
    #[inline]
    pub fn is_ext_rising(&self) -> bool {
        *self == CSR::EXT_RISING
    }
}
#[doc = r" Proxy"]
pub struct _FOC_AW<'a> {
    w: &'a mut W,
}
impl<'a> _FOC_AW<'a> {
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
pub struct _FOC_BW<'a> {
    w: &'a mut W,
}
impl<'a> _FOC_BW<'a> {
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
#[doc = r" Proxy"]
pub struct _WGM2W<'a> {
    w: &'a mut W,
}
impl<'a> _WGM2W<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CS`"]
pub enum CSW {
    #[doc = "No clock source (Timer/Counter stopped)"]
    STOPPED,
    #[doc = "clkIO/1 (No prescaling)"]
    IO,
    #[doc = "clkIO/8 (From prescaler)"]
    IO_8,
    #[doc = "clkIO/64 (From prescaler)"]
    IO_64,
    #[doc = "clkIO/256 (From prescaler)"]
    IO_256,
    #[doc = "clkIO/1024 (From prescaler)"]
    IO_1024,
    #[doc = "External clock source on T0 pin. Clock on falling edge."]
    EXT_FALLING,
    #[doc = "External clock source on T0 pin. Clock on rising edge."]
    EXT_RISING,
}
impl CSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSW::STOPPED => 0,
            CSW::IO => 1,
            CSW::IO_8 => 2,
            CSW::IO_64 => 3,
            CSW::IO_256 => 4,
            CSW::IO_1024 => 5,
            CSW::EXT_FALLING => 6,
            CSW::EXT_RISING => 7,
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
    #[doc = "clkIO/1 (No prescaling)"]
    #[inline]
    pub fn io(self) -> &'a mut W {
        self.variant(CSW::IO)
    }
    #[doc = "clkIO/8 (From prescaler)"]
    #[inline]
    pub fn io_8(self) -> &'a mut W {
        self.variant(CSW::IO_8)
    }
    #[doc = "clkIO/64 (From prescaler)"]
    #[inline]
    pub fn io_64(self) -> &'a mut W {
        self.variant(CSW::IO_64)
    }
    #[doc = "clkIO/256 (From prescaler)"]
    #[inline]
    pub fn io_256(self) -> &'a mut W {
        self.variant(CSW::IO_256)
    }
    #[doc = "clkIO/1024 (From prescaler)"]
    #[inline]
    pub fn io_1024(self) -> &'a mut W {
        self.variant(CSW::IO_1024)
    }
    #[doc = "External clock source on T0 pin. Clock on falling edge."]
    #[inline]
    pub fn ext_falling(self) -> &'a mut W {
        self.variant(CSW::EXT_FALLING)
    }
    #[doc = "External clock source on T0 pin. Clock on rising edge."]
    #[inline]
    pub fn ext_rising(self) -> &'a mut W {
        self.variant(CSW::EXT_RISING)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
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
    #[doc = "Bit 3 - Waveform Generation Mode Bit 2 (Enable Top: *OCRA* for PWM modes)"]
    #[inline]
    pub fn wgm2(&self) -> WGM2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        WGM2R { bits }
    }
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline]
    pub fn cs(&self) -> CSR {
        CSR::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bit 7 - Force Output Compare A"]
    #[inline]
    pub fn foc_a(&mut self) -> _FOC_AW {
        _FOC_AW { w: self }
    }
    #[doc = "Bit 6 - Force Output Compare B"]
    #[inline]
    pub fn foc_b(&mut self) -> _FOC_BW {
        _FOC_BW { w: self }
    }
    #[doc = "Bit 3 - Waveform Generation Mode Bit 2 (Enable Top: *OCRA* for PWM modes)"]
    #[inline]
    pub fn wgm2(&mut self) -> _WGM2W {
        _WGM2W { w: self }
    }
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline]
    pub fn cs(&mut self) -> _CSW {
        _CSW { w: self }
    }
}
