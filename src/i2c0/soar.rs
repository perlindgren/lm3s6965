#[doc = "Reader of register SOAR"]
pub type R = crate::R<u32, super::SOAR>;
#[doc = "Writer for register SOAR"]
pub type W = crate::W<u32, super::SOAR>;
#[doc = "Register SOAR `reset()`'s with value 0"]
impl crate::ResetValue for super::SOAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_SOAR_OAR`"]
pub type I2C_SOAR_OAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_SOAR_OAR`"]
pub struct I2C_SOAR_OAR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SOAR_OAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - I2C Slave Own Address"]
    #[inline(always)]
    pub fn i2c_soar_oar(&self) -> I2C_SOAR_OAR_R {
        I2C_SOAR_OAR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - I2C Slave Own Address"]
    #[inline(always)]
    pub fn i2c_soar_oar(&mut self) -> I2C_SOAR_OAR_W {
        I2C_SOAR_OAR_W { w: self }
    }
}
