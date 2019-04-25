#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::EICRA {
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
        R {
            bits: self.register.get(),
        }
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
#[doc = "Possible values of the field `ISC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISC3R {
    #[doc = "The low level of INTn generates an interrupt request"]
    LVL_LOW,
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    EDGE_BOTH,
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    EDGE_FALLING,
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    EDGE_RISING,
}
impl ISC3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ISC3R::LVL_LOW => 0,
            ISC3R::EDGE_BOTH => 1,
            ISC3R::EDGE_FALLING => 2,
            ISC3R::EDGE_RISING => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ISC3R {
        match value {
            0 => ISC3R::LVL_LOW,
            1 => ISC3R::EDGE_BOTH,
            2 => ISC3R::EDGE_FALLING,
            3 => ISC3R::EDGE_RISING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LVL_LOW`"]
    #[inline]
    pub fn is_lvl_low(&self) -> bool {
        *self == ISC3R::LVL_LOW
    }
    #[doc = "Checks if the value of the field is `EDGE_BOTH`"]
    #[inline]
    pub fn is_edge_both(&self) -> bool {
        *self == ISC3R::EDGE_BOTH
    }
    #[doc = "Checks if the value of the field is `EDGE_FALLING`"]
    #[inline]
    pub fn is_edge_falling(&self) -> bool {
        *self == ISC3R::EDGE_FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE_RISING`"]
    #[inline]
    pub fn is_edge_rising(&self) -> bool {
        *self == ISC3R::EDGE_RISING
    }
}
#[doc = "Possible values of the field `ISC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISC2R {
    #[doc = "The low level of INTn generates an interrupt request"]
    LVL_LOW,
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    EDGE_BOTH,
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    EDGE_FALLING,
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    EDGE_RISING,
}
impl ISC2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ISC2R::LVL_LOW => 0,
            ISC2R::EDGE_BOTH => 1,
            ISC2R::EDGE_FALLING => 2,
            ISC2R::EDGE_RISING => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ISC2R {
        match value {
            0 => ISC2R::LVL_LOW,
            1 => ISC2R::EDGE_BOTH,
            2 => ISC2R::EDGE_FALLING,
            3 => ISC2R::EDGE_RISING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LVL_LOW`"]
    #[inline]
    pub fn is_lvl_low(&self) -> bool {
        *self == ISC2R::LVL_LOW
    }
    #[doc = "Checks if the value of the field is `EDGE_BOTH`"]
    #[inline]
    pub fn is_edge_both(&self) -> bool {
        *self == ISC2R::EDGE_BOTH
    }
    #[doc = "Checks if the value of the field is `EDGE_FALLING`"]
    #[inline]
    pub fn is_edge_falling(&self) -> bool {
        *self == ISC2R::EDGE_FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE_RISING`"]
    #[inline]
    pub fn is_edge_rising(&self) -> bool {
        *self == ISC2R::EDGE_RISING
    }
}
#[doc = "Possible values of the field `ISC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISC1R {
    #[doc = "The low level of INTn generates an interrupt request"]
    LVL_LOW,
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    EDGE_BOTH,
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    EDGE_FALLING,
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    EDGE_RISING,
}
impl ISC1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ISC1R::LVL_LOW => 0,
            ISC1R::EDGE_BOTH => 1,
            ISC1R::EDGE_FALLING => 2,
            ISC1R::EDGE_RISING => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ISC1R {
        match value {
            0 => ISC1R::LVL_LOW,
            1 => ISC1R::EDGE_BOTH,
            2 => ISC1R::EDGE_FALLING,
            3 => ISC1R::EDGE_RISING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LVL_LOW`"]
    #[inline]
    pub fn is_lvl_low(&self) -> bool {
        *self == ISC1R::LVL_LOW
    }
    #[doc = "Checks if the value of the field is `EDGE_BOTH`"]
    #[inline]
    pub fn is_edge_both(&self) -> bool {
        *self == ISC1R::EDGE_BOTH
    }
    #[doc = "Checks if the value of the field is `EDGE_FALLING`"]
    #[inline]
    pub fn is_edge_falling(&self) -> bool {
        *self == ISC1R::EDGE_FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE_RISING`"]
    #[inline]
    pub fn is_edge_rising(&self) -> bool {
        *self == ISC1R::EDGE_RISING
    }
}
#[doc = "Possible values of the field `ISC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISC0R {
    #[doc = "The low level of INTn generates an interrupt request"]
    LVL_LOW,
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    EDGE_BOTH,
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    EDGE_FALLING,
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    EDGE_RISING,
}
impl ISC0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ISC0R::LVL_LOW => 0,
            ISC0R::EDGE_BOTH => 1,
            ISC0R::EDGE_FALLING => 2,
            ISC0R::EDGE_RISING => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ISC0R {
        match value {
            0 => ISC0R::LVL_LOW,
            1 => ISC0R::EDGE_BOTH,
            2 => ISC0R::EDGE_FALLING,
            3 => ISC0R::EDGE_RISING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LVL_LOW`"]
    #[inline]
    pub fn is_lvl_low(&self) -> bool {
        *self == ISC0R::LVL_LOW
    }
    #[doc = "Checks if the value of the field is `EDGE_BOTH`"]
    #[inline]
    pub fn is_edge_both(&self) -> bool {
        *self == ISC0R::EDGE_BOTH
    }
    #[doc = "Checks if the value of the field is `EDGE_FALLING`"]
    #[inline]
    pub fn is_edge_falling(&self) -> bool {
        *self == ISC0R::EDGE_FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE_RISING`"]
    #[inline]
    pub fn is_edge_rising(&self) -> bool {
        *self == ISC0R::EDGE_RISING
    }
}
#[doc = "Values that can be written to the field `ISC3`"]
pub enum ISC3W {
    #[doc = "The low level of INTn generates an interrupt request"]
    LVL_LOW,
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    EDGE_BOTH,
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    EDGE_FALLING,
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    EDGE_RISING,
}
impl ISC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ISC3W::LVL_LOW => 0,
            ISC3W::EDGE_BOTH => 1,
            ISC3W::EDGE_FALLING => 2,
            ISC3W::EDGE_RISING => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISC3W<'a> {
    w: &'a mut W,
}
impl<'a> _ISC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISC3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The low level of INTn generates an interrupt request"]
    #[inline]
    pub fn lvl_low(self) -> &'a mut W {
        self.variant(ISC3W::LVL_LOW)
    }
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_both(self) -> &'a mut W {
        self.variant(ISC3W::EDGE_BOTH)
    }
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_falling(self) -> &'a mut W {
        self.variant(ISC3W::EDGE_FALLING)
    }
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_rising(self) -> &'a mut W {
        self.variant(ISC3W::EDGE_RISING)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ISC2`"]
pub enum ISC2W {
    #[doc = "The low level of INTn generates an interrupt request"]
    LVL_LOW,
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    EDGE_BOTH,
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    EDGE_FALLING,
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    EDGE_RISING,
}
impl ISC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ISC2W::LVL_LOW => 0,
            ISC2W::EDGE_BOTH => 1,
            ISC2W::EDGE_FALLING => 2,
            ISC2W::EDGE_RISING => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISC2W<'a> {
    w: &'a mut W,
}
impl<'a> _ISC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISC2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The low level of INTn generates an interrupt request"]
    #[inline]
    pub fn lvl_low(self) -> &'a mut W {
        self.variant(ISC2W::LVL_LOW)
    }
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_both(self) -> &'a mut W {
        self.variant(ISC2W::EDGE_BOTH)
    }
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_falling(self) -> &'a mut W {
        self.variant(ISC2W::EDGE_FALLING)
    }
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_rising(self) -> &'a mut W {
        self.variant(ISC2W::EDGE_RISING)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ISC1`"]
