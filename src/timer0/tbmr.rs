#[doc = "Reader of register TBMR"]
pub type R = crate::R<u32, super::TBMR>;
#[doc = "Writer for register TBMR"]
pub type W = crate::W<u32, super::TBMR>;
#[doc = "Register TBMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPTM Timer B Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_TBMR_TBMR_A {
    #[doc = "1: One-Shot Timer mode"]
    TIMER_TBMR_TBMR_1_SHOT = 1,
    #[doc = "2: Periodic Timer mode"]
    TIMER_TBMR_TBMR_PERIOD = 2,
    #[doc = "3: Capture mode"]
    TIMER_TBMR_TBMR_CAP = 3,
}
impl From<TIMER_TBMR_TBMR_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_TBMR_TBMR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMER_TBMR_TBMR`"]
pub type TIMER_TBMR_TBMR_R = crate::R<u8, TIMER_TBMR_TBMR_A>;
impl TIMER_TBMR_TBMR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMER_TBMR_TBMR_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_1_SHOT),
            2 => Val(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_PERIOD),
            3 => Val(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_CAP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TBMR_1_SHOT`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tbmr_1_shot(&self) -> bool {
        *self == TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_1_SHOT
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TBMR_PERIOD`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tbmr_period(&self) -> bool {
        *self == TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_PERIOD
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TBMR_CAP`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tbmr_cap(&self) -> bool {
        *self == TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_CAP
    }
}
#[doc = "Write proxy for field `TIMER_TBMR_TBMR`"]
pub struct TIMER_TBMR_TBMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TBMR_TBMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_TBMR_TBMR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr_1_shot(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_1_SHOT)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr_period(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_PERIOD)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr_cap(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_CAP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TIMER_TBMR_TBCMR`"]
pub type TIMER_TBMR_TBCMR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_TBMR_TBCMR`"]
pub struct TIMER_TBMR_TBCMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TBMR_TBCMR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TIMER_TBMR_TBAMS`"]
pub type TIMER_TBMR_TBAMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_TBMR_TBAMS`"]
pub struct TIMER_TBMR_TBAMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TBMR_TBAMS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr(&self) -> TIMER_TBMR_TBMR_R {
        TIMER_TBMR_TBMR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbcmr(&self) -> TIMER_TBMR_TBCMR_R {
        TIMER_TBMR_TBCMR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tbmr_tbams(&self) -> TIMER_TBMR_TBAMS_R {
        TIMER_TBMR_TBAMS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr(&mut self) -> TIMER_TBMR_TBMR_W {
        TIMER_TBMR_TBMR_W { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbcmr(&mut self) -> TIMER_TBMR_TBCMR_W {
        TIMER_TBMR_TBCMR_W { w: self }
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tbmr_tbams(&mut self) -> TIMER_TBMR_TBAMS_W {
        TIMER_TBMR_TBAMS_W { w: self }
    }
}
