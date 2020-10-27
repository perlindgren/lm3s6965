#[doc = "Reader of register RTCT"]
pub type R = crate::R<u32, super::RTCT>;
#[doc = "Writer for register RTCT"]
pub type W = crate::W<u32, super::RTCT>;
#[doc = "Register RTCT `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HIB_RTCT_TRIM`"]
pub type HIB_RTCT_TRIM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HIB_RTCT_TRIM`"]
pub struct HIB_RTCT_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_RTCT_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RTC Trim Value"]
    #[inline(always)]
    pub fn hib_rtct_trim(&self) -> HIB_RTCT_TRIM_R {
        HIB_RTCT_TRIM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC Trim Value"]
    #[inline(always)]
    pub fn hib_rtct_trim(&mut self) -> HIB_RTCT_TRIM_W {
        HIB_RTCT_TRIM_W { w: self }
    }
}
