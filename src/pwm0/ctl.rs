#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_CTL_GLOBALSYNC0`"]
pub type PWM_CTL_GLOBALSYNC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_CTL_GLOBALSYNC0`"]
pub struct PWM_CTL_GLOBALSYNC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_CTL_GLOBALSYNC0_W<'a> {
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
#[doc = "Reader of field `PWM_CTL_GLOBALSYNC1`"]
pub type PWM_CTL_GLOBALSYNC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_CTL_GLOBALSYNC1`"]
pub struct PWM_CTL_GLOBALSYNC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_CTL_GLOBALSYNC1_W<'a> {
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
#[doc = "Reader of field `PWM_CTL_GLOBALSYNC2`"]
pub type PWM_CTL_GLOBALSYNC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_CTL_GLOBALSYNC2`"]
pub struct PWM_CTL_GLOBALSYNC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_CTL_GLOBALSYNC2_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Update PWM Generator 0"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync0(&self) -> PWM_CTL_GLOBALSYNC0_R {
        PWM_CTL_GLOBALSYNC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Update PWM Generator 1"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync1(&self) -> PWM_CTL_GLOBALSYNC1_R {
        PWM_CTL_GLOBALSYNC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Update PWM Generator 2"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync2(&self) -> PWM_CTL_GLOBALSYNC2_R {
        PWM_CTL_GLOBALSYNC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update PWM Generator 0"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync0(&mut self) -> PWM_CTL_GLOBALSYNC0_W {
        PWM_CTL_GLOBALSYNC0_W { w: self }
    }
    #[doc = "Bit 1 - Update PWM Generator 1"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync1(&mut self) -> PWM_CTL_GLOBALSYNC1_W {
        PWM_CTL_GLOBALSYNC1_W { w: self }
    }
    #[doc = "Bit 2 - Update PWM Generator 2"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync2(&mut self) -> PWM_CTL_GLOBALSYNC2_W {
        PWM_CTL_GLOBALSYNC2_W { w: self }
    }
}
