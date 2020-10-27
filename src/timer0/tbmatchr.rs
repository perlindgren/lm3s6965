#[doc = "Reader of register TBMATCHR"]
pub type R = crate::R<u32, super::TBMATCHR>;
#[doc = "Writer for register TBMATCHR"]
pub type W = crate::W<u32, super::TBMATCHR>;
#[doc = "Register TBMATCHR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBMATCHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER_TBMATCHR_TBMRL`"]
pub type TIMER_TBMATCHR_TBMRL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMER_TBMATCHR_TBMRL`"]
pub struct TIMER_TBMATCHR_TBMRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TBMATCHR_TBMRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B Match Register Low"]
    #[inline(always)]
    pub fn timer_tbmatchr_tbmrl(&self) -> TIMER_TBMATCHR_TBMRL_R {
        TIMER_TBMATCHR_TBMRL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer B Match Register Low"]
    #[inline(always)]
    pub fn timer_tbmatchr_tbmrl(&mut self) -> TIMER_TBMATCHR_TBMRL_W {
        TIMER_TBMATCHR_TBMRL_W { w: self }
    }
}
