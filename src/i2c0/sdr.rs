#[doc = "Reader of register SDR"]
pub type R = crate::R<u32, super::SDR>;
#[doc = "Writer for register SDR"]
pub type W = crate::W<u32, super::SDR>;
#[doc = "Register SDR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_SDR_DATA`"]
pub type I2C_SDR_DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_SDR_DATA`"]
pub struct I2C_SDR_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SDR_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data for Transfer"]
    #[inline(always)]
    pub fn i2c_sdr_data(&self) -> I2C_SDR_DATA_R {
        I2C_SDR_DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data for Transfer"]
    #[inline(always)]
    pub fn i2c_sdr_data(&mut self) -> I2C_SDR_DATA_W {
        I2C_SDR_DATA_W { w: self }
    }
}
