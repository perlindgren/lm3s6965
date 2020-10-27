#[doc = "Reader of register RTCC"]
pub type R = crate::R<u32, super::RTCC>;
#[doc = "Writer for register RTCC"]
pub type W = crate::W<u32, super::RTCC>;
#[doc = "Register RTCC `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HIB_RTCC`"]
pub type HIB_RTCC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HIB_RTCC`"]
pub struct HIB_RTCC_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_RTCC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RTC Counter"]
    #[inline(always)]
    pub fn hib_rtcc(&self) -> HIB_RTCC_R {
        HIB_RTCC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC Counter"]
    #[inline(always)]
    pub fn hib_rtcc(&mut self) -> HIB_RTCC_W {
        HIB_RTCC_W { w: self }
    }
}
