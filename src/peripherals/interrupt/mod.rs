use core::convert::TryFrom;
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "1 - External Interrupt 0"]
    INT0,
    #[doc = "2 - External Interrupt 1"]
    INT1,
    #[doc = "3 - External Interrupt 2"]
    INT2,
    #[doc = "4 - External Interrupt 3"]
    INT3,
    #[doc = "7 - External Interrupt 6"]
    INT6,
    #[doc = "16 - Timer/Counter1 Capture Event"]
    TIMER1_CAPT,
    #[doc = "17 - Timer/Counter1 Compare Match A"]
    TIMER1_COMPA,
    #[doc = "18 - Timer/Counter1 Compare Match B"]
    TIMER1_COMPB,
    #[doc = "19 - Timer/Counter1 Compare Match C"]
    TIMER1_COMPC,
    #[doc = "20 - Timer/Counter1 Overflow"]
    TIMER1_OVF,
    #[doc = "21 - Timer/Counter0 Compare Match A"]
    TIMER0_COMPA,
    #[doc = "22 - Timer/Counter0 Compare Match B"]
    TIMER0_COMPB,
    #[doc = "23 - Timer/Counter0 Overflow"]
    TIMER0_OVF,
    #[doc = "31 - Timer/Counter3 Capture Event"]
    TIMER3_CAPT,
    #[doc = "32 - Timer/Counter3 Compare Match A"]
    TIMER3_COMPA,
    #[doc = "33 - Timer/Counter3 Compare Match B"]
    TIMER3_COMPB,
    #[doc = "34 - Timer/Counter3 Compare Match C"]
    TIMER3_COMPC,
    #[doc = "35 - Timer/Counter3 Overflow"]
    TIMER3_OVF,
    #[doc = "38 - Timer/Counter4 Compare Match A"]
    TIMER4_COMPA,
    #[doc = "39 - Timer/Counter4 Compare Match B"]
    TIMER4_COMPB,
    #[doc = "40 - Timer/Counter4 Compare Match D"]
    TIMER4_COMPD,
    #[doc = "41 - Timer/Counter4 Overflow"]
    TIMER4_OVF,
    #[doc = "42 - Timer/Counter4 Fault Protection Interrupt"]
    TIMER4_FPF,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::INT0 => 1,
            Interrupt::INT1 => 2,
            Interrupt::INT2 => 3,
            Interrupt::INT3 => 4,
            Interrupt::INT6 => 7,
            Interrupt::TIMER1_CAPT => 16,
            Interrupt::TIMER1_COMPA => 17,
            Interrupt::TIMER1_COMPB => 18,
            Interrupt::TIMER1_COMPC => 19,
            Interrupt::TIMER1_OVF => 20,
            Interrupt::TIMER0_COMPA => 21,
            Interrupt::TIMER0_COMPB => 22,
            Interrupt::TIMER0_OVF => 23,
            Interrupt::TIMER3_CAPT => 31,
            Interrupt::TIMER3_COMPA => 32,
            Interrupt::TIMER3_COMPB => 33,
            Interrupt::TIMER3_COMPC => 34,
            Interrupt::TIMER3_OVF => 35,
            Interrupt::TIMER4_COMPA => 38,
            Interrupt::TIMER4_COMPB => 39,
            Interrupt::TIMER4_COMPD => 40,
            Interrupt::TIMER4_OVF => 41,
            Interrupt::TIMER4_FPF => 42,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl TryFrom<u8> for Interrupt {
    type Error = TryFromInterruptError;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Interrupt::INT0),
            2 => Ok(Interrupt::INT1),
            3 => Ok(Interrupt::INT2),
            4 => Ok(Interrupt::INT3),
            7 => Ok(Interrupt::INT6),
            16 => Ok(Interrupt::TIMER1_CAPT),
            17 => Ok(Interrupt::TIMER1_COMPA),
            18 => Ok(Interrupt::TIMER1_COMPB),
            19 => Ok(Interrupt::TIMER1_COMPC),
            20 => Ok(Interrupt::TIMER1_OVF),
            21 => Ok(Interrupt::TIMER0_COMPA),
            22 => Ok(Interrupt::TIMER0_COMPB),
            23 => Ok(Interrupt::TIMER0_OVF),
            31 => Ok(Interrupt::TIMER3_CAPT),
            32 => Ok(Interrupt::TIMER3_COMPA),
            33 => Ok(Interrupt::TIMER3_COMPB),
            34 => Ok(Interrupt::TIMER3_COMPC),
            35 => Ok(Interrupt::TIMER3_OVF),
            38 => Ok(Interrupt::TIMER4_COMPA),
            39 => Ok(Interrupt::TIMER4_COMPB),
            40 => Ok(Interrupt::TIMER4_COMPD),
            41 => Ok(Interrupt::TIMER4_OVF),
            42 => Ok(Interrupt::TIMER4_FPF),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
