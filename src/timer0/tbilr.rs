#[doc = "Reader of register TBILR"]
pub type R = crate::R<u32, super::TBILR>;
#[doc = "Writer for register TBILR"]
pub type W = crate::W<u32, super::TBILR>;
#[doc = "Register TBILR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBILR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER_TBILR_TBILRL`"]
pub type TIMER_TBILR_TBILRL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMER_TBILR_TBILRL`"]
pub struct TIMER_TBILR_TBILRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TBILR_TBILRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B Interval Load Register"]
    #[inline(always)]
    pub fn timer_tbilr_tbilrl(&self) -> TIMER_TBILR_TBILRL_R {
        TIMER_TBILR_TBILRL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer B Interval Load Register"]
    #[inline(always)]
    pub fn timer_tbilr_tbilrl(&mut self) -> TIMER_TBILR_TBILRL_W {
        TIMER_TBILR_TBILRL_W { w: self }
    }
}
