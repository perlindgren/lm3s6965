#[doc = "Reader of register _0_DBFALL"]
pub type R = crate::R<u32, super::_0_DBFALL>;
#[doc = "Writer for register _0_DBFALL"]
pub type W = crate::W<u32, super::_0_DBFALL>;
#[doc = "Register _0_DBFALL `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_DBFALL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_X_DBFALL_DELAY`"]
pub type PWM_X_DBFALL_DELAY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PWM_X_DBFALL_DELAY`"]
pub struct PWM_X_DBFALL_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_DBFALL_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Dead-Band Fall Delay"]
    #[inline(always)]
    pub fn pwm_x_dbfall_delay(&self) -> PWM_X_DBFALL_DELAY_R {
        PWM_X_DBFALL_DELAY_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Dead-Band Fall Delay"]
    #[inline(always)]
    pub fn pwm_x_dbfall_delay(&mut self) -> PWM_X_DBFALL_DELAY_W {
        PWM_X_DBFALL_DELAY_W { w: self }
    }
}
