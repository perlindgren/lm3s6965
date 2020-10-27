#[doc = "Reader of register SYNC"]
pub type R = crate::R<u32, super::SYNC>;
#[doc = "Writer for register SYNC"]
pub type W = crate::W<u32, super::SYNC>;
#[doc = "Register SYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_SYNC_SYNC0`"]
pub type PWM_SYNC_SYNC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_SYNC_SYNC0`"]
pub struct PWM_SYNC_SYNC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_SYNC_SYNC0_W<'a> {
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
#[doc = "Reader of field `PWM_SYNC_SYNC1`"]
pub type PWM_SYNC_SYNC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_SYNC_SYNC1`"]
pub struct PWM_SYNC_SYNC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_SYNC_SYNC1_W<'a> {
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
#[doc = "Reader of field `PWM_SYNC_SYNC2`"]
pub type PWM_SYNC_SYNC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_SYNC_SYNC2`"]
pub struct PWM_SYNC_SYNC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_SYNC_SYNC2_W<'a> {
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
    #[doc = "Bit 0 - Reset Generator 0 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync0(&self) -> PWM_SYNC_SYNC0_R {
        PWM_SYNC_SYNC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset Generator 1 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync1(&self) -> PWM_SYNC_SYNC1_R {
        PWM_SYNC_SYNC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset Generator 2 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync2(&self) -> PWM_SYNC_SYNC2_R {
        PWM_SYNC_SYNC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Generator 0 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync0(&mut self) -> PWM_SYNC_SYNC0_W {
        PWM_SYNC_SYNC0_W { w: self }
    }
    #[doc = "Bit 1 - Reset Generator 1 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync1(&mut self) -> PWM_SYNC_SYNC1_W {
        PWM_SYNC_SYNC1_W { w: self }
    }
    #[doc = "Bit 2 - Reset Generator 2 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync2(&mut self) -> PWM_SYNC_SYNC2_W {
        PWM_SYNC_SYNC2_W { w: self }
    }
}
