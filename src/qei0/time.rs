#[doc = "Reader of register TIME"]
pub type R = crate::R<u32, super::TIME>;
#[doc = "Writer for register TIME"]
pub type W = crate::W<u32, super::TIME>;
#[doc = "Register TIME `reset()`'s with value 0"]
impl crate::ResetValue for super::TIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QEI_TIME`"]
pub type QEI_TIME_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QEI_TIME`"]
pub struct QEI_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Velocity Timer Current Value"]
    #[inline(always)]
    pub fn qei_time(&self) -> QEI_TIME_R {
        QEI_TIME_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Velocity Timer Current Value"]
    #[inline(always)]
    pub fn qei_time(&mut self) -> QEI_TIME_W {
        QEI_TIME_W { w: self }
    }
}
