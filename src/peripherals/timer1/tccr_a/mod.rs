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
#[doc = "Possible values of the field `COM_A`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COM_AR {
    #[doc = "Normal port operation, OC1x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC1x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC1x on Compare Match (If PWM is enabled, OC1x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC1x on Compare Match (If PWM is enabled, OC1x is cleared at TOP)"]
    MATCH_SET,
}
impl COM_AR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COM_AR::DISCONNECTED => 0,
            COM_AR::MATCH_TOGGLE => 1,
            COM_AR::MATCH_CLEAR => 2,
            COM_AR::MATCH_SET => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COM_AR {
        match value {
            0 => COM_AR::DISCONNECTED,
            1 => COM_AR::MATCH_TOGGLE,
            2 => COM_AR::MATCH_CLEAR,
            3 => COM_AR::MATCH_SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline]
    pub fn is_disconnected(&self) -> bool {
        *self == COM_AR::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `MATCH_TOGGLE`"]
    #[inline]
    pub fn is_match_toggle(&self) -> bool {
        *self == COM_AR::MATCH_TOGGLE
    }
    #[doc = "Checks if the value of the field is `MATCH_CLEAR`"]
    #[inline]
    pub fn is_match_clear(&self) -> bool {
        *self == COM_AR::MATCH_CLEAR
    }
    #[doc = "Checks if the value of the field is `MATCH_SET`"]
    #[inline]
    pub fn is_match_set(&self) -> bool {
        *self == COM_AR::MATCH_SET
    }
}
#[doc = "Possible values of the field `COM_B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COM_BR {
    #[doc = "Normal port operation, OC1x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC1x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC1x on Compare Match (If PWM is enabled, OC1x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC1x on Compare Match (If PWM is enabled, OC1x is cleared at TOP)"]
    MATCH_SET,
}
impl COM_BR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COM_BR::DISCONNECTED => 0,
            COM_BR::MATCH_TOGGLE => 1,
            COM_BR::MATCH_CLEAR => 2,
            COM_BR::MATCH_SET => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COM_BR {
        match value {
            0 => COM_BR::DISCONNECTED,
            1 => COM_BR::MATCH_TOGGLE,
            2 => COM_BR::MATCH_CLEAR,
            3 => COM_BR::MATCH_SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline]
    pub fn is_disconnected(&self) -> bool {
        *self == COM_BR::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `MATCH_TOGGLE`"]
    #[inline]
    pub fn is_match_toggle(&self) -> bool {
        *self == COM_BR::MATCH_TOGGLE
    }
    #[doc = "Checks if the value of the field is `MATCH_CLEAR`"]
    #[inline]
    pub fn is_match_clear(&self) -> bool {
        *self == COM_BR::MATCH_CLEAR
    }
    #[doc = "Checks if the value of the field is `MATCH_SET`"]
    #[inline]
    pub fn is_match_set(&self) -> bool {
        *self == COM_BR::MATCH_SET
    }
}
#[doc = "Possible values of the field `COM_C`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COM_CR {
    #[doc = "Normal port operation, OC1x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC1x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC1x on Compare Match (If PWM is enabled, OC1x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC1x on Compare Match (If PWM is enabled, OC1x is cleared at TOP)"]
    MATCH_SET,
}
impl COM_CR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COM_CR::DISCONNECTED => 0,
            COM_CR::MATCH_TOGGLE => 1,
            COM_CR::MATCH_CLEAR => 2,
            COM_CR::MATCH_SET => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COM_CR {
        match value {
            0 => COM_CR::DISCONNECTED,
            1 => COM_CR::MATCH_TOGGLE,
            2 => COM_CR::MATCH_CLEAR,
            3 => COM_CR::MATCH_SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline]
    pub fn is_disconnected(&self) -> bool {
        *self == COM_CR::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `MATCH_TOGGLE`"]
    #[inline]
    pub fn is_match_toggle(&self) -> bool {
        *self == COM_CR::MATCH_TOGGLE
    }
    #[doc = "Checks if the value of the field is `MATCH_CLEAR`"]
    #[inline]
    pub fn is_match_clear(&self) -> bool {
        *self == COM_CR::MATCH_CLEAR
    }
    #[doc = "Checks if the value of the field is `MATCH_SET`"]
    #[inline]
    pub fn is_match_set(&self) -> bool {
        *self == COM_CR::MATCH_SET
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
#[doc = "Values that can be written to the field `COM_A`"]
pub enum COM_AW {
    #[doc = "Normal port operation, OC1x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC1x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC1x on Compare Match (If PWM is enabled, OC1x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC1x on Compare Match (If PWM is enabled, OC1x is cleared at TOP)"]
    MATCH_SET,
}
impl COM_AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COM_AW::DISCONNECTED => 0,
            COM_AW::MATCH_TOGGLE => 1,
            COM_AW::MATCH_CLEAR => 2,
            COM_AW::MATCH_SET => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COM_AW<'a> {
    w: &'a mut W,
}
impl<'a> _COM_AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COM_AW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal port operation, OC1x disconnected"]
    #[inline]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(COM_AW::DISCONNECTED)
    }
    #[doc = "Toggle OC1x on Compare Match (Might depend on WGM)"]
    #[inline]
    pub fn match_toggle(self) -> &'a mut W {
        self.variant(COM_AW::MATCH_TOGGLE)
    }
    #[doc = "Clear OC1x on Compare Match (If PWM is enabled, OC1x is set at TOP)"]
    #[inline]
    pub fn match_clear(self) -> &'a mut W {
        self.variant(COM_AW::MATCH_CLEAR)
    }
    #[doc = "Set OC1x on Compare Match (If PWM is enabled, OC1x is cleared at TOP)"]
    #[inline]
    pub fn match_set(self) -> &'a mut W {
        self.variant(COM_AW::MATCH_SET)
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
#[doc = "Values that can be written to the field `COM_B`"]
pub enum COM_BW {
    #[doc = "Normal port operation, OC1x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC1x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC1x on Compare Match (If PWM is enabled, OC1x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC1x on Compare Match (If PWM is enabled, OC1x is cleared at TOP)"]
    MATCH_SET,
}
impl COM_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COM_BW::DISCONNECTED => 0,
            COM_BW::MATCH_TOGGLE => 1,
            COM_BW::MATCH_CLEAR => 2,
            COM_BW::MATCH_SET => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COM_BW<'a> {
    w: &'a mut W,
}
impl<'a> _COM_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COM_BW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal port operation, OC1x disconnected"]
    #[inline]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(COM_BW::DISCONNECTED)
    }
    #[doc = "Toggle OC1x on Compare Match (Might depend on WGM)"]
    #[inline]
    pub fn match_toggle(self) -> &'a mut W {
        self.variant(COM_BW::MATCH_TOGGLE)
    }
    #[doc = "Clear OC1x on Compare Match (If PWM is enabled, OC1x is set at TOP)"]
    #[inline]
    pub fn match_clear(self) -> &'a mut W {
        self.variant(COM_BW::MATCH_CLEAR)
    }
    #[doc = "Set OC1x on Compare Match (If PWM is enabled, OC1x is cleared at TOP)"]
    #[inline]
    pub fn match_set(self) -> &'a mut W {
        self.variant(COM_BW::MATCH_SET)
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
#[doc = "Values that can be written to the field `COM_C`"]
pub enum COM_CW {
    #[doc = "Normal port operation, OC1x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC1x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC1x on Compare Match (If PWM is enabled, OC1x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC1x on Compare Match (If PWM is enabled, OC1x is cleared at TOP)"]
    MATCH_SET,
}
impl COM_CW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COM_CW::DISCONNECTED => 0,
            COM_CW::MATCH_TOGGLE => 1,
            COM_CW::MATCH_CLEAR => 2,
            COM_CW::MATCH_SET => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COM_CW<'a> {
    w: &'a mut W,
}
impl<'a> _COM_CW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COM_CW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal port operation, OC1x disconnected"]
    #[inline]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(COM_CW::DISCONNECTED)
    }
    #[doc = "Toggle OC1x on Compare Match (Might depend on WGM)"]
    #[inline]
    pub fn match_toggle(self) -> &'a mut W {
        self.variant(COM_CW::MATCH_TOGGLE)
    }
    #[doc = "Clear OC1x on Compare Match (If PWM is enabled, OC1x is set at TOP)"]
    #[inline]
    pub fn match_clear(self) -> &'a mut W {
        self.variant(COM_CW::MATCH_CLEAR)
    }
    #[doc = "Set OC1x on Compare Match (If PWM is enabled, OC1x is cleared at TOP)"]
    #[inline]
    pub fn match_set(self) -> &'a mut W {
        self.variant(COM_CW::MATCH_SET)
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
    #[doc = "Bits 6:7 - Compare Output Mode for Channel A"]
    #[inline]
    pub fn com_a(&self) -> COM_AR {
        COM_AR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - Compare Output Mode for Channel B"]
    #[inline]
    pub fn com_b(&self) -> COM_BR {
        COM_BR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 2:3 - Compare Output Mode for Channel C"]
    #[inline]
    pub fn com_c(&self) -> COM_CR {
        COM_CR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 0:1 - Waveform Generation Mode 1:0"]
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
    #[doc = "Bits 6:7 - Compare Output Mode for Channel A"]
    #[inline]
    pub fn com_a(&mut self) -> _COM_AW {
        _COM_AW { w: self }
    }
    #[doc = "Bits 4:5 - Compare Output Mode for Channel B"]
    #[inline]
    pub fn com_b(&mut self) -> _COM_BW {
        _COM_BW { w: self }
    }
    #[doc = "Bits 2:3 - Compare Output Mode for Channel C"]
    #[inline]
    pub fn com_c(&mut self) -> _COM_CW {
        _COM_CW { w: self }
    }
    #[doc = "Bits 0:1 - Waveform Generation Mode 1:0"]
    #[inline]
    pub fn wgm0(&mut self) -> _WGM0W {
        _WGM0W { w: self }
    }
}
