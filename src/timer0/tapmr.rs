#[doc = "Reader of register TAPMR"]
pub type R = crate::R<u32, super::TAPMR>;
#[doc = "Writer for register TAPMR"]
pub type W = crate::W<u32, super::TAPMR>;
#[doc = "Register TAPMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAPMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER_TAPMR_TAPSMR`"]
pub type TIMER_TAPMR_TAPSMR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMER_TAPMR_TAPSMR`"]
pub struct TIMER_TAPMR_TAPSMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TAPMR_TAPSMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPTM TimerA Prescale Match"]
    #[inline(always)]
    pub fn timer_tapmr_tapsmr(&self) -> TIMER_TAPMR_TAPSMR_R {
        TIMER_TAPMR_TAPSMR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM TimerA Prescale Match"]
    #[inline(always)]
    pub fn timer_tapmr_tapsmr(&mut self) -> TIMER_TAPMR_TAPSMR_W {
        TIMER_TAPMR_TAPSMR_W { w: self }
    }
}
