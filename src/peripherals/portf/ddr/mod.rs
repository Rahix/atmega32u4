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
#[doc = "Possible values of the field `P0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl P0R {
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
            P0R::INPUT => false,
            P0R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P0R {
        match value {
            false => P0R::INPUT,
            true => P0R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == P0R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == P0R::OUTPUT
    }
}
#[doc = "Possible values of the field `P1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl P1R {
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
            P1R::INPUT => false,
            P1R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1R {
        match value {
            false => P1R::INPUT,
            true => P1R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == P1R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == P1R::OUTPUT
    }
}
#[doc = "Possible values of the field `P4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl P4R {
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
            P4R::INPUT => false,
            P4R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P4R {
        match value {
            false => P4R::INPUT,
            true => P4R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == P4R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == P4R::OUTPUT
    }
}
#[doc = "Possible values of the field `P5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P5R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl P5R {
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
            P5R::INPUT => false,
            P5R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P5R {
        match value {
            false => P5R::INPUT,
            true => P5R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == P5R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == P5R::OUTPUT
    }
}
#[doc = "Possible values of the field `P6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P6R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl P6R {
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
            P6R::INPUT => false,
            P6R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P6R {
        match value {
            false => P6R::INPUT,
            true => P6R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == P6R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == P6R::OUTPUT
    }
}
#[doc = "Possible values of the field `P7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P7R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl P7R {
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
            P7R::INPUT => false,
            P7R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P7R {
        match value {
            false => P7R::INPUT,
            true => P7R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == P7R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == P7R::OUTPUT
    }
}
#[doc = "Values that can be written to the field `P0`"]
pub enum P0W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl P0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P0W::INPUT => false,
            P0W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0W<'a> {
    w: &'a mut W,
}
impl<'a> _P0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(P0W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(P0W::OUTPUT)
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
#[doc = "Values that can be written to the field `P1`"]
pub enum P1W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl P1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1W::INPUT => false,
            P1W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1W<'a> {
    w: &'a mut W,
}
impl<'a> _P1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(P1W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(P1W::OUTPUT)
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
#[doc = "Values that can be written to the field `P4`"]
pub enum P4W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl P4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P4W::INPUT => false,
            P4W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P4W<'a> {
    w: &'a mut W,
}
impl<'a> _P4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(P4W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(P4W::OUTPUT)
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
#[doc = "Values that can be written to the field `P5`"]
pub enum P5W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl P5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P5W::INPUT => false,
            P5W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P5W<'a> {
    w: &'a mut W,
}
impl<'a> _P5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(P5W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(P5W::OUTPUT)
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
#[doc = "Values that can be written to the field `P6`"]
pub enum P6W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl P6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P6W::INPUT => false,
            P6W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P6W<'a> {
    w: &'a mut W,
}
impl<'a> _P6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(P6W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(P6W::OUTPUT)
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
#[doc = "Values that can be written to the field `P7`"]
pub enum P7W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl P7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P7W::INPUT => false,
            P7W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P7W<'a> {
    w: &'a mut W,
}
impl<'a> _P7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(P7W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(P7W::OUTPUT)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Pin 0 Direction"]
    #[inline]
    pub fn p0(&self) -> P0R {
        P0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Pin 1 Direction"]
    #[inline]
    pub fn p1(&self) -> P1R {
        P1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Pin 4 Direction"]
    #[inline]
    pub fn p4(&self) -> P4R {
        P4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Pin 5 Direction"]
    #[inline]
    pub fn p5(&self) -> P5R {
        P5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Pin 6 Direction"]
    #[inline]
    pub fn p6(&self) -> P6R {
        P6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Pin 7 Direction"]
    #[inline]
    pub fn p7(&self) -> P7R {
        P7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Pin 0 Direction"]
    #[inline]
    pub fn p0(&mut self) -> _P0W {
        _P0W { w: self }
    }
    #[doc = "Bit 1 - Pin 1 Direction"]
    #[inline]
    pub fn p1(&mut self) -> _P1W {
        _P1W { w: self }
    }
    #[doc = "Bit 4 - Pin 4 Direction"]
    #[inline]
    pub fn p4(&mut self) -> _P4W {
        _P4W { w: self }
    }
    #[doc = "Bit 5 - Pin 5 Direction"]
    #[inline]
    pub fn p5(&mut self) -> _P5W {
        _P5W { w: self }
    }
    #[doc = "Bit 6 - Pin 6 Direction"]
    #[inline]
    pub fn p6(&mut self) -> _P6W {
        _P6W { w: self }
    }
    #[doc = "Bit 7 - Pin 7 Direction"]
    #[inline]
    pub fn p7(&mut self) -> _P7W {
        _P7W { w: self }
    }
}
