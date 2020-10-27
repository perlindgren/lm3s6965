#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_MCR_LPBK`"]
pub type I2C_MCR_LPBK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_MCR_LPBK`"]
pub struct I2C_MCR_LPBK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MCR_LPBK_W<'a> {
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
#[doc = "Reader of field `I2C_MCR_MFE`"]
pub type I2C_MCR_MFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_MCR_MFE`"]
pub struct I2C_MCR_MFE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MCR_MFE_W<'a> {
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
#[doc = "Reader of field `I2C_MCR_SFE`"]
pub type I2C_MCR_SFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_MCR_SFE`"]
pub struct I2C_MCR_SFE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MCR_SFE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2C Loopback"]
    #[inline(always)]
    pub fn i2c_mcr_lpbk(&self) -> I2C_MCR_LPBK_R {
        I2C_MCR_LPBK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Master Function Enable"]
    #[inline(always)]
    pub fn i2c_mcr_mfe(&self) -> I2C_MCR_MFE_R {
        I2C_MCR_MFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Slave Function Enable"]
    #[inline(always)]
    pub fn i2c_mcr_sfe(&self) -> I2C_MCR_SFE_R {
        I2C_MCR_SFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Loopback"]
    #[inline(always)]
    pub fn i2c_mcr_lpbk(&mut self) -> I2C_MCR_LPBK_W {
        I2C_MCR_LPBK_W { w: self }
    }
    #[doc = "Bit 4 - I2C Master Function Enable"]
    #[inline(always)]
    pub fn i2c_mcr_mfe(&mut self) -> I2C_MCR_MFE_W {
        I2C_MCR_MFE_W { w: self }
    }
    #[doc = "Bit 5 - I2C Slave Function Enable"]
    #[inline(always)]
    pub fn i2c_mcr_sfe(&mut self) -> I2C_MCR_SFE_W {
        I2C_MCR_SFE_W { w: self }
    }
}
