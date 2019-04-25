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
#[doc = "Possible values of the field `D5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D5R {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl D5R {
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
            D5R::LOW => false,
            D5R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D5R {
        match value {
            false => D5R::LOW,
            true => D5R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == D5R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == D5R::HIGH
    }
}
#[doc = "Possible values of the field `D4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D4R {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl D4R {
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
            D4R::LOW => false,
            D4R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D4R {
        match value {
            false => D4R::LOW,
            true => D4R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == D4R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == D4R::HIGH
    }
}
#[doc = "Possible values of the field `D1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D1R {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl D1R {
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
            D1R::LOW => false,
            D1R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D1R {
        match value {
            false => D1R::LOW,
            true => D1R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == D1R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == D1R::HIGH
    }
}
#[doc = "Possible values of the field `D0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D0R {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl D0R {
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
            D0R::LOW => false,
            D0R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D0R {
        match value {
            false => D0R::LOW,
            true => D0R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == D0R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == D0R::HIGH
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
#[doc = "Values that can be written to the field `D5`"]
pub enum D5W {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl D5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D5W::LOW => false,
            D5W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D5W<'a> {
    w: &'a mut W,
}
impl<'a> _D5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(D5W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(D5W::HIGH)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `D4`"]
pub enum D4W {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl D4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D4W::LOW => false,
            D4W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D4W<'a> {
    w: &'a mut W,
}
impl<'a> _D4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(D4W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(D4W::HIGH)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `D1`"]
pub enum D1W {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl D1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D1W::LOW => false,
            D1W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D1W<'a> {
    w: &'a mut W,
}
impl<'a> _D1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(D1W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(D1W::HIGH)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `D0`"]
pub enum D0W {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl D0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D0W::LOW => false,
            D0W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D0W<'a> {
    w: &'a mut W,
}
impl<'a> _D0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(D0W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(D0W::HIGH)
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
    #[doc = "Bit 5 - Pin 5 Output/Data"]
    #[inline]
    pub fn d5(&self) -> D5R {
        D5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Pin 4 Output/Data"]
    #[inline]
    pub fn d4(&self) -> D4R {
        D4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Pin 1 Output/Data"]
    #[inline]
    pub fn d1(&self) -> D1R {
        D1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 0 - Pin 0 Output/Data"]
    #[inline]
    pub fn d0(&self) -> D0R {
        D0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 5 - Pin 5 Output/Data"]
    #[inline]
    pub fn d5(&mut self) -> _D5W {
        _D5W { w: self }
    }
    #[doc = "Bit 4 - Pin 4 Output/Data"]
    #[inline]
    pub fn d4(&mut self) -> _D4W {
        _D4W { w: self }
    }
    #[doc = "Bit 1 - Pin 1 Output/Data"]
    #[inline]
    pub fn d1(&mut self) -> _D1W {
        _D1W { w: self }
    }
    #[doc = "Bit 0 - Pin 0 Output/Data"]
    #[inline]
    pub fn d0(&mut self) -> _D0W {
        _D0W { w: self }
    }
}
