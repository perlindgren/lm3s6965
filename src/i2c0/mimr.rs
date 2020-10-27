#[doc = "Reader of register MIMR"]
pub type R = crate::R<u32, super::MIMR>;
#[doc = "Writer for register MIMR"]
pub type W = crate::W<u32, super::MIMR>;
#[doc = "Register MIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::MIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_MIMR_IM`"]
pub type I2C_MIMR_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_MIMR_IM`"]
pub struct I2C_MIMR_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MIMR_IM_W<'a> {
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
    #[doc = "Bit 0 - Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_im(&self) -> I2C_MIMR_IM_R {
        I2C_MIMR_IM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_im(&mut self) -> I2C_MIMR_IM_W {
        I2C_MIMR_IM_W { w: self }
    }
}
