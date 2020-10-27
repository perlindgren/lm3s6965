#[doc = "Reader of register IMC"]
pub type R = crate::R<u32, super::IMC>;
#[doc = "Writer for register IMC"]
pub type W = crate::W<u32, super::IMC>;
#[doc = "Register IMC `reset()`'s with value 0"]
impl crate::ResetValue for super::IMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCTL_IMC_BORIM`"]
pub type SYSCTL_IMC_BORIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_IMC_BORIM`"]
pub struct SYSCTL_IMC_BORIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_IMC_BORIM_W<'a> {
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
#[doc = "Reader of field `SYSCTL_IMC_PLLLIM`"]
pub type SYSCTL_IMC_PLLLIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_IMC_PLLLIM`"]
pub struct SYSCTL_IMC_PLLLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_IMC_PLLLIM_W<'a> {
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
    #[doc = "Bit 1 - Brown-Out Reset Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_borim(&self) -> SYSCTL_IMC_BORIM_R {
        SYSCTL_IMC_BORIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_plllim(&self) -> SYSCTL_IMC_PLLLIM_R {
        SYSCTL_IMC_PLLLIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Brown-Out Reset Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_borim(&mut self) -> SYSCTL_IMC_BORIM_W {
        SYSCTL_IMC_BORIM_W { w: self }
    }
    #[doc = "Bit 6 - PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_plllim(&mut self) -> SYSCTL_IMC_PLLLIM_W {
        SYSCTL_IMC_PLLLIM_W { w: self }
    }
}
