#[doc = "Reader of register ENABLE"]
pub type R = crate::R<u32, super::ENABLE>;
#[doc = "Writer for register ENABLE"]
pub type W = crate::W<u32, super::ENABLE>;
#[doc = "Register ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_ENABLE_PWM0EN`"]
pub type PWM_ENABLE_PWM0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_ENABLE_PWM0EN`"]
pub struct PWM_ENABLE_PWM0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_ENABLE_PWM0EN_W<'a> {
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
#[doc = "Reader of field `PWM_ENABLE_PWM1EN`"]
pub type PWM_ENABLE_PWM1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_ENABLE_PWM1EN`"]
pub struct PWM_ENABLE_PWM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_ENABLE_PWM1EN_W<'a> {
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
#[doc = "Reader of field `PWM_ENABLE_PWM2EN`"]
pub type PWM_ENABLE_PWM2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_ENABLE_PWM2EN`"]
pub struct PWM_ENABLE_PWM2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_ENABLE_PWM2EN_W<'a> {
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
#[doc = "Reader of field `PWM_ENABLE_PWM3EN`"]
pub type PWM_ENABLE_PWM3EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_ENABLE_PWM3EN`"]
pub struct PWM_ENABLE_PWM3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_ENABLE_PWM3EN_W<'a> {
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
#[doc = "Reader of field `PWM_ENABLE_PWM4EN`"]
pub type PWM_ENABLE_PWM4EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_ENABLE_PWM4EN`"]
pub struct PWM_ENABLE_PWM4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_ENABLE_PWM4EN_W<'a> {
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
#[doc = "Reader of field `PWM_ENABLE_PWM5EN`"]
pub type PWM_ENABLE_PWM5EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_ENABLE_PWM5EN`"]
pub struct PWM_ENABLE_PWM5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_ENABLE_PWM5EN_W<'a> {
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
    #[doc = "Bit 0 - PWM0 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm0en(&self) -> PWM_ENABLE_PWM0EN_R {
        PWM_ENABLE_PWM0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM1 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm1en(&self) -> PWM_ENABLE_PWM1EN_R {
        PWM_ENABLE_PWM1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM2 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm2en(&self) -> PWM_ENABLE_PWM2EN_R {
        PWM_ENABLE_PWM2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM3 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm3en(&self) -> PWM_ENABLE_PWM3EN_R {
        PWM_ENABLE_PWM3EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM4 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm4en(&self) -> PWM_ENABLE_PWM4EN_R {
        PWM_ENABLE_PWM4EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM5 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm5en(&self) -> PWM_ENABLE_PWM5EN_R {
        PWM_ENABLE_PWM5EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm0en(&mut self) -> PWM_ENABLE_PWM0EN_W {
        PWM_ENABLE_PWM0EN_W { w: self }
    }
    #[doc = "Bit 1 - PWM1 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm1en(&mut self) -> PWM_ENABLE_PWM1EN_W {
        PWM_ENABLE_PWM1EN_W { w: self }
    }
    #[doc = "Bit 2 - PWM2 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm2en(&mut self) -> PWM_ENABLE_PWM2EN_W {
        PWM_ENABLE_PWM2EN_W { w: self }
    }
    #[doc = "Bit 3 - PWM3 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm3en(&mut self) -> PWM_ENABLE_PWM3EN_W {
        PWM_ENABLE_PWM3EN_W { w: self }
    }
    #[doc = "Bit 4 - PWM4 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm4en(&mut self) -> PWM_ENABLE_PWM4EN_W {
        PWM_ENABLE_PWM4EN_W { w: self }
    }
    #[doc = "Bit 5 - PWM5 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm5en(&mut self) -> PWM_ENABLE_PWM5EN_W {
        PWM_ENABLE_PWM5EN_W { w: self }
    }
}
