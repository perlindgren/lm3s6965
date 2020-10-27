#[doc = "Reader of register NP"]
pub type R = crate::R<u32, super::NP>;
#[doc = "Writer for register NP"]
pub type W = crate::W<u32, super::NP>;
#[doc = "Register NP `reset()`'s with value 0"]
impl crate::ResetValue for super::NP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_NP_NPR`"]
pub type MAC_NP_NPR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAC_NP_NPR`"]
pub struct MAC_NP_NPR_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_NP_NPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Number of Packets in Receive FIFO"]
    #[inline(always)]
    pub fn mac_np_npr(&self) -> MAC_NP_NPR_R {
        MAC_NP_NPR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of Packets in Receive FIFO"]
    #[inline(always)]
    pub fn mac_np_npr(&mut self) -> MAC_NP_NPR_W {
        MAC_NP_NPR_W { w: self }
    }
}
