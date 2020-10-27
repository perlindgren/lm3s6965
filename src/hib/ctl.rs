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
#[doc = "Reader of field `HIB_CTL_RTCEN`"]
pub type HIB_CTL_RTCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIB_CTL_RTCEN`"]
pub struct HIB_CTL_RTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_CTL_RTCEN_W<'a> {
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
#[doc = "Reader of field `HIB_CTL_HIBREQ`"]
pub type HIB_CTL_HIBREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIB_CTL_HIBREQ`"]
pub struct HIB_CTL_HIBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_CTL_HIBREQ_W<'a> {
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
#[doc = "Reader of field `HIB_CTL_CLKSEL`"]
pub type HIB_CTL_CLKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIB_CTL_CLKSEL`"]
pub struct HIB_CTL_CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_CTL_CLKSEL_W<'a> {
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
#[doc = "Reader of field `HIB_CTL_RTCWEN`"]
pub type HIB_CTL_RTCWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIB_CTL_RTCWEN`"]
pub struct HIB_CTL_RTCWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_CTL_RTCWEN_W<'a> {
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
#[doc = "Reader of field `HIB_CTL_PINWEN`"]
pub type HIB_CTL_PINWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIB_CTL_PINWEN`"]
pub struct HIB_CTL_PINWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_CTL_PINWEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `HIB_CTL_LOWBATEN`"]
pub type HIB_CTL_LOWBATEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIB_CTL_LOWBATEN`"]
pub struct HIB_CTL_LOWBATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_CTL_LOWBATEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `HIB_CTL_CLK32EN`"]
pub type HIB_CTL_CLK32EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIB_CTL_CLK32EN`"]
pub struct HIB_CTL_CLK32EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_CTL_CLK32EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `HIB_CTL_VABORT`"]
pub type HIB_CTL_VABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIB_CTL_VABORT`"]
pub struct HIB_CTL_VABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_CTL_VABORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline(always)]
    pub fn hib_ctl_rtcen(&self) -> HIB_CTL_RTCEN_R {
        HIB_CTL_RTCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hibernation Request"]
    #[inline(always)]
    pub fn hib_ctl_hibreq(&self) -> HIB_CTL_HIBREQ_R {
        HIB_CTL_HIBREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Hibernation Module Clock Select"]
    #[inline(always)]
    pub fn hib_ctl_clksel(&self) -> HIB_CTL_CLKSEL_R {
        HIB_CTL_CLKSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC Wake-up Enable"]
    #[inline(always)]
    pub fn hib_ctl_rtcwen(&self) -> HIB_CTL_RTCWEN_R {
        HIB_CTL_RTCWEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External WAKE Pin Enable"]
    #[inline(always)]
    pub fn hib_ctl_pinwen(&self) -> HIB_CTL_PINWEN_R {
        HIB_CTL_PINWEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Low Battery Monitoring Enable"]
    #[inline(always)]
    pub fn hib_ctl_lowbaten(&self) -> HIB_CTL_LOWBATEN_R {
        HIB_CTL_LOWBATEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clocking Enable"]
    #[inline(always)]
    pub fn hib_ctl_clk32en(&self) -> HIB_CTL_CLK32EN_R {
        HIB_CTL_CLK32EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Power Cut Abort Enable"]
    #[inline(always)]
    pub fn hib_ctl_vabort(&self) -> HIB_CTL_VABORT_R {
        HIB_CTL_VABORT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline(always)]
    pub fn hib_ctl_rtcen(&mut self) -> HIB_CTL_RTCEN_W {
        HIB_CTL_RTCEN_W { w: self }
    }
    #[doc = "Bit 1 - Hibernation Request"]
    #[inline(always)]
    pub fn hib_ctl_hibreq(&mut self) -> HIB_CTL_HIBREQ_W {
        HIB_CTL_HIBREQ_W { w: self }
    }
    #[doc = "Bit 2 - Hibernation Module Clock Select"]
    #[inline(always)]
    pub fn hib_ctl_clksel(&mut self) -> HIB_CTL_CLKSEL_W {
        HIB_CTL_CLKSEL_W { w: self }
    }
    #[doc = "Bit 3 - RTC Wake-up Enable"]
    #[inline(always)]
    pub fn hib_ctl_rtcwen(&mut self) -> HIB_CTL_RTCWEN_W {
        HIB_CTL_RTCWEN_W { w: self }
    }
    #[doc = "Bit 4 - External WAKE Pin Enable"]
    #[inline(always)]
    pub fn hib_ctl_pinwen(&mut self) -> HIB_CTL_PINWEN_W {
        HIB_CTL_PINWEN_W { w: self }
    }
    #[doc = "Bit 5 - Low Battery Monitoring Enable"]
    #[inline(always)]
    pub fn hib_ctl_lowbaten(&mut self) -> HIB_CTL_LOWBATEN_W {
        HIB_CTL_LOWBATEN_W { w: self }
    }
    #[doc = "Bit 6 - Clocking Enable"]
    #[inline(always)]
    pub fn hib_ctl_clk32en(&mut self) -> HIB_CTL_CLK32EN_W {
        HIB_CTL_CLK32EN_W { w: self }
    }
    #[doc = "Bit 7 - Power Cut Abort Enable"]
    #[inline(always)]
    pub fn hib_ctl_vabort(&mut self) -> HIB_CTL_VABORT_W {
        HIB_CTL_VABORT_W { w: self }
    }
}
