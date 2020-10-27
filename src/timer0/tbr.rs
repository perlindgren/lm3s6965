#[doc = "Reader of register TBR"]
pub type R = crate::R<u32, super::TBR>;
#[doc = "Writer for register TBR"]
pub type W = crate::W<u32, super::TBR>;
#[doc = "Register TBR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER_TBR_TBRL`"]
pub type TIMER_TBR_TBRL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMER_TBR_TBRL`"]
pub struct TIMER_TBR_TBRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TBR_TBRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B"]
    #[inline(always)]
    pub fn timer_tbr_tbrl(&self) -> TIMER_TBR_TBRL_R {
        TIMER_TBR_TBRL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer B"]
    #[inline(always)]
    pub fn timer_tbr_tbrl(&mut self) -> TIMER_TBR_TBRL_W {
        TIMER_TBR_TBRL_W { w: self }
    }
}
