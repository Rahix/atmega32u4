#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::TCCR_A {
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
pub struct COM_AR {
    bits: u8,
}
impl COM_AR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COM_BR {
    bits: u8,
}
impl COM_BR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WGM0R {
    bits: u8,
}
impl WGM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _COM_AW<'a> {
    w: &'a mut W,
}
impl<'a> _COM_AW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COM_BW<'a> {
    w: &'a mut W,
}
impl<'a> _COM_BW<'a> {
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
#[doc = r" Proxy"]
pub struct _WGM0W<'a> {
    w: &'a mut W,
}
impl<'a> _WGM0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 6:7 - Compare Match Output A Mode"]
    #[inline]
    pub fn com_a(&self) -> COM_AR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        COM_AR { bits }
    }
    #[doc = "Bits 4:5 - Compare Match Output B Mode"]
    #[inline]
    pub fn com_b(&self) -> COM_BR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        COM_BR { bits }
    }
    #[doc = "Bits 0:1 - Waveform Generation Mode Bits 1:0"]
    #[inline]
    pub fn wgm0(&self) -> WGM0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        WGM0R { bits }
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
    #[doc = "Bits 6:7 - Compare Match Output A Mode"]
    #[inline]
    pub fn com_a(&mut self) -> _COM_AW {
        _COM_AW { w: self }
    }
    #[doc = "Bits 4:5 - Compare Match Output B Mode"]
    #[inline]
    pub fn com_b(&mut self) -> _COM_BW {
        _COM_BW { w: self }
    }
    #[doc = "Bits 0:1 - Waveform Generation Mode Bits 1:0"]
    #[inline]
    pub fn wgm0(&mut self) -> _WGM0W {
        _WGM0W { w: self }
    }
}
