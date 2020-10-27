#[doc = "Reader of register _0_RIS"]
pub type R = crate::R<u32, super::_0_RIS>;
#[doc = "Writer for register _0_RIS"]
pub type W = crate::W<u32, super::_0_RIS>;
#[doc = "Register _0_RIS `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_RIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_X_RIS_INTCNTZERO`"]
pub type PWM_X_RIS_INTCNTZERO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_RIS_INTCNTZERO`"]
pub struct PWM_X_RIS_INTCNTZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_RIS_INTCNTZERO_W<'a> {
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
#[doc = "Reader of field `PWM_X_RIS_INTCNTLOAD`"]
pub type PWM_X_RIS_INTCNTLOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_RIS_INTCNTLOAD`"]
pub struct PWM_X_RIS_INTCNTLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_RIS_INTCNTLOAD_W<'a> {
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
#[doc = "Reader of field `PWM_X_RIS_INTCMPAU`"]
pub type PWM_X_RIS_INTCMPAU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_RIS_INTCMPAU`"]
pub struct PWM_X_RIS_INTCMPAU_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_RIS_INTCMPAU_W<'a> {
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
#[doc = "Reader of field `PWM_X_RIS_INTCMPAD`"]
pub type PWM_X_RIS_INTCMPAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_RIS_INTCMPAD`"]
pub struct PWM_X_RIS_INTCMPAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_RIS_INTCMPAD_W<'a> {
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
#[doc = "Reader of field `PWM_X_RIS_INTCMPBU`"]
pub type PWM_X_RIS_INTCMPBU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_RIS_INTCMPBU`"]
pub struct PWM_X_RIS_INTCMPBU_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_RIS_INTCMPBU_W<'a> {
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
#[doc = "Reader of field `PWM_X_RIS_INTCMPBD`"]
pub type PWM_X_RIS_INTCMPBD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_RIS_INTCMPBD`"]
pub struct PWM_X_RIS_INTCMPBD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_RIS_INTCMPBD_W<'a> {
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
    #[doc = "Bit 0 - Counter=0 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_x_ris_intcntzero(&self) -> PWM_X_RIS_INTCNTZERO_R {
        PWM_X_RIS_INTCNTZERO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter=Load Interrupt Status"]
    #[inline(always)]
    pub fn pwm_x_ris_intcntload(&self) -> PWM_X_RIS_INTCNTLOAD_R {
        PWM_X_RIS_INTCNTLOAD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator A Up Interrupt Status"]
    #[inline(always)]
    pub fn pwm_x_ris_intcmpau(&self) -> PWM_X_RIS_INTCMPAU_R {
        PWM_X_RIS_INTCMPAU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparator A Down Interrupt Status"]
    #[inline(always)]
    pub fn pwm_x_ris_intcmpad(&self) -> PWM_X_RIS_INTCMPAD_R {
        PWM_X_RIS_INTCMPAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator B Up Interrupt Status"]
    #[inline(always)]
    pub fn pwm_x_ris_intcmpbu(&self) -> PWM_X_RIS_INTCMPBU_R {
        PWM_X_RIS_INTCMPBU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comparator B Down Interrupt Status"]
    #[inline(always)]
    pub fn pwm_x_ris_intcmpbd(&self) -> PWM_X_RIS_INTCMPBD_R {
        PWM_X_RIS_INTCMPBD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter=0 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_x_ris_intcntzero(&mut self) -> PWM_X_RIS_INTCNTZERO_W {
        PWM_X_RIS_INTCNTZERO_W { w: self }
    }
    #[doc = "Bit 1 - Counter=Load Interrupt Status"]
    #[inline(always)]
    pub fn pwm_x_ris_intcntload(&mut self) -> PWM_X_RIS_INTCNTLOAD_W {
        PWM_X_RIS_INTCNTLOAD_W { w: self }
    }
    #[doc = "Bit 2 - Comparator A Up Interrupt Status"]
    #[inline(always)]
    pub fn pwm_x_ris_intcmpau(&mut self) -> PWM_X_RIS_INTCMPAU_W {
        PWM_X_RIS_INTCMPAU_W { w: self }
    }
    #[doc = "Bit 3 - Comparator A Down Interrupt Status"]
    #[inline(always)]
    pub fn pwm_x_ris_intcmpad(&mut self) -> PWM_X_RIS_INTCMPAD_W {
        PWM_X_RIS_INTCMPAD_W { w: self }
    }
    #[doc = "Bit 4 - Comparator B Up Interrupt Status"]
    #[inline(always)]
    pub fn pwm_x_ris_intcmpbu(&mut self) -> PWM_X_RIS_INTCMPBU_W {
        PWM_X_RIS_INTCMPBU_W { w: self }
    }
    #[doc = "Bit 5 - Comparator B Down Interrupt Status"]
    #[inline(always)]
    pub fn pwm_x_ris_intcmpbd(&mut self) -> PWM_X_RIS_INTCMPBD_W {
        PWM_X_RIS_INTCMPBD_W { w: self }
    }
}
