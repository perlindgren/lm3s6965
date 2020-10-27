#[doc = "Reader of register POS"]
pub type R = crate::R<u32, super::POS>;
#[doc = "Writer for register POS"]
pub type W = crate::W<u32, super::POS>;
#[doc = "Register POS `reset()`'s with value 0"]
impl crate::ResetValue for super::POS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QEI_POS`"]
pub type QEI_POS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QEI_POS`"]
pub struct QEI_POS_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_POS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Current Position Integrator Value"]
    #[inline(always)]
    pub fn qei_pos(&self) -> QEI_POS_R {
        QEI_POS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current Position Integrator Value"]
    #[inline(always)]
    pub fn qei_pos(&mut self) -> QEI_POS_W {
        QEI_POS_W { w: self }
    }
}
