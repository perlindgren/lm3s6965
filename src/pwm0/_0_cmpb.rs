#[doc = "Reader of register _0_CMPB"]
pub type R = crate::R<u32, super::_0_CMPB>;
#[doc = "Writer for register _0_CMPB"]
pub type W = crate::W<u32, super::_0_CMPB>;
#[doc = "Register _0_CMPB `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_CMPB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_X_CMPB`"]
pub type PWM_X_CMPB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PWM_X_CMPB`"]
pub struct PWM_X_CMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_CMPB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparator B Value"]
    #[inline(always)]
    pub fn pwm_x_cmpb(&self) -> PWM_X_CMPB_R {
        PWM_X_CMPB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparator B Value"]
    #[inline(always)]
    pub fn pwm_x_cmpb(&mut self) -> PWM_X_CMPB_W {
        PWM_X_CMPB_W { w: self }
    }
}
