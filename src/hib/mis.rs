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
#[doc = "Reader of field `HIB_MIS_RTCALT0`"]
pub type HIB_MIS_RTCALT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIB_MIS_RTCALT0`"]
pub struct HIB_MIS_RTCALT0_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_MIS_RTCALT0_W<'a> {
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
#[doc = "Reader of field `HIB_MIS_RTCALT1`"]
pub type HIB_MIS_RTCALT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIB_MIS_RTCALT1`"]
pub struct HIB_MIS_RTCALT1_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_MIS_RTCALT1_W<'a> {
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
#[doc = "Reader of field `HIB_MIS_LOWBAT`"]
pub type HIB_MIS_LOWBAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIB_MIS_LOWBAT`"]
pub struct HIB_MIS_LOWBAT_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_MIS_LOWBAT_W<'a> {
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
#[doc = "Reader of field `HIB_MIS_EXTW`"]
pub type HIB_MIS_EXTW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIB_MIS_EXTW`"]
pub struct HIB_MIS_EXTW_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_MIS_EXTW_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RTC Alert 0 Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_rtcalt0(&self) -> HIB_MIS_RTCALT0_R {
        HIB_MIS_RTCALT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Alert 1 Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_rtcalt1(&self) -> HIB_MIS_RTCALT1_R {
        HIB_MIS_RTCALT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Battery Voltage Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_lowbat(&self) -> HIB_MIS_LOWBAT_R {
        HIB_MIS_LOWBAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Wake-Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_extw(&self) -> HIB_MIS_EXTW_R {
        HIB_MIS_EXTW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alert 0 Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_rtcalt0(&mut self) -> HIB_MIS_RTCALT0_W {
        HIB_MIS_RTCALT0_W { w: self }
    }
    #[doc = "Bit 1 - RTC Alert 1 Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_rtcalt1(&mut self) -> HIB_MIS_RTCALT1_W {
        HIB_MIS_RTCALT1_W { w: self }
    }
    #[doc = "Bit 2 - Low Battery Voltage Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_lowbat(&mut self) -> HIB_MIS_LOWBAT_W {
        HIB_MIS_LOWBAT_W { w: self }
    }
    #[doc = "Bit 3 - External Wake-Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_extw(&mut self) -> HIB_MIS_EXTW_W {
        HIB_MIS_EXTW_W { w: self }
    }
}
