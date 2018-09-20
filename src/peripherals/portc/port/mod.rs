#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PORT {
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
#[doc = "Possible values of the field `D7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D7R {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl D7R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            D7R::LOW => false,
            D7R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D7R {
        match value {
            false => D7R::LOW,
            true => D7R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == D7R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == D7R::HIGH
    }
}
#[doc = "Possible values of the field `D6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D6R {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl D6R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            D6R::LOW => false,
            D6R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D6R {
        match value {
            false => D6R::LOW,
            true => D6R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == D6R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == D6R::HIGH
    }
}
#[doc = "Values that can be written to the field `D7`"]
pub enum D7W {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl D7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D7W::LOW => false,
            D7W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D7W<'a> {
    w: &'a mut W,
}
impl<'a> _D7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(D7W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(D7W::HIGH)
    }
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
#[doc = "Values that can be written to the field `D6`"]
pub enum D6W {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl D6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D6W::LOW => false,
            D6W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D6W<'a> {
    w: &'a mut W,
}
impl<'a> _D6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(D6W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(D6W::HIGH)
    }
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
    #[doc = "Bit 7 - Pin 7 Output/Data"]
    #[inline]
    pub fn d7(&self) -> D7R {
        D7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Pin 6 Output/Data"]
    #[inline]
    pub fn d6(&self) -> D6R {
        D6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    #[doc = "Bit 7 - Pin 7 Output/Data"]
    #[inline]
    pub fn d7(&mut self) -> _D7W {
        _D7W { w: self }
    }
    #[doc = "Bit 6 - Pin 6 Output/Data"]
    #[inline]
    pub fn d6(&mut self) -> _D6W {
        _D6W { w: self }
    }
}
