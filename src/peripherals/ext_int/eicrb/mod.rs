#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::EICRB {
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
#[doc = "Possible values of the field `ISC6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISC6R {
    #[doc = "The low level of INTn generates an interrupt request"]
    LVL_LOW,
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    EDGE_BOTH,
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    EDGE_FALLING,
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    EDGE_RISING,
}
impl ISC6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ISC6R::LVL_LOW => 0,
            ISC6R::EDGE_BOTH => 1,
            ISC6R::EDGE_FALLING => 2,
            ISC6R::EDGE_RISING => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ISC6R {
        match value {
            0 => ISC6R::LVL_LOW,
            1 => ISC6R::EDGE_BOTH,
            2 => ISC6R::EDGE_FALLING,
            3 => ISC6R::EDGE_RISING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LVL_LOW`"]
    #[inline]
    pub fn is_lvl_low(&self) -> bool {
        *self == ISC6R::LVL_LOW
    }
    #[doc = "Checks if the value of the field is `EDGE_BOTH`"]
    #[inline]
    pub fn is_edge_both(&self) -> bool {
        *self == ISC6R::EDGE_BOTH
    }
    #[doc = "Checks if the value of the field is `EDGE_FALLING`"]
    #[inline]
    pub fn is_edge_falling(&self) -> bool {
        *self == ISC6R::EDGE_FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE_RISING`"]
    #[inline]
    pub fn is_edge_rising(&self) -> bool {
        *self == ISC6R::EDGE_RISING
    }
}
#[doc = "Values that can be written to the field `ISC6`"]
pub enum ISC6W {
    #[doc = "The low level of INTn generates an interrupt request"]
    LVL_LOW,
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    EDGE_BOTH,
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    EDGE_FALLING,
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    EDGE_RISING,
}
impl ISC6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ISC6W::LVL_LOW => 0,
            ISC6W::EDGE_BOTH => 1,
            ISC6W::EDGE_FALLING => 2,
            ISC6W::EDGE_RISING => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISC6W<'a> {
    w: &'a mut W,
}
impl<'a> _ISC6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISC6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The low level of INTn generates an interrupt request"]
    #[inline]
    pub fn lvl_low(self) -> &'a mut W {
        self.variant(ISC6W::LVL_LOW)
    }
    #[doc = "Any edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_both(self) -> &'a mut W {
        self.variant(ISC6W::EDGE_BOTH)
    }
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_falling(self) -> &'a mut W {
        self.variant(ISC6W::EDGE_FALLING)
    }
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request"]
    #[inline]
    pub fn edge_rising(self) -> &'a mut W {
        self.variant(ISC6W::EDGE_RISING)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 4:5 - External Interrupt 6 Sense Control Bits"]
    #[inline]
    pub fn isc6(&self) -> ISC6R {
        ISC6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 4:5 - External Interrupt 6 Sense Control Bits"]
    #[inline]
    pub fn isc6(&mut self) -> _ISC6W {
        _ISC6W { w: self }
    }
}
