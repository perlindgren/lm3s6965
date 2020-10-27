#[doc = "Reader of register _0_CTL"]
pub type R = crate::R<u32, super::_0_CTL>;
#[doc = "Writer for register _0_CTL"]
pub type W = crate::W<u32, super::_0_CTL>;
#[doc = "Register _0_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_X_CTL_ENABLE`"]
pub type PWM_X_CTL_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_CTL_ENABLE`"]
pub struct PWM_X_CTL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_CTL_ENABLE_W<'a> {
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
#[doc = "Reader of field `PWM_X_CTL_MODE`"]
pub type PWM_X_CTL_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_CTL_MODE`"]
pub struct PWM_X_CTL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_CTL_MODE_W<'a> {
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
#[doc = "Reader of field `PWM_X_CTL_DEBUG`"]
pub type PWM_X_CTL_DEBUG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_CTL_DEBUG`"]
pub struct PWM_X_CTL_DEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_CTL_DEBUG_W<'a> {
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
#[doc = "Reader of field `PWM_X_CTL_LOADUPD`"]
pub type PWM_X_CTL_LOADUPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_CTL_LOADUPD`"]
pub struct PWM_X_CTL_LOADUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_CTL_LOADUPD_W<'a> {
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
#[doc = "Reader of field `PWM_X_CTL_CMPAUPD`"]
pub type PWM_X_CTL_CMPAUPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_CTL_CMPAUPD`"]
pub struct PWM_X_CTL_CMPAUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_CTL_CMPAUPD_W<'a> {
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
#[doc = "Reader of field `PWM_X_CTL_CMPBUPD`"]
pub type PWM_X_CTL_CMPBUPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_CTL_CMPBUPD`"]
pub struct PWM_X_CTL_CMPBUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_CTL_CMPBUPD_W<'a> {
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
    #[doc = "Bit 0 - PWM Block Enable"]
    #[inline(always)]
    pub fn pwm_x_ctl_enable(&self) -> PWM_X_CTL_ENABLE_R {
        PWM_X_CTL_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter Mode"]
    #[inline(always)]
    pub fn pwm_x_ctl_mode(&self) -> PWM_X_CTL_MODE_R {
        PWM_X_CTL_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Debug Mode"]
    #[inline(always)]
    pub fn pwm_x_ctl_debug(&self) -> PWM_X_CTL_DEBUG_R {
        PWM_X_CTL_DEBUG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Load Register Update Mode"]
    #[inline(always)]
    pub fn pwm_x_ctl_loadupd(&self) -> PWM_X_CTL_LOADUPD_R {
        PWM_X_CTL_LOADUPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator A Update Mode"]
    #[inline(always)]
    pub fn pwm_x_ctl_cmpaupd(&self) -> PWM_X_CTL_CMPAUPD_R {
        PWM_X_CTL_CMPAUPD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comparator B Update Mode"]
    #[inline(always)]
    pub fn pwm_x_ctl_cmpbupd(&self) -> PWM_X_CTL_CMPBUPD_R {
        PWM_X_CTL_CMPBUPD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Block Enable"]
    #[inline(always)]
    pub fn pwm_x_ctl_enable(&mut self) -> PWM_X_CTL_ENABLE_W {
        PWM_X_CTL_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Counter Mode"]
    #[inline(always)]
    pub fn pwm_x_ctl_mode(&mut self) -> PWM_X_CTL_MODE_W {
        PWM_X_CTL_MODE_W { w: self }
    }
    #[doc = "Bit 2 - Debug Mode"]
    #[inline(always)]
    pub fn pwm_x_ctl_debug(&mut self) -> PWM_X_CTL_DEBUG_W {
        PWM_X_CTL_DEBUG_W { w: self }
    }
    #[doc = "Bit 3 - Load Register Update Mode"]
    #[inline(always)]
    pub fn pwm_x_ctl_loadupd(&mut self) -> PWM_X_CTL_LOADUPD_W {
        PWM_X_CTL_LOADUPD_W { w: self }
    }
    #[doc = "Bit 4 - Comparator A Update Mode"]
    #[inline(always)]
    pub fn pwm_x_ctl_cmpaupd(&mut self) -> PWM_X_CTL_CMPAUPD_W {
        PWM_X_CTL_CMPAUPD_W { w: self }
    }
    #[doc = "Bit 5 - Comparator B Update Mode"]
    #[inline(always)]
    pub fn pwm_x_ctl_cmpbupd(&mut self) -> PWM_X_CTL_CMPBUPD_W {
        PWM_X_CTL_CMPBUPD_W { w: self }
    }
}
