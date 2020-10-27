#[doc = "Reader of register PBORCTL"]
pub type R = crate::R<u32, super::PBORCTL>;
#[doc = "Writer for register PBORCTL"]
pub type W = crate::W<u32, super::PBORCTL>;
#[doc = "Register PBORCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PBORCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCTL_PBORCTL_BORIOR`"]
pub type SYSCTL_PBORCTL_BORIOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_PBORCTL_BORIOR`"]
pub struct SYSCTL_PBORCTL_BORIOR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_PBORCTL_BORIOR_W<'a> {
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
    #[doc = "Bit 1 - BOR Interrupt or Reset"]
    #[inline(always)]
    pub fn sysctl_pborctl_borior(&self) -> SYSCTL_PBORCTL_BORIOR_R {
        SYSCTL_PBORCTL_BORIOR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BOR Interrupt or Reset"]
    #[inline(always)]
    pub fn sysctl_pborctl_borior(&mut self) -> SYSCTL_PBORCTL_BORIOR_W {
        SYSCTL_PBORCTL_BORIOR_W { w: self }
    }
}
