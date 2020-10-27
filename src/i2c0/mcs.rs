#[doc = "Reader of register MCS"]
pub type R = crate::R<u32, super::MCS>;
#[doc = "Writer for register MCS"]
pub type W = crate::W<u32, super::MCS>;
#[doc = "Register MCS `reset()`'s with value 0"]
impl crate::ResetValue for super::MCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_MCS_BUSY`"]
pub type I2C_MCS_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_MCS_BUSY`"]
pub struct I2C_MCS_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MCS_BUSY_W<'a> {
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
#[doc = "Reader of field `I2C_MCS_ERROR`"]
pub type I2C_MCS_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_MCS_ERROR`"]
pub struct I2C_MCS_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MCS_ERROR_W<'a> {
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
#[doc = "Reader of field `I2C_MCS_STOP`"]
pub type I2C_MCS_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_MCS_STOP`"]
pub struct I2C_MCS_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MCS_STOP_W<'a> {
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
#[doc = "Reader of field `I2C_MCS_DATACK`"]
pub type I2C_MCS_DATACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_MCS_DATACK`"]
pub struct I2C_MCS_DATACK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MCS_DATACK_W<'a> {
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
    #[doc = "Bit 0 - I2C Busy"]
    #[inline(always)]
    pub fn i2c_mcs_busy(&self) -> I2C_MCS_BUSY_R {
        I2C_MCS_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Error"]
    #[inline(always)]
    pub fn i2c_mcs_error(&self) -> I2C_MCS_ERROR_R {
        I2C_MCS_ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Generate STOP"]
    #[inline(always)]
    pub fn i2c_mcs_stop(&self) -> I2C_MCS_STOP_R {
        I2C_MCS_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Acknowledge Data"]
    #[inline(always)]
    pub fn i2c_mcs_datack(&self) -> I2C_MCS_DATACK_R {
        I2C_MCS_DATACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Busy"]
    #[inline(always)]
    pub fn i2c_mcs_busy(&mut self) -> I2C_MCS_BUSY_W {
        I2C_MCS_BUSY_W { w: self }
    }
    #[doc = "Bit 1 - Error"]
    #[inline(always)]
    pub fn i2c_mcs_error(&mut self) -> I2C_MCS_ERROR_W {
        I2C_MCS_ERROR_W { w: self }
    }
    #[doc = "Bit 2 - Generate STOP"]
    #[inline(always)]
    pub fn i2c_mcs_stop(&mut self) -> I2C_MCS_STOP_W {
        I2C_MCS_STOP_W { w: self }
    }
    #[doc = "Bit 3 - Acknowledge Data"]
    #[inline(always)]
    pub fn i2c_mcs_datack(&mut self) -> I2C_MCS_DATACK_W {
        I2C_MCS_DATACK_W { w: self }
    }
}
