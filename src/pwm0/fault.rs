#[doc = "Reader of register FAULT"]
pub type R = crate::R<u32, super::FAULT>;
#[doc = "Writer for register FAULT"]
pub type W = crate::W<u32, super::FAULT>;
#[doc = "Register FAULT `reset()`'s with value 0"]
impl crate::ResetValue for super::FAULT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_FAULT_FAULT0`"]
pub type PWM_FAULT_FAULT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_FAULT_FAULT0`"]
pub struct PWM_FAULT_FAULT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_FAULT_FAULT0_W<'a> {
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
#[doc = "Reader of field `PWM_FAULT_FAULT1`"]
pub type PWM_FAULT_FAULT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_FAULT_FAULT1`"]
pub struct PWM_FAULT_FAULT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_FAULT_FAULT1_W<'a> {
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
#[doc = "Reader of field `PWM_FAULT_FAULT2`"]
pub type PWM_FAULT_FAULT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_FAULT_FAULT2`"]
pub struct PWM_FAULT_FAULT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_FAULT_FAULT2_W<'a> {
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
#[doc = "Reader of field `PWM_FAULT_FAULT3`"]
pub type PWM_FAULT_FAULT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_FAULT_FAULT3`"]
pub struct PWM_FAULT_FAULT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_FAULT_FAULT3_W<'a> {
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
#[doc = "Reader of field `PWM_FAULT_FAULT4`"]
pub type PWM_FAULT_FAULT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_FAULT_FAULT4`"]
pub struct PWM_FAULT_FAULT4_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_FAULT_FAULT4_W<'a> {
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
#[doc = "Reader of field `PWM_FAULT_FAULT5`"]
pub type PWM_FAULT_FAULT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_FAULT_FAULT5`"]
pub struct PWM_FAULT_FAULT5_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_FAULT_FAULT5_W<'a> {
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
    #[doc = "Bit 0 - PWM0 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault0(&self) -> PWM_FAULT_FAULT0_R {
        PWM_FAULT_FAULT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM1 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault1(&self) -> PWM_FAULT_FAULT1_R {
        PWM_FAULT_FAULT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM2 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault2(&self) -> PWM_FAULT_FAULT2_R {
        PWM_FAULT_FAULT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM3 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault3(&self) -> PWM_FAULT_FAULT3_R {
        PWM_FAULT_FAULT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM4 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault4(&self) -> PWM_FAULT_FAULT4_R {
        PWM_FAULT_FAULT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM5 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault5(&self) -> PWM_FAULT_FAULT5_R {
        PWM_FAULT_FAULT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault0(&mut self) -> PWM_FAULT_FAULT0_W {
        PWM_FAULT_FAULT0_W { w: self }
    }
    #[doc = "Bit 1 - PWM1 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault1(&mut self) -> PWM_FAULT_FAULT1_W {
        PWM_FAULT_FAULT1_W { w: self }
    }
    #[doc = "Bit 2 - PWM2 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault2(&mut self) -> PWM_FAULT_FAULT2_W {
        PWM_FAULT_FAULT2_W { w: self }
    }
    #[doc = "Bit 3 - PWM3 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault3(&mut self) -> PWM_FAULT_FAULT3_W {
        PWM_FAULT_FAULT3_W { w: self }
    }
    #[doc = "Bit 4 - PWM4 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault4(&mut self) -> PWM_FAULT_FAULT4_W {
        PWM_FAULT_FAULT4_W { w: self }
    }
    #[doc = "Bit 5 - PWM5 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault5(&mut self) -> PWM_FAULT_FAULT5_W {
        PWM_FAULT_FAULT5_W { w: self }
    }
}
