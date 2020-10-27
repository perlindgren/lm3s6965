#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_INTEN_INTPWM0`"]
pub type PWM_INTEN_INTPWM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_INTEN_INTPWM0`"]
pub struct PWM_INTEN_INTPWM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INTEN_INTPWM0_W<'a> {
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
#[doc = "Reader of field `PWM_INTEN_INTPWM1`"]
pub type PWM_INTEN_INTPWM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_INTEN_INTPWM1`"]
pub struct PWM_INTEN_INTPWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INTEN_INTPWM1_W<'a> {
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
#[doc = "Reader of field `PWM_INTEN_INTPWM2`"]
pub type PWM_INTEN_INTPWM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_INTEN_INTPWM2`"]
pub struct PWM_INTEN_INTPWM2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INTEN_INTPWM2_W<'a> {
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
#[doc = "Reader of field `PWM_INTEN_INTFAULT`"]
pub type PWM_INTEN_INTFAULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_INTEN_INTFAULT`"]
pub struct PWM_INTEN_INTFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INTEN_INTFAULT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PWM0 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm0(&self) -> PWM_INTEN_INTPWM0_R {
        PWM_INTEN_INTPWM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm1(&self) -> PWM_INTEN_INTPWM1_R {
        PWM_INTEN_INTPWM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm2(&self) -> PWM_INTEN_INTPWM2_R {
        PWM_INTEN_INTPWM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fault Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intfault(&self) -> PWM_INTEN_INTFAULT_R {
        PWM_INTEN_INTFAULT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm0(&mut self) -> PWM_INTEN_INTPWM0_W {
        PWM_INTEN_INTPWM0_W { w: self }
    }
    #[doc = "Bit 1 - PWM1 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm1(&mut self) -> PWM_INTEN_INTPWM1_W {
        PWM_INTEN_INTPWM1_W { w: self }
    }
    #[doc = "Bit 2 - PWM2 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm2(&mut self) -> PWM_INTEN_INTPWM2_W {
        PWM_INTEN_INTPWM2_W { w: self }
    }
    #[doc = "Bit 16 - Fault Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intfault(&mut self) -> PWM_INTEN_INTFAULT_W {
        PWM_INTEN_INTFAULT_W { w: self }
    }
}
