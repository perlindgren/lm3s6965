#[doc = "Reader of register _0_INTEN"]
pub type R = crate::R<u32, super::_0_INTEN>;
#[doc = "Writer for register _0_INTEN"]
pub type W = crate::W<u32, super::_0_INTEN>;
#[doc = "Register _0_INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_X_INTEN_INTCNTZERO`"]
pub type PWM_X_INTEN_INTCNTZERO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_INTEN_INTCNTZERO`"]
pub struct PWM_X_INTEN_INTCNTZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_INTEN_INTCNTZERO_W<'a> {
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
#[doc = "Reader of field `PWM_X_INTEN_INTCNTLOAD`"]
pub type PWM_X_INTEN_INTCNTLOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_INTEN_INTCNTLOAD`"]
pub struct PWM_X_INTEN_INTCNTLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_INTEN_INTCNTLOAD_W<'a> {
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
#[doc = "Reader of field `PWM_X_INTEN_INTCMPAU`"]
pub type PWM_X_INTEN_INTCMPAU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_INTEN_INTCMPAU`"]
pub struct PWM_X_INTEN_INTCMPAU_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_INTEN_INTCMPAU_W<'a> {
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
#[doc = "Reader of field `PWM_X_INTEN_INTCMPAD`"]
pub type PWM_X_INTEN_INTCMPAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_INTEN_INTCMPAD`"]
pub struct PWM_X_INTEN_INTCMPAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_INTEN_INTCMPAD_W<'a> {
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
#[doc = "Reader of field `PWM_X_INTEN_INTCMPBU`"]
pub type PWM_X_INTEN_INTCMPBU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_INTEN_INTCMPBU`"]
pub struct PWM_X_INTEN_INTCMPBU_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_INTEN_INTCMPBU_W<'a> {
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
#[doc = "Reader of field `PWM_X_INTEN_INTCMPBD`"]
pub type PWM_X_INTEN_INTCMPBD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_INTEN_INTCMPBD`"]
pub struct PWM_X_INTEN_INTCMPBD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_INTEN_INTCMPBD_W<'a> {
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
#[doc = "Reader of field `PWM_X_INTEN_TRCNTZERO`"]
pub type PWM_X_INTEN_TRCNTZERO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_INTEN_TRCNTZERO`"]
pub struct PWM_X_INTEN_TRCNTZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_INTEN_TRCNTZERO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PWM_X_INTEN_TRCNTLOAD`"]
pub type PWM_X_INTEN_TRCNTLOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_INTEN_TRCNTLOAD`"]
pub struct PWM_X_INTEN_TRCNTLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_INTEN_TRCNTLOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PWM_X_INTEN_TRCMPAU`"]
pub type PWM_X_INTEN_TRCMPAU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_INTEN_TRCMPAU`"]
pub struct PWM_X_INTEN_TRCMPAU_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_INTEN_TRCMPAU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `PWM_X_INTEN_TRCMPAD`"]
pub type PWM_X_INTEN_TRCMPAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_INTEN_TRCMPAD`"]
pub struct PWM_X_INTEN_TRCMPAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_INTEN_TRCMPAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PWM_X_INTEN_TRCMPBU`"]
pub type PWM_X_INTEN_TRCMPBU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_INTEN_TRCMPBU`"]
pub struct PWM_X_INTEN_TRCMPBU_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_INTEN_TRCMPBU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PWM_X_INTEN_TRCMPBD`"]
pub type PWM_X_INTEN_TRCMPBD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_X_INTEN_TRCMPBD`"]
pub struct PWM_X_INTEN_TRCMPBD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_INTEN_TRCMPBD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt for Counter=0"]
    #[inline(always)]
    pub fn pwm_x_inten_intcntzero(&self) -> PWM_X_INTEN_INTCNTZERO_R {
        PWM_X_INTEN_INTCNTZERO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn pwm_x_inten_intcntload(&self) -> PWM_X_INTEN_INTCNTLOAD_R {
        PWM_X_INTEN_INTCNTLOAD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn pwm_x_inten_intcmpau(&self) -> PWM_X_INTEN_INTCMPAU_R {
        PWM_X_INTEN_INTCMPAU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn pwm_x_inten_intcmpad(&self) -> PWM_X_INTEN_INTCMPAD_R {
        PWM_X_INTEN_INTCMPAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn pwm_x_inten_intcmpbu(&self) -> PWM_X_INTEN_INTCMPBU_R {
        PWM_X_INTEN_INTCMPBU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn pwm_x_inten_intcmpbd(&self) -> PWM_X_INTEN_INTCMPBD_R {
        PWM_X_INTEN_INTCMPBD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Trigger for Counter=0"]
    #[inline(always)]
    pub fn pwm_x_inten_trcntzero(&self) -> PWM_X_INTEN_TRCNTZERO_R {
        PWM_X_INTEN_TRCNTZERO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Trigger for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn pwm_x_inten_trcntload(&self) -> PWM_X_INTEN_TRCNTLOAD_R {
        PWM_X_INTEN_TRCNTLOAD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Trigger for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn pwm_x_inten_trcmpau(&self) -> PWM_X_INTEN_TRCMPAU_R {
        PWM_X_INTEN_TRCMPAU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Trigger for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn pwm_x_inten_trcmpad(&self) -> PWM_X_INTEN_TRCMPAD_R {
        PWM_X_INTEN_TRCMPAD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Trigger for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn pwm_x_inten_trcmpbu(&self) -> PWM_X_INTEN_TRCMPBU_R {
        PWM_X_INTEN_TRCMPBU_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Trigger for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn pwm_x_inten_trcmpbd(&self) -> PWM_X_INTEN_TRCMPBD_R {
        PWM_X_INTEN_TRCMPBD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt for Counter=0"]
    #[inline(always)]
    pub fn pwm_x_inten_intcntzero(&mut self) -> PWM_X_INTEN_INTCNTZERO_W {
        PWM_X_INTEN_INTCNTZERO_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn pwm_x_inten_intcntload(&mut self) -> PWM_X_INTEN_INTCNTLOAD_W {
        PWM_X_INTEN_INTCNTLOAD_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn pwm_x_inten_intcmpau(&mut self) -> PWM_X_INTEN_INTCMPAU_W {
        PWM_X_INTEN_INTCMPAU_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn pwm_x_inten_intcmpad(&mut self) -> PWM_X_INTEN_INTCMPAD_W {
        PWM_X_INTEN_INTCMPAD_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn pwm_x_inten_intcmpbu(&mut self) -> PWM_X_INTEN_INTCMPBU_W {
        PWM_X_INTEN_INTCMPBU_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn pwm_x_inten_intcmpbd(&mut self) -> PWM_X_INTEN_INTCMPBD_W {
        PWM_X_INTEN_INTCMPBD_W { w: self }
    }
    #[doc = "Bit 8 - Trigger for Counter=0"]
    #[inline(always)]
    pub fn pwm_x_inten_trcntzero(&mut self) -> PWM_X_INTEN_TRCNTZERO_W {
        PWM_X_INTEN_TRCNTZERO_W { w: self }
    }
    #[doc = "Bit 9 - Trigger for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn pwm_x_inten_trcntload(&mut self) -> PWM_X_INTEN_TRCNTLOAD_W {
        PWM_X_INTEN_TRCNTLOAD_W { w: self }
    }
    #[doc = "Bit 10 - Trigger for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn pwm_x_inten_trcmpau(&mut self) -> PWM_X_INTEN_TRCMPAU_W {
        PWM_X_INTEN_TRCMPAU_W { w: self }
    }
    #[doc = "Bit 11 - Trigger for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn pwm_x_inten_trcmpad(&mut self) -> PWM_X_INTEN_TRCMPAD_W {
        PWM_X_INTEN_TRCMPAD_W { w: self }
    }
    #[doc = "Bit 12 - Trigger for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn pwm_x_inten_trcmpbu(&mut self) -> PWM_X_INTEN_TRCMPBU_W {
        PWM_X_INTEN_TRCMPBU_W { w: self }
    }
    #[doc = "Bit 13 - Trigger for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn pwm_x_inten_trcmpbd(&mut self) -> PWM_X_INTEN_TRCMPBD_W {
        PWM_X_INTEN_TRCMPBD_W { w: self }
    }
}
