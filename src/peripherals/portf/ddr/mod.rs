#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::DDR {
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
#[doc = "Possible values of the field `DD7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DD7R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DD7R {
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
            DD7R::INPUT => false,
            DD7R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DD7R {
        match value {
            false => DD7R::INPUT,
            true => DD7R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DD7R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DD7R::OUTPUT
    }
}
#[doc = "Possible values of the field `DD6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DD6R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DD6R {
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
            DD6R::INPUT => false,
            DD6R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DD6R {
        match value {
            false => DD6R::INPUT,
            true => DD6R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DD6R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DD6R::OUTPUT
    }
}
#[doc = "Possible values of the field `DD5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DD5R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DD5R {
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
            DD5R::INPUT => false,
            DD5R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DD5R {
        match value {
            false => DD5R::INPUT,
            true => DD5R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DD5R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DD5R::OUTPUT
    }
}
#[doc = "Possible values of the field `DD4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DD4R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DD4R {
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
            DD4R::INPUT => false,
            DD4R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DD4R {
        match value {
            false => DD4R::INPUT,
            true => DD4R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DD4R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DD4R::OUTPUT
    }
}
#[doc = "Possible values of the field `DD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DD1R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DD1R {
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
            DD1R::INPUT => false,
            DD1R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DD1R {
        match value {
            false => DD1R::INPUT,
            true => DD1R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DD1R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DD1R::OUTPUT
    }
}
#[doc = "Possible values of the field `DD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DD0R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DD0R {
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
            DD0R::INPUT => false,
            DD0R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DD0R {
        match value {
            false => DD0R::INPUT,
            true => DD0R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DD0R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DD0R::OUTPUT
    }
}
#[doc = "Values that can be written to the field `DD7`"]
pub enum DD7W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DD7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DD7W::INPUT => false,
            DD7W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DD7W<'a> {
    w: &'a mut W,
}
impl<'a> _DD7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DD7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DD7W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DD7W::OUTPUT)
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
#[doc = "Values that can be written to the field `DD6`"]
pub enum DD6W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DD6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DD6W::INPUT => false,
            DD6W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DD6W<'a> {
    w: &'a mut W,
}
impl<'a> _DD6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DD6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DD6W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DD6W::OUTPUT)
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
#[doc = "Values that can be written to the field `DD5`"]
pub enum DD5W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DD5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DD5W::INPUT => false,
            DD5W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DD5W<'a> {
    w: &'a mut W,
}
impl<'a> _DD5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DD5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DD5W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DD5W::OUTPUT)
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
#[doc = "Values that can be written to the field `DD4`"]
pub enum DD4W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DD4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DD4W::INPUT => false,
            DD4W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DD4W<'a> {
    w: &'a mut W,
}
impl<'a> _DD4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DD4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DD4W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DD4W::OUTPUT)
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
#[doc = "Values that can be written to the field `DD1`"]
pub enum DD1W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DD1W::INPUT => false,
            DD1W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DD1W<'a> {
    w: &'a mut W,
}
impl<'a> _DD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DD1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DD1W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DD1W::OUTPUT)
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
#[doc = "Values that can be written to the field `DD0`"]
pub enum DD0W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DD0W::INPUT => false,
            DD0W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DD0W<'a> {
    w: &'a mut W,
}
impl<'a> _DD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DD0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DD0W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DD0W::OUTPUT)
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
    #[doc = "Bit 7 - Pin 7 Direction"]
    #[inline]
    pub fn dd7(&self) -> DD7R {
        DD7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Pin 6 Direction"]
    #[inline]
    pub fn dd6(&self) -> DD6R {
        DD6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Pin 5 Direction"]
    #[inline]
    pub fn dd5(&self) -> DD5R {
        DD5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Pin 4 Direction"]
    #[inline]
    pub fn dd4(&self) -> DD4R {
        DD4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Pin 1 Direction"]
    #[inline]
    pub fn dd1(&self) -> DD1R {
        DD1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 0 - Pin 0 Direction"]
    #[inline]
    pub fn dd0(&self) -> DD0R {
        DD0R::_from({
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
    #[doc = "Bit 7 - Pin 7 Direction"]
    #[inline]
    pub fn dd7(&mut self) -> _DD7W {
        _DD7W { w: self }
    }
    #[doc = "Bit 6 - Pin 6 Direction"]
    #[inline]
    pub fn dd6(&mut self) -> _DD6W {
        _DD6W { w: self }
    }
    #[doc = "Bit 5 - Pin 5 Direction"]
    #[inline]
    pub fn dd5(&mut self) -> _DD5W {
        _DD5W { w: self }
    }
    #[doc = "Bit 4 - Pin 4 Direction"]
    #[inline]
    pub fn dd4(&mut self) -> _DD4W {
        _DD4W { w: self }
    }
    #[doc = "Bit 1 - Pin 1 Direction"]
    #[inline]
    pub fn dd1(&mut self) -> _DD1W {
        _DD1W { w: self }
    }
    #[doc = "Bit 0 - Pin 0 Direction"]
    #[inline]
    pub fn dd0(&mut self) -> _DD0W {
        _DD0W { w: self }
    }
}
