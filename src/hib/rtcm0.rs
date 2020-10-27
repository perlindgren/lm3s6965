#[doc = "Reader of register RTCM0"]
pub type R = crate::R<u32, super::RTCM0>;
#[doc = "Writer for register RTCM0"]
pub type W = crate::W<u32, super::RTCM0>;
#[doc = "Register RTCM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HIB_RTCM0`"]
pub type HIB_RTCM0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HIB_RTCM0`"]
pub struct HIB_RTCM0_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_RTCM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RTC Match 0"]
    #[inline(always)]
    pub fn hib_rtcm0(&self) -> HIB_RTCM0_R {
        HIB_RTCM0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC Match 0"]
    #[inline(always)]
    pub fn hib_rtcm0(&mut self) -> HIB_RTCM0_W {
        HIB_RTCM0_W { w: self }
    }
}
