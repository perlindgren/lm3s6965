#[doc = "Reader of register PLLCFG"]
pub type R = crate::R<u32, super::PLLCFG>;
#[doc = "Writer for register PLLCFG"]
pub type W = crate::W<u32, super::PLLCFG>;
#[doc = "Register PLLCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PLLCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCTL_PLLCFG_R`"]
pub type SYSCTL_PLLCFG_R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCTL_PLLCFG_R`"]
pub struct SYSCTL_PLLCFG_R_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_PLLCFG_R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_PLLCFG_F`"]
pub type SYSCTL_PLLCFG_F_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SYSCTL_PLLCFG_F`"]
pub struct SYSCTL_PLLCFG_F_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_PLLCFG_F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 5)) | (((value as u32) & 0x01ff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - PLL R Value"]
    #[inline(always)]
    pub fn sysctl_pllcfg_r(&self) -> SYSCTL_PLLCFG_R_R {
        SYSCTL_PLLCFG_R_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:13 - PLL F Value"]
    #[inline(always)]
    pub fn sysctl_pllcfg_f(&self) -> SYSCTL_PLLCFG_F_R {
        SYSCTL_PLLCFG_F_R::new(((self.bits >> 5) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL R Value"]
    #[inline(always)]
    pub fn sysctl_pllcfg_r(&mut self) -> SYSCTL_PLLCFG_R_W {
        SYSCTL_PLLCFG_R_W { w: self }
    }
    #[doc = "Bits 5:13 - PLL F Value"]
    #[inline(always)]
    pub fn sysctl_pllcfg_f(&mut self) -> SYSCTL_PLLCFG_F_W {
        SYSCTL_PLLCFG_F_W { w: self }
    }
}
