#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `WDT_ICR`"]
pub struct WDT_ICR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_ICR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Interrupt Clear"]
    #[inline(always)]
    pub fn wdt_icr(&mut self) -> WDT_ICR_W {
        WDT_ICR_W { w: self }
    }
}
