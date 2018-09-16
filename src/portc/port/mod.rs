#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PORT {
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
}
#[doc = "Values that can be written to the field `P0`"]
pub enum P0W {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl P0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P0W::LOW => false,
            P0W::HIGH => true,
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
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(P0W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(P0W::HIGH)
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
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl P1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1W::LOW => false,
            P1W::HIGH => true,
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
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(P1W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(P1W::HIGH)
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
#[doc = "Values that can be written to the field `P2`"]
pub enum P2W {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl P2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2W::LOW => false,
            P2W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2W<'a> {
    w: &'a mut W,
}
impl<'a> _P2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(P2W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(P2W::HIGH)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P3`"]
pub enum P3W {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl P3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P3W::LOW => false,
            P3W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P3W<'a> {
    w: &'a mut W,
}
impl<'a> _P3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(P3W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(P3W::HIGH)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P4`"]
pub enum P4W {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl P4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P4W::LOW => false,
            P4W::HIGH => true,
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
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(P4W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(P4W::HIGH)
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
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl P5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P5W::LOW => false,
            P5W::HIGH => true,
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
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(P5W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(P5W::HIGH)
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
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl P6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P6W::LOW => false,
            P6W::HIGH => true,
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
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(P6W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(P6W::HIGH)
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
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl P7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P7W::LOW => false,
            P7W::HIGH => true,
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
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(P7W::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(P7W::HIGH)
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
    #[doc = "Bit 0 - Pin 0 Output"]
    #[inline]
    pub fn p0(&mut self) -> _P0W {
        _P0W { w: self }
    }
    #[doc = "Bit 1 - Pin 1 Output"]
    #[inline]
    pub fn p1(&mut self) -> _P1W {
        _P1W { w: self }
    }
    #[doc = "Bit 2 - Pin 2 Output"]
    #[inline]
    pub fn p2(&mut self) -> _P2W {
        _P2W { w: self }
    }
    #[doc = "Bit 3 - Pin 3 Output"]
    #[inline]
    pub fn p3(&mut self) -> _P3W {
        _P3W { w: self }
    }
    #[doc = "Bit 4 - Pin 4 Output"]
    #[inline]
    pub fn p4(&mut self) -> _P4W {
        _P4W { w: self }
    }
    #[doc = "Bit 5 - Pin 5 Output"]
    #[inline]
    pub fn p5(&mut self) -> _P5W {
        _P5W { w: self }
    }
    #[doc = "Bit 6 - Pin 6 Output"]
    #[inline]
    pub fn p6(&mut self) -> _P6W {
        _P6W { w: self }
    }
    #[doc = "Bit 7 - Pin 7 Output"]
    #[inline]
    pub fn p7(&mut self) -> _P7W {
        _P7W { w: self }
    }
}
