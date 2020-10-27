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
#[doc = "Reader of field `SYSCTL_RIS_BORRIS`"]
pub type SYSCTL_RIS_BORRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RIS_BORRIS`"]
pub struct SYSCTL_RIS_BORRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RIS_BORRIS_W<'a> {
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
#[doc = "Reader of field `SYSCTL_RIS_PLLLRIS`"]
pub type SYSCTL_RIS_PLLLRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RIS_PLLLRIS`"]
pub struct SYSCTL_RIS_PLLLRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RIS_PLLLRIS_W<'a> {
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
impl R {
    #[doc = "Bit 1 - Brown-Out Reset Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_borris(&self) -> SYSCTL_RIS_BORRIS_R {
        SYSCTL_RIS_BORRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Lock Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_plllris(&self) -> SYSCTL_RIS_PLLLRIS_R {
        SYSCTL_RIS_PLLLRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Brown-Out Reset Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_borris(&mut self) -> SYSCTL_RIS_BORRIS_W {
        SYSCTL_RIS_BORRIS_W { w: self }
    }
    #[doc = "Bit 6 - PLL Lock Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_plllris(&mut self) -> SYSCTL_RIS_PLLLRIS_W {
        SYSCTL_RIS_PLLLRIS_W { w: self }
    }
}
