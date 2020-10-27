#[doc = "Reader of register MTPR"]
pub type R = crate::R<u32, super::MTPR>;
#[doc = "Writer for register MTPR"]
pub type W = crate::W<u32, super::MTPR>;
#[doc = "Register MTPR `reset()`'s with value 0"]
impl crate::ResetValue for super::MTPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_MTPR_TPR`"]
pub type I2C_MTPR_TPR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_MTPR_TPR`"]
pub struct I2C_MTPR_TPR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MTPR_TPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - SCL Clock Period"]
    #[inline(always)]
    pub fn i2c_mtpr_tpr(&self) -> I2C_MTPR_TPR_R {
        I2C_MTPR_TPR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - SCL Clock Period"]
    #[inline(always)]
    pub fn i2c_mtpr_tpr(&mut self) -> I2C_MTPR_TPR_W {
        I2C_MTPR_TPR_W { w: self }
    }
}
