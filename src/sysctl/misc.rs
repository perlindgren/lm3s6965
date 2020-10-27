#[doc = "Reader of register MISC"]
pub type R = crate::R<u32, super::MISC>;
#[doc = "Writer for register MISC"]
pub type W = crate::W<u32, super::MISC>;
#[doc = "Register MISC `reset()`'s with value 0"]
impl crate::ResetValue for super::MISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCTL_MISC_BORMIS`"]
pub type SYSCTL_MISC_BORMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_MISC_BORMIS`"]
pub struct SYSCTL_MISC_BORMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_MISC_BORMIS_W<'a> {
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
#[doc = "Reader of field `SYSCTL_MISC_PLLLMIS`"]
pub type SYSCTL_MISC_PLLLMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_MISC_PLLLMIS`"]
pub struct SYSCTL_MISC_PLLLMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_MISC_PLLLMIS_W<'a> {
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
    #[doc = "Bit 1 - BOR Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_bormis(&self) -> SYSCTL_MISC_BORMIS_R {
        SYSCTL_MISC_BORMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_plllmis(&self) -> SYSCTL_MISC_PLLLMIS_R {
        SYSCTL_MISC_PLLLMIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BOR Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_bormis(&mut self) -> SYSCTL_MISC_BORMIS_W {
        SYSCTL_MISC_BORMIS_W { w: self }
    }
    #[doc = "Bit 6 - PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_plllmis(&mut self) -> SYSCTL_MISC_PLLLMIS_W {
        SYSCTL_MISC_PLLLMIS_W { w: self }
    }
}