pub enum ISC1W {
    #[doc = "The low level of INTn generates an interrupt request"]
    LVL_LOW,
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    EDGE_BOTH,
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    EDGE_FALLING,
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    EDGE_RISING,
}
impl ISC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ISC1W::LVL_LOW => 0,
            ISC1W::EDGE_BOTH => 1,
            ISC1W::EDGE_FALLING => 2,
            ISC1W::EDGE_RISING => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISC1W<'a> {
    w: &'a mut W,
}
impl<'a> _ISC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISC1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The low level of INTn generates an interrupt request"]
    #[inline]
    pub fn lvl_low(self) -> &'a mut W {
        self.variant(ISC1W::LVL_LOW)
    }
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_both(self) -> &'a mut W {
        self.variant(ISC1W::EDGE_BOTH)
    }
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_falling(self) -> &'a mut W {
        self.variant(ISC1W::EDGE_FALLING)
    }
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_rising(self) -> &'a mut W {
        self.variant(ISC1W::EDGE_RISING)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ISC0`"]
pub enum ISC0W {
    #[doc = "The low level of INTn generates an interrupt request"]
    LVL_LOW,
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    EDGE_BOTH,
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    EDGE_FALLING,
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    EDGE_RISING,
}
impl ISC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ISC0W::LVL_LOW => 0,
            ISC0W::EDGE_BOTH => 1,
            ISC0W::EDGE_FALLING => 2,
            ISC0W::EDGE_RISING => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISC0W<'a> {
    w: &'a mut W,
}
impl<'a> _ISC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISC0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The low level of INTn generates an interrupt request"]
    #[inline]
    pub fn lvl_low(self) -> &'a mut W {
        self.variant(ISC0W::LVL_LOW)
    }
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_both(self) -> &'a mut W {
        self.variant(ISC0W::EDGE_BOTH)
    }
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_falling(self) -> &'a mut W {
        self.variant(ISC0W::EDGE_FALLING)
    }
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_rising(self) -> &'a mut W {
        self.variant(ISC0W::EDGE_RISING)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 6:7 - External Interrupt 3 Sense Control Bits"]
    #[inline]
    pub fn isc3(&self) -> ISC3R {
        ISC3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - External Interrupt 2 Sense Control Bits"]
    #[inline]
    pub fn isc2(&self) -> ISC2R {
        ISC2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 2:3 - External Interrupt 1 Sense Control Bits"]
    #[inline]
    pub fn isc1(&self) -> ISC1R {
        ISC1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 0:1 - External Interrupt 0 Sense Control Bits"]
    #[inline]
    pub fn isc0(&self) -> ISC0R {
        ISC0R::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bits 6:7 - External Interrupt 3 Sense Control Bits"]
    #[inline]
    pub fn isc3(&mut self) -> _ISC3W {
        _ISC3W { w: self }
    }
    #[doc = "Bits 4:5 - External Interrupt 2 Sense Control Bits"]
    #[inline]
    pub fn isc2(&mut self) -> _ISC2W {
        _ISC2W { w: self }
    }
    #[doc = "Bits 2:3 - External Interrupt 1 Sense Control Bits"]
    #[inline]
    pub fn isc1(&mut self) -> _ISC1W {
        _ISC1W { w: self }
    }
    #[doc = "Bits 0:1 - External Interrupt 0 Sense Control Bits"]
    #[inline]
    pub fn isc0(&mut self) -> _ISC0W {
        _ISC0W { w: self }
    }
}
