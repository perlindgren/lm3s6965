#[doc = "Reader of register SMIS"]
pub type R = crate::R<u32, super::SMIS>;
#[doc = "Writer for register SMIS"]
pub type W = crate::W<u32, super::SMIS>;
#[doc = "Register SMIS `reset()`'s with value 0"]
impl crate::ResetValue for super::SMIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_SMIS_DATAMIS`"]
pub type I2C_SMIS_DATAMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SMIS_DATAMIS`"]
pub struct I2C_SMIS_DATAMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SMIS_DATAMIS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Data Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_datamis(&self) -> I2C_SMIS_DATAMIS_R {
        I2C_SMIS_DATAMIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_datamis(&mut self) -> I2C_SMIS_DATAMIS_W {
        I2C_SMIS_DATAMIS_W { w: self }
    }
}
