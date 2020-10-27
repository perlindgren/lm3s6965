#[doc = "Reader of register _0_COUNT"]
pub type R = crate::R<u32, super::_0_COUNT>;
#[doc = "Writer for register _0_COUNT"]
pub type W = crate::W<u32, super::_0_COUNT>;
#[doc = "Register _0_COUNT `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_COUNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_X_COUNT`"]
pub type PWM_X_COUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PWM_X_COUNT`"]
pub struct PWM_X_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Counter Value"]
    #[inline(always)]
    pub fn pwm_x_count(&self) -> PWM_X_COUNT_R {
        PWM_X_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Value"]
    #[inline(always)]
    pub fn pwm_x_count(&mut self) -> PWM_X_COUNT_W {
        PWM_X_COUNT_W { w: self }
    }
}
