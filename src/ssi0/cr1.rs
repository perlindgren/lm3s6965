#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Writer for register CR1"]
pub type W = crate::W<u32, super::CR1>;
#[doc = "Register CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSI_CR1_LBM`"]
pub type SSI_CR1_LBM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_CR1_LBM`"]
pub struct SSI_CR1_LBM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_CR1_LBM_W<'a> {
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
#[doc = "Reader of field `SSI_CR1_SSE`"]
pub type SSI_CR1_SSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_CR1_SSE`"]
pub struct SSI_CR1_SSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_CR1_SSE_W<'a> {
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
#[doc = "Reader of field `SSI_CR1_MS`"]
pub type SSI_CR1_MS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_CR1_MS`"]
pub struct SSI_CR1_MS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_CR1_MS_W<'a> {
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
#[doc = "Reader of field `SSI_CR1_SOD`"]
pub type SSI_CR1_SOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_CR1_SOD`"]
pub struct SSI_CR1_SOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_CR1_SOD_W<'a> {
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
    #[doc = "Bit 0 - SSI Loopback Mode"]
    #[inline(always)]
    pub fn ssi_cr1_lbm(&self) -> SSI_CR1_LBM_R {
        SSI_CR1_LBM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SSI Synchronous Serial Port Enable"]
    #[inline(always)]
    pub fn ssi_cr1_sse(&self) -> SSI_CR1_SSE_R {
        SSI_CR1_SSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSI Master/Slave Select"]
    #[inline(always)]
    pub fn ssi_cr1_ms(&self) -> SSI_CR1_MS_R {
        SSI_CR1_MS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SSI Slave Mode Output Disable"]
    #[inline(always)]
    pub fn ssi_cr1_sod(&self) -> SSI_CR1_SOD_R {
        SSI_CR1_SOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Loopback Mode"]
    #[inline(always)]
    pub fn ssi_cr1_lbm(&mut self) -> SSI_CR1_LBM_W {
        SSI_CR1_LBM_W { w: self }
    }
    #[doc = "Bit 1 - SSI Synchronous Serial Port Enable"]
    #[inline(always)]
    pub fn ssi_cr1_sse(&mut self) -> SSI_CR1_SSE_W {
        SSI_CR1_SSE_W { w: self }
    }
    #[doc = "Bit 2 - SSI Master/Slave Select"]
    #[inline(always)]
    pub fn ssi_cr1_ms(&mut self) -> SSI_CR1_MS_W {
        SSI_CR1_MS_W { w: self }
    }
    #[doc = "Bit 3 - SSI Slave Mode Output Disable"]
    #[inline(always)]
    pub fn ssi_cr1_sod(&mut self) -> SSI_CR1_SOD_W {
        SSI_CR1_SOD_W { w: self }
    }
}
