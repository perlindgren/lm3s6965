#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPTM Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_CFG_A {
    #[doc = "0: 32-bit timer configuration"]
    TIMER_CFG_32_BIT_TIMER = 0,
    #[doc = "1: 32-bit real-time clock (RTC) counter configuration"]
    TIMER_CFG_32_BIT_RTC = 1,
    #[doc = "4: 16-bit timer configuration. The function is controlled by bits 1:0 of GPTMTAMR and GPTMTBMR"]
    TIMER_CFG_16_BIT = 4,
}
impl From<TIMER_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMER_CFG`"]
pub type TIMER_CFG_R = crate::R<u8, TIMER_CFG_A>;
impl TIMER_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMER_CFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMER_CFG_A::TIMER_CFG_32_BIT_TIMER),
            1 => Val(TIMER_CFG_A::TIMER_CFG_32_BIT_RTC),
            4 => Val(TIMER_CFG_A::TIMER_CFG_16_BIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CFG_32_BIT_TIMER`"]
    #[inline(always)]
    pub fn is_timer_cfg_32_bit_timer(&self) -> bool {
        *self == TIMER_CFG_A::TIMER_CFG_32_BIT_TIMER
    }
    #[doc = "Checks if the value of the field is `TIMER_CFG_32_BIT_RTC`"]
    #[inline(always)]
    pub fn is_timer_cfg_32_bit_rtc(&self) -> bool {
        *self == TIMER_CFG_A::TIMER_CFG_32_BIT_RTC
    }
    #[doc = "Checks if the value of the field is `TIMER_CFG_16_BIT`"]
    #[inline(always)]
    pub fn is_timer_cfg_16_bit(&self) -> bool {
        *self == TIMER_CFG_A::TIMER_CFG_16_BIT
    }
}
#[doc = "Write proxy for field `TIMER_CFG`"]
pub struct TIMER_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_CFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "32-bit timer configuration"]
    #[inline(always)]
    pub fn timer_cfg_32_bit_timer(self) -> &'a mut W {
        self.variant(TIMER_CFG_A::TIMER_CFG_32_BIT_TIMER)
    }
    #[doc = "32-bit real-time clock (RTC) counter configuration"]
    #[inline(always)]
    pub fn timer_cfg_32_bit_rtc(self) -> &'a mut W {
        self.variant(TIMER_CFG_A::TIMER_CFG_32_BIT_RTC)
    }
    #[doc = "16-bit timer configuration. The function is controlled by bits 1:0 of GPTMTAMR and GPTMTBMR"]
    #[inline(always)]
    pub fn timer_cfg_16_bit(self) -> &'a mut W {
        self.variant(TIMER_CFG_A::TIMER_CFG_16_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - GPTM Configuration"]
    #[inline(always)]
    pub fn timer_cfg(&self) -> TIMER_CFG_R {
        TIMER_CFG_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - GPTM Configuration"]
    #[inline(always)]
    pub fn timer_cfg(&mut self) -> TIMER_CFG_W {
        TIMER_CFG_W { w: self }
    }
}
