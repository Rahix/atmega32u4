#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::TCCR_C {
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
#[doc = "Possible values of the field `COM_A`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COM_AR {
    #[doc = "Normal port operation, OC4x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC4x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC4x on Compare Match (If PWM is enabled, OC4x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC4x on Compare Match (If PWM is enabled, OC4x is cleared at TOP)"]
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
    #[doc = "Normal port operation, OC4x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC4x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC4x on Compare Match (If PWM is enabled, OC4x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC4x on Compare Match (If PWM is enabled, OC4x is cleared at TOP)"]
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
#[doc = "Possible values of the field `COM_D`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COM_DR {
    #[doc = "Normal port operation, OC4x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC4x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC4x on Compare Match (If PWM is enabled, OC4x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC4x on Compare Match (If PWM is enabled, OC4x is cleared at TOP)"]
    MATCH_SET,
}
impl COM_DR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COM_DR::DISCONNECTED => 0,
            COM_DR::MATCH_TOGGLE => 1,
            COM_DR::MATCH_CLEAR => 2,
            COM_DR::MATCH_SET => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COM_DR {
        match value {
            0 => COM_DR::DISCONNECTED,
            1 => COM_DR::MATCH_TOGGLE,
            2 => COM_DR::MATCH_CLEAR,
            3 => COM_DR::MATCH_SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline]
    pub fn is_disconnected(&self) -> bool {
        *self == COM_DR::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `MATCH_TOGGLE`"]
    #[inline]
    pub fn is_match_toggle(&self) -> bool {
        *self == COM_DR::MATCH_TOGGLE
    }
    #[doc = "Checks if the value of the field is `MATCH_CLEAR`"]
    #[inline]
    pub fn is_match_clear(&self) -> bool {
        *self == COM_DR::MATCH_CLEAR
    }
    #[doc = "Checks if the value of the field is `MATCH_SET`"]
    #[inline]
    pub fn is_match_set(&self) -> bool {
        *self == COM_DR::MATCH_SET
    }
}
#[doc = r" Value of the field"]
pub struct PWM_DR {
    bits: bool,
}
impl PWM_DR {
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
#[doc = "Values that can be written to the field `COM_A`"]
pub enum COM_AW {
    #[doc = "Normal port operation, OC4x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC4x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC4x on Compare Match (If PWM is enabled, OC4x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC4x on Compare Match (If PWM is enabled, OC4x is cleared at TOP)"]
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
    #[doc = "Normal port operation, OC4x disconnected"]
    #[inline]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(COM_AW::DISCONNECTED)
    }
    #[doc = "Toggle OC4x on Compare Match (Might depend on WGM)"]
    #[inline]
    pub fn match_toggle(self) -> &'a mut W {
        self.variant(COM_AW::MATCH_TOGGLE)
    }
    #[doc = "Clear OC4x on Compare Match (If PWM is enabled, OC4x is set at TOP)"]
    #[inline]
    pub fn match_clear(self) -> &'a mut W {
        self.variant(COM_AW::MATCH_CLEAR)
    }
    #[doc = "Set OC4x on Compare Match (If PWM is enabled, OC4x is cleared at TOP)"]
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
    #[doc = "Normal port operation, OC4x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC4x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC4x on Compare Match (If PWM is enabled, OC4x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC4x on Compare Match (If PWM is enabled, OC4x is cleared at TOP)"]
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
    #[doc = "Normal port operation, OC4x disconnected"]
    #[inline]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(COM_BW::DISCONNECTED)
    }
    #[doc = "Toggle OC4x on Compare Match (Might depend on WGM)"]
    #[inline]
    pub fn match_toggle(self) -> &'a mut W {
        self.variant(COM_BW::MATCH_TOGGLE)
    }
    #[doc = "Clear OC4x on Compare Match (If PWM is enabled, OC4x is set at TOP)"]
    #[inline]
    pub fn match_clear(self) -> &'a mut W {
        self.variant(COM_BW::MATCH_CLEAR)
    }
    #[doc = "Set OC4x on Compare Match (If PWM is enabled, OC4x is cleared at TOP)"]
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
#[doc = "Values that can be written to the field `COM_D`"]
pub enum COM_DW {
    #[doc = "Normal port operation, OC4x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC4x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC4x on Compare Match (If PWM is enabled, OC4x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC4x on Compare Match (If PWM is enabled, OC4x is cleared at TOP)"]
    MATCH_SET,
}
impl COM_DW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COM_DW::DISCONNECTED => 0,
            COM_DW::MATCH_TOGGLE => 1,
            COM_DW::MATCH_CLEAR => 2,
            COM_DW::MATCH_SET => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COM_DW<'a> {
    w: &'a mut W,
}
impl<'a> _COM_DW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COM_DW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal port operation, OC4x disconnected"]
    #[inline]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(COM_DW::DISCONNECTED)
    }
    #[doc = "Toggle OC4x on Compare Match (Might depend on WGM)"]
    #[inline]
    pub fn match_toggle(self) -> &'a mut W {
        self.variant(COM_DW::MATCH_TOGGLE)
    }
    #[doc = "Clear OC4x on Compare Match (If PWM is enabled, OC4x is set at TOP)"]
    #[inline]
    pub fn match_clear(self) -> &'a mut W {
        self.variant(COM_DW::MATCH_CLEAR)
    }
    #[doc = "Set OC4x on Compare Match (If PWM is enabled, OC4x is cleared at TOP)"]
    #[inline]
    pub fn match_set(self) -> &'a mut W {
        self.variant(COM_DW::MATCH_SET)
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
pub struct _PWM_DW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_DW<'a> {
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
#[doc = r" Proxy"]
pub struct _FOC_DW<'a> {
    w: &'a mut W,
}
impl<'a> _FOC_DW<'a> {
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
    #[doc = "Bits 2:3 - Compare Output Mode for Channel D"]
    #[inline]
    pub fn com_d(&self) -> COM_DR {
        COM_DR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 0 - Pulse Width Modulator D"]
    #[inline]
    pub fn pwm_d(&self) -> PWM_DR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        PWM_DR { bits }
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
    #[doc = "Bits 2:3 - Compare Output Mode for Channel D"]
    #[inline]
    pub fn com_d(&mut self) -> _COM_DW {
        _COM_DW { w: self }
    }
    #[doc = "Bit 0 - Pulse Width Modulator D"]
    #[inline]
    pub fn pwm_d(&mut self) -> _PWM_DW {
        _PWM_DW { w: self }
    }
    #[doc = "Bit 1 - Force Output Compare Match D"]
    #[inline]
    pub fn foc_d(&mut self) -> _FOC_DW {
        _FOC_DW { w: self }
    }
}
