#[doc = "Reader of register RTCLD"]
pub type R = crate::R<u32, super::RTCLD>;
#[doc = "Writer for register RTCLD"]
pub type W = crate::W<u32, super::RTCLD>;
#[doc = "Register RTCLD `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HIB_RTCLD`"]
pub type HIB_RTCLD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HIB_RTCLD`"]
pub struct HIB_RTCLD_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_RTCLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RTC Load"]
    #[inline(always)]
    pub fn hib_rtcld(&self) -> HIB_RTCLD_R {
        HIB_RTCLD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC Load"]
    #[inline(always)]
    pub fn hib_rtcld(&mut self) -> HIB_RTCLD_W {
        HIB_RTCLD_W { w: self }
    }
}
