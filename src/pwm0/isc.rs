#[doc = "Reader of register ISC"]
pub type R = crate::R<u32, super::ISC>;
#[doc = "Writer for register ISC"]
pub type W = crate::W<u32, super::ISC>;
#[doc = "Register ISC `reset()`'s with value 0"]
impl crate::ResetValue for super::ISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_ISC_INTPWM0`"]
pub type PWM_ISC_INTPWM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_ISC_INTPWM0`"]
pub struct PWM_ISC_INTPWM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_ISC_INTPWM0_W<'a> {
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
#[doc = "Reader of field `PWM_ISC_INTPWM1`"]
pub type PWM_ISC_INTPWM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_ISC_INTPWM1`"]
pub struct PWM_ISC_INTPWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_ISC_INTPWM1_W<'a> {
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
#[doc = "Reader of field `PWM_ISC_INTPWM2`"]
pub type PWM_ISC_INTPWM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_ISC_INTPWM2`"]
pub struct PWM_ISC_INTPWM2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_ISC_INTPWM2_W<'a> {
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
#[doc = "Reader of field `PWM_ISC_INTFAULT`"]
pub type PWM_ISC_INTFAULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_ISC_INTFAULT`"]
pub struct PWM_ISC_INTFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_ISC_INTFAULT_W<'a> {
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
    #[doc = "Bit 0 - PWM0 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm0(&self) -> PWM_ISC_INTPWM0_R {
        PWM_ISC_INTPWM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm1(&self) -> PWM_ISC_INTPWM1_R {
        PWM_ISC_INTPWM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm2(&self) -> PWM_ISC_INTPWM2_R {
        PWM_ISC_INTPWM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fault Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_isc_intfault(&self) -> PWM_ISC_INTFAULT_R {
        PWM_ISC_INTFAULT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm0(&mut self) -> PWM_ISC_INTPWM0_W {
        PWM_ISC_INTPWM0_W { w: self }
    }
    #[doc = "Bit 1 - PWM1 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm1(&mut self) -> PWM_ISC_INTPWM1_W {
        PWM_ISC_INTPWM1_W { w: self }
    }
    #[doc = "Bit 2 - PWM2 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm2(&mut self) -> PWM_ISC_INTPWM2_W {
        PWM_ISC_INTPWM2_W { w: self }
    }
    #[doc = "Bit 16 - Fault Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_isc_intfault(&mut self) -> PWM_ISC_INTFAULT_W {
        PWM_ISC_INTFAULT_W { w: self }
    }
}
