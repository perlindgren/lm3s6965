#[doc = "Reader of register MRXD"]
pub type R = crate::R<u32, super::MRXD>;
#[doc = "Writer for register MRXD"]
pub type W = crate::W<u32, super::MRXD>;
#[doc = "Register MRXD `reset()`'s with value 0"]
impl crate::ResetValue for super::MRXD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_MRXD_MDRX`"]
pub type MAC_MRXD_MDRX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAC_MRXD_MDRX`"]
pub struct MAC_MRXD_MDRX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_MRXD_MDRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MII Register Receive Data"]
    #[inline(always)]
    pub fn mac_mrxd_mdrx(&self) -> MAC_MRXD_MDRX_R {
        MAC_MRXD_MDRX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII Register Receive Data"]
    #[inline(always)]
    pub fn mac_mrxd_mdrx(&mut self) -> MAC_MRXD_MDRX_W {
        MAC_MRXD_MDRX_W { w: self }
    }
}
