#[doc = "Reader of register DATA"]
pub type R = crate::R<u32, super::DATA>;
#[doc = "Writer for register DATA"]
pub type W = crate::W<u32, super::DATA>;
#[doc = "Register DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HIB_DATA_RTD`"]
pub type HIB_DATA_RTD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HIB_DATA_RTD`"]
pub struct HIB_DATA_RTD_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_DATA_RTD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Hibernation Module NV Data"]
    #[inline(always)]
    pub fn hib_data_rtd(&self) -> HIB_DATA_RTD_R {
        HIB_DATA_RTD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hibernation Module NV Data"]
    #[inline(always)]
    pub fn hib_data_rtd(&mut self) -> HIB_DATA_RTD_W {
        HIB_DATA_RTD_W { w: self }
    }
}
