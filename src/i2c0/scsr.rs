#[doc = "Reader of register SCSR"]
pub type R = crate::R<u32, super::SCSR>;
#[doc = "Writer for register SCSR"]
pub type W = crate::W<u32, super::SCSR>;
#[doc = "Register SCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_SCSR_DA`"]
pub type I2C_SCSR_DA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SCSR_DA`"]
pub struct I2C_SCSR_DA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCSR_DA_W<'a> {
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
    #[doc = "Bit 0 - Device Active"]
    #[inline(always)]
    pub fn i2c_scsr_da(&self) -> I2C_SCSR_DA_R {
        I2C_SCSR_DA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device Active"]
    #[inline(always)]
    pub fn i2c_scsr_da(&mut self) -> I2C_SCSR_DA_W {
        I2C_SCSR_DA_W { w: self }
    }
}
