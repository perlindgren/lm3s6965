#[doc = "Reader of register _0_DBCTL"]
pub type R = crate::R<u32, super::_0_DBCTL>;
#[doc = "Writer for register _0_DBCTL"]
pub type W = crate::W<u32, super::_0_DBCTL>;
#[doc = "Register _0_DBCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_DBCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_X_DBCTL_ENABLE`"]
pub type PWM_X_DBCTL_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_DBCTL_ENABLE`"]
pub struct PWM_X_DBCTL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_DBCTL_ENABLE_W<'a> {
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
    #[doc = "Bit 0 - Dead-Band Generator Enable"]
    #[inline(always)]
    pub fn pwm_x_dbctl_enable(&self) -> PWM_X_DBCTL_ENABLE_R {
        PWM_X_DBCTL_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dead-Band Generator Enable"]
    #[inline(always)]
    pub fn pwm_x_dbctl_enable(&mut self) -> PWM_X_DBCTL_ENABLE_W {
        PWM_X_DBCTL_ENABLE_W { w: self }
    }
}
