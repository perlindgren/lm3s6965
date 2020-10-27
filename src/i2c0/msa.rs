#[doc = "Reader of register MSA"]
pub type R = crate::R<u32, super::MSA>;
#[doc = "Writer for register MSA"]
pub type W = crate::W<u32, super::MSA>;
#[doc = "Register MSA `reset()`'s with value 0"]
impl crate::ResetValue for super::MSA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_MSA_RS`"]
pub type I2C_MSA_RS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_MSA_RS`"]
pub struct I2C_MSA_RS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MSA_RS_W<'a> {
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
#[doc = "Reader of field `I2C_MSA_SA`"]
pub type I2C_MSA_SA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_MSA_SA`"]
pub struct I2C_MSA_SA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MSA_SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive not send"]
    #[inline(always)]
    pub fn i2c_msa_rs(&self) -> I2C_MSA_RS_R {
        I2C_MSA_RS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - I2C Slave Address"]
    #[inline(always)]
    pub fn i2c_msa_sa(&self) -> I2C_MSA_SA_R {
        I2C_MSA_SA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive not send"]
    #[inline(always)]
    pub fn i2c_msa_rs(&mut self) -> I2C_MSA_RS_W {
        I2C_MSA_RS_W { w: self }
    }
    #[doc = "Bits 1:7 - I2C Slave Address"]
    #[inline(always)]
    pub fn i2c_msa_sa(&mut self) -> I2C_MSA_SA_W {
        I2C_MSA_SA_W { w: self }
    }
}
