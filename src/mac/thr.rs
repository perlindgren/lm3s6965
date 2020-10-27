#[doc = "Reader of register THR"]
pub type R = crate::R<u32, super::THR>;
#[doc = "Writer for register THR"]
pub type W = crate::W<u32, super::THR>;
#[doc = "Register THR `reset()`'s with value 0"]
impl crate::ResetValue for super::THR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_THR_THRESH`"]
pub type MAC_THR_THRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAC_THR_THRESH`"]
pub struct MAC_THR_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_THR_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Threshold Value"]
    #[inline(always)]
    pub fn mac_thr_thresh(&self) -> MAC_THR_THRESH_R {
        MAC_THR_THRESH_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Threshold Value"]
    #[inline(always)]
    pub fn mac_thr_thresh(&mut self) -> MAC_THR_THRESH_W {
        MAC_THR_THRESH_W { w: self }
    }
}
