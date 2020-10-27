#[doc = "Reader of register DCGC0"]
pub type R = crate::R<u32, super::DCGC0>;
#[doc = "Writer for register DCGC0"]
pub type W = crate::W<u32, super::DCGC0>;
#[doc = "Register DCGC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DCGC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCTL_DCGC0_HIB`"]
pub type SYSCTL_DCGC0_HIB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC0_HIB`"]
pub struct SYSCTL_DCGC0_HIB_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC0_HIB_W<'a> {
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
    #[doc = "Bit 6 - HIB Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc0_hib(&self) -> SYSCTL_DCGC0_HIB_R {
        SYSCTL_DCGC0_HIB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - HIB Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc0_hib(&mut self) -> SYSCTL_DCGC0_HIB_W {
        SYSCTL_DCGC0_HIB_W { w: self }
    }
}
