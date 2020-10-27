#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Writer for register MIS"]
pub type W = crate::W<u32, super::MIS>;
#[doc = "Register MIS `reset()`'s with value 0"]
impl crate::ResetValue for super::MIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_MIS_WDTMIS`"]
pub type WDT_MIS_WDTMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_MIS_WDTMIS`"]
pub struct WDT_MIS_WDTMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_MIS_WDTMIS_W<'a> {
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
    #[doc = "Bit 0 - Watchdog Masked Interrupt Status"]
    #[inline(always)]
    pub fn wdt_mis_wdtmis(&self) -> WDT_MIS_WDTMIS_R {
        WDT_MIS_WDTMIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Masked Interrupt Status"]
    #[inline(always)]
    pub fn wdt_mis_wdtmis(&mut self) -> WDT_MIS_WDTMIS_W {
        WDT_MIS_WDTMIS_W { w: self }
    }
}
