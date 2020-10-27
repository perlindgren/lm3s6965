#[doc = "Reader of register INVERT"]
pub type R = crate::R<u32, super::INVERT>;
#[doc = "Writer for register INVERT"]
pub type W = crate::W<u32, super::INVERT>;
#[doc = "Register INVERT `reset()`'s with value 0"]
impl crate::ResetValue for super::INVERT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_INVERT_PWM0INV`"]
pub type PWM_INVERT_PWM0INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_INVERT_PWM0INV`"]
pub struct PWM_INVERT_PWM0INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INVERT_PWM0INV_W<'a> {
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
#[doc = "Reader of field `PWM_INVERT_PWM1INV`"]
pub type PWM_INVERT_PWM1INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_INVERT_PWM1INV`"]
pub struct PWM_INVERT_PWM1INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INVERT_PWM1INV_W<'a> {
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
#[doc = "Reader of field `PWM_INVERT_PWM2INV`"]
pub type PWM_INVERT_PWM2INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_INVERT_PWM2INV`"]
pub struct PWM_INVERT_PWM2INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INVERT_PWM2INV_W<'a> {
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
#[doc = "Reader of field `PWM_INVERT_PWM3INV`"]
pub type PWM_INVERT_PWM3INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_INVERT_PWM3INV`"]
pub struct PWM_INVERT_PWM3INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INVERT_PWM3INV_W<'a> {
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
#[doc = "Reader of field `PWM_INVERT_PWM4INV`"]
pub type PWM_INVERT_PWM4INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_INVERT_PWM4INV`"]
pub struct PWM_INVERT_PWM4INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INVERT_PWM4INV_W<'a> {
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
#[doc = "Reader of field `PWM_INVERT_PWM5INV`"]
pub type PWM_INVERT_PWM5INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_INVERT_PWM5INV`"]
pub struct PWM_INVERT_PWM5INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INVERT_PWM5INV_W<'a> {
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
    #[doc = "Bit 0 - Invert PWM0 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm0inv(&self) -> PWM_INVERT_PWM0INV_R {
        PWM_INVERT_PWM0INV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Invert PWM1 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm1inv(&self) -> PWM_INVERT_PWM1INV_R {
        PWM_INVERT_PWM1INV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Invert PWM2 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm2inv(&self) -> PWM_INVERT_PWM2INV_R {
        PWM_INVERT_PWM2INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Invert PWM3 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm3inv(&self) -> PWM_INVERT_PWM3INV_R {
        PWM_INVERT_PWM3INV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Invert PWM4 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm4inv(&self) -> PWM_INVERT_PWM4INV_R {
        PWM_INVERT_PWM4INV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Invert PWM5 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm5inv(&self) -> PWM_INVERT_PWM5INV_R {
        PWM_INVERT_PWM5INV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invert PWM0 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm0inv(&mut self) -> PWM_INVERT_PWM0INV_W {
        PWM_INVERT_PWM0INV_W { w: self }
    }
    #[doc = "Bit 1 - Invert PWM1 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm1inv(&mut self) -> PWM_INVERT_PWM1INV_W {
        PWM_INVERT_PWM1INV_W { w: self }
    }
    #[doc = "Bit 2 - Invert PWM2 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm2inv(&mut self) -> PWM_INVERT_PWM2INV_W {
        PWM_INVERT_PWM2INV_W { w: self }
    }
    #[doc = "Bit 3 - Invert PWM3 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm3inv(&mut self) -> PWM_INVERT_PWM3INV_W {
        PWM_INVERT_PWM3INV_W { w: self }
    }
    #[doc = "Bit 4 - Invert PWM4 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm4inv(&mut self) -> PWM_INVERT_PWM4INV_W {
        PWM_INVERT_PWM4INV_W { w: self }
    }
    #[doc = "Bit 5 - Invert PWM5 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm5inv(&mut self) -> PWM_INVERT_PWM5INV_W {
        PWM_INVERT_PWM5INV_W { w: self }
    }
}
