#[doc = "Reader of register SPEED"]
pub type R = crate::R<u32, super::SPEED>;
#[doc = "Writer for register SPEED"]
pub type W = crate::W<u32, super::SPEED>;
#[doc = "Register SPEED `reset()`'s with value 0"]
impl crate::ResetValue for super::SPEED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QEI_SPEED`"]
pub type QEI_SPEED_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QEI_SPEED`"]
pub struct QEI_SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_SPEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Velocity"]
    #[inline(always)]
    pub fn qei_speed(&self) -> QEI_SPEED_R {
        QEI_SPEED_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Velocity"]
    #[inline(always)]
    pub fn qei_speed(&mut self) -> QEI_SPEED_W {
        QEI_SPEED_W { w: self }
    }
}
