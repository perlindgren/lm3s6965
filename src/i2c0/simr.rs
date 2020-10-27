#[doc = "Reader of register SIMR"]
pub type R = crate::R<u32, super::SIMR>;
#[doc = "Writer for register SIMR"]
pub type W = crate::W<u32, super::SIMR>;
#[doc = "Register SIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::SIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_SIMR_DATAIM`"]
pub type I2C_SIMR_DATAIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SIMR_DATAIM`"]
pub struct I2C_SIMR_DATAIM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SIMR_DATAIM_W<'a> {
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
    #[doc = "Bit 0 - Data Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dataim(&self) -> I2C_SIMR_DATAIM_R {
        I2C_SIMR_DATAIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dataim(&mut self) -> I2C_SIMR_DATAIM_W {
        I2C_SIMR_DATAIM_W { w: self }
    }
}
