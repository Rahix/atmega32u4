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
    #[doc = "Normal port operation, OC0x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC0x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC0x on Compare Match (If PWM is enabled, OC0x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC0x on Compare Match (If PWM is enabled, OC0x is cleared at TOP)"]
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
    #[doc = "Normal port operation, OC0x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC0x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC0x on Compare Match (If PWM is enabled, OC0x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC0x on Compare Match (If PWM is enabled, OC0x is cleared at TOP)"]
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
#[doc = "Possible values of the field `WGM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WGM0R {
    #[doc = "Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*"]
    NORMAL_TOP,
    #[doc = "Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*"]
    PWM_PHASE,
    #[doc = "CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*"]
    CTC,
    #[doc = "Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*"]
    PWM_FAST,
}
impl WGM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WGM0R::NORMAL_TOP => 0,
            WGM0R::PWM_PHASE => 1,
            WGM0R::CTC => 2,
            WGM0R::PWM_FAST => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WGM0R {
        match value {
            0 => WGM0R::NORMAL_TOP,
            1 => WGM0R::PWM_PHASE,
            2 => WGM0R::CTC,
            3 => WGM0R::PWM_FAST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_TOP`"]
    #[inline]
    pub fn is_normal_top(&self) -> bool {
        *self == WGM0R::NORMAL_TOP
    }
    #[doc = "Checks if the value of the field is `PWM_PHASE`"]
    #[inline]
    pub fn is_pwm_phase(&self) -> bool {
        *self == WGM0R::PWM_PHASE
    }
    #[doc = "Checks if the value of the field is `CTC`"]
    #[inline]
    pub fn is_ctc(&self) -> bool {
        *self == WGM0R::CTC
    }
    #[doc = "Checks if the value of the field is `PWM_FAST`"]
    #[inline]
    pub fn is_pwm_fast(&self) -> bool {
        *self == WGM0R::PWM_FAST
    }
}
#[doc = "Values that can be written to the field `COM_A`"]
pub enum COM_AW {
    #[doc = "Normal port operation, OC0x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC0x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC0x on Compare Match (If PWM is enabled, OC0x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC0x on Compare Match (If PWM is enabled, OC0x is cleared at TOP)"]
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
    #[doc = "Normal port operation, OC0x disconnected"]
    #[inline]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(COM_AW::DISCONNECTED)
    }
    #[doc = "Toggle OC0x on Compare Match (Might depend on WGM)"]
    #[inline]
    pub fn match_toggle(self) -> &'a mut W {
        self.variant(COM_AW::MATCH_TOGGLE)
    }
    #[doc = "Clear OC0x on Compare Match (If PWM is enabled, OC0x is set at TOP)"]
    #[inline]
    pub fn match_clear(self) -> &'a mut W {
        self.variant(COM_AW::MATCH_CLEAR)
    }
    #[doc = "Set OC0x on Compare Match (If PWM is enabled, OC0x is cleared at TOP)"]
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
    #[doc = "Normal port operation, OC0x disconnected"]
    DISCONNECTED,
    #[doc = "Toggle OC0x on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE,
    #[doc = "Clear OC0x on Compare Match (If PWM is enabled, OC0x is set at TOP)"]
    MATCH_CLEAR,
    #[doc = "Set OC0x on Compare Match (If PWM is enabled, OC0x is cleared at TOP)"]
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
    #[doc = "Normal port operation, OC0x disconnected"]
    #[inline]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(COM_BW::DISCONNECTED)
    }
    #[doc = "Toggle OC0x on Compare Match (Might depend on WGM)"]
    #[inline]
    pub fn match_toggle(self) -> &'a mut W {
        self.variant(COM_BW::MATCH_TOGGLE)
    }
    #[doc = "Clear OC0x on Compare Match (If PWM is enabled, OC0x is set at TOP)"]
    #[inline]
    pub fn match_clear(self) -> &'a mut W {
        self.variant(COM_BW::MATCH_CLEAR)
    }
    #[doc = "Set OC0x on Compare Match (If PWM is enabled, OC0x is cleared at TOP)"]
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
#[doc = "Values that can be written to the field `WGM0`"]
pub enum WGM0W {
    #[doc = "Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*"]
    NORMAL_TOP,
    #[doc = "Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*"]
    PWM_PHASE,
    #[doc = "CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*"]
    CTC,
    #[doc = "Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*"]
    PWM_FAST,
}
impl WGM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WGM0W::NORMAL_TOP => 0,
            WGM0W::PWM_PHASE => 1,
            WGM0W::CTC => 2,
            WGM0W::PWM_FAST => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WGM0W<'a> {
    w: &'a mut W,
}
impl<'a> _WGM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WGM0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*"]
    #[inline]
    pub fn normal_top(self) -> &'a mut W {
        self.variant(WGM0W::NORMAL_TOP)
    }
    #[doc = "Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*"]
    #[inline]
    pub fn pwm_phase(self) -> &'a mut W {
        self.variant(WGM0W::PWM_PHASE)
    }
    #[doc = "CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*"]
    #[inline]
    pub fn ctc(self) -> &'a mut W {
        self.variant(WGM0W::CTC)
    }
    #[doc = "Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*"]
    #[inline]
    pub fn pwm_fast(self) -> &'a mut W {
        self.variant(WGM0W::PWM_FAST)
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
    #[doc = "Bits 6:7 - Compare Match Output A Mode"]
    #[inline]
    pub fn com_a(&self) -> COM_AR {
        COM_AR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - Compare Match Output B Mode"]
    #[inline]
    pub fn com_b(&self) -> COM_BR {
        COM_BR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 0:1 - Waveform Generation Mode Bits 1:0"]
    #[inline]
    pub fn wgm0(&self) -> WGM0R {
        WGM0R::_from({
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
