#[doc = "Reader of register RTCM1"]
pub type R = crate::R<u32, super::RTCM1>;
#[doc = "Writer for register RTCM1"]
pub type W = crate::W<u32, super::RTCM1>;
#[doc = "Register RTCM1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCM1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HIB_RTCM1`"]
pub type HIB_RTCM1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HIB_RTCM1`"]
pub struct HIB_RTCM1_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_RTCM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RTC Match 1"]
    #[inline(always)]
    pub fn hib_rtcm1(&self) -> HIB_RTCM1_R {
        HIB_RTCM1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC Match 1"]
    #[inline(always)]
    pub fn hib_rtcm1(&mut self) -> HIB_RTCM1_W {
        HIB_RTCM1_W { w: self }
    }
}
