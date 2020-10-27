#[doc = "Reader of register TBPMR"]
pub type R = crate::R<u32, super::TBPMR>;
#[doc = "Writer for register TBPMR"]
pub type W = crate::W<u32, super::TBPMR>;
#[doc = "Register TBPMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBPMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER_TBPMR_TBPSMR`"]
pub type TIMER_TBPMR_TBPSMR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMER_TBPMR_TBPSMR`"]
pub struct TIMER_TBPMR_TBPSMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TBPMR_TBPSMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPTM TimerB Prescale Match"]
    #[inline(always)]
    pub fn timer_tbpmr_tbpsmr(&self) -> TIMER_TBPMR_TBPSMR_R {
        TIMER_TBPMR_TBPSMR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM TimerB Prescale Match"]
    #[inline(always)]
    pub fn timer_tbpmr_tbpsmr(&mut self) -> TIMER_TBPMR_TBPSMR_W {
        TIMER_TBPMR_TBPSMR_W { w: self }
    }
}
