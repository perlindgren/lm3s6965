#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Writer for register RIS"]
pub type W = crate::W<u32, super::RIS>;
#[doc = "Register RIS `reset()`'s with value 0"]
impl crate::ResetValue for super::RIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_RIS_WDTRIS`"]
pub type WDT_RIS_WDTRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_RIS_WDTRIS`"]
pub struct WDT_RIS_WDTRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_RIS_WDTRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Raw Interrupt Status"]
    #[inline(always)]
    pub fn wdt_ris_wdtris(&self) -> WDT_RIS_WDTRIS_R {
        WDT_RIS_WDTRIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Raw Interrupt Status"]
    #[inline(always)]
    pub fn wdt_ris_wdtris(&mut self) -> WDT_RIS_WDTRIS_W {
        WDT_RIS_WDTRIS_W { w: self }
    }
}
