#[doc = "Reader of register TAMR"]
pub type R = crate::R<u32, super::TAMR>;
#[doc = "Writer for register TAMR"]
pub type W = crate::W<u32, super::TAMR>;
#[doc = "Register TAMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPTM Timer A Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_TAMR_TAMR_A {
    #[doc = "1: One-Shot Timer mode"]
    TIMER_TAMR_TAMR_1_SHOT = 1,
    #[doc = "2: Periodic Timer mode"]
    TIMER_TAMR_TAMR_PERIOD = 2,
    #[doc = "3: Capture mode"]
    TIMER_TAMR_TAMR_CAP = 3,
}
impl From<TIMER_TAMR_TAMR_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_TAMR_TAMR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMER_TAMR_TAMR`"]
pub type TIMER_TAMR_TAMR_R = crate::R<u8, TIMER_TAMR_TAMR_A>;
impl TIMER_TAMR_TAMR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMER_TAMR_TAMR_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_1_SHOT),
            2 => Val(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_PERIOD),
            3 => Val(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_CAP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TAMR_1_SHOT`"]
    #[inline(always)]
    pub fn is_timer_tamr_tamr_1_shot(&self) -> bool {
        *self == TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_1_SHOT
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TAMR_PERIOD`"]
    #[inline(always)]
    pub fn is_timer_tamr_tamr_period(&self) -> bool {
        *self == TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_PERIOD
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TAMR_CAP`"]
    #[inline(always)]
    pub fn is_timer_tamr_tamr_cap(&self) -> bool {
        *self == TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_CAP
    }
}
#[doc = "Write proxy for field `TIMER_TAMR_TAMR`"]
pub struct TIMER_TAMR_TAMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TAMR_TAMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_TAMR_TAMR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr_1_shot(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_1_SHOT)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr_period(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_PERIOD)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr_cap(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_CAP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TIMER_TAMR_TACMR`"]
pub type TIMER_TAMR_TACMR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_TAMR_TACMR`"]
pub struct TIMER_TAMR_TACMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TAMR_TACMR_W<'a> {
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
#[doc = "Reader of field `TIMER_TAMR_TAAMS`"]
pub type TIMER_TAMR_TAAMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_TAMR_TAAMS`"]
pub struct TIMER_TAMR_TAAMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TAMR_TAAMS_W<'a> {
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
    #[doc = "Bits 0:1 - GPTM Timer A Mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr(&self) -> TIMER_TAMR_TAMR_R {
        TIMER_TAMR_TAMR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode"]
    #[inline(always)]
    pub fn timer_tamr_tacmr(&self) -> TIMER_TAMR_TACMR_R {
        TIMER_TAMR_TACMR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPTM Timer A Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tamr_taams(&self) -> TIMER_TAMR_TAAMS_R {
        TIMER_TAMR_TAAMS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTM Timer A Mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr(&mut self) -> TIMER_TAMR_TAMR_W {
        TIMER_TAMR_TAMR_W { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode"]
    #[inline(always)]
    pub fn timer_tamr_tacmr(&mut self) -> TIMER_TAMR_TACMR_W {
        TIMER_TAMR_TACMR_W { w: self }
    }
    #[doc = "Bit 3 - GPTM Timer A Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tamr_taams(&mut self) -> TIMER_TAMR_TAAMS_W {
        TIMER_TAMR_TAAMS_W { w: self }
    }
}
