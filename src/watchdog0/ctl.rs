#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_CTL_INTEN`"]
pub type WDT_CTL_INTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_CTL_INTEN`"]
pub struct WDT_CTL_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_CTL_INTEN_W<'a> {
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
#[doc = "Reader of field `WDT_CTL_RESEN`"]
pub type WDT_CTL_RESEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_CTL_RESEN`"]
pub struct WDT_CTL_RESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_CTL_RESEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdt_ctl_inten(&self) -> WDT_CTL_INTEN_R {
        WDT_CTL_INTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Watchdog Reset Enable"]
    #[inline(always)]
    pub fn wdt_ctl_resen(&self) -> WDT_CTL_RESEN_R {
        WDT_CTL_RESEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdt_ctl_inten(&mut self) -> WDT_CTL_INTEN_W {
        WDT_CTL_INTEN_W { w: self }
    }
    #[doc = "Bit 1 - Watchdog Reset Enable"]
    #[inline(always)]
    pub fn wdt_ctl_resen(&mut self) -> WDT_CTL_RESEN_W {
        WDT_CTL_RESEN_W { w: self }
    }
}
