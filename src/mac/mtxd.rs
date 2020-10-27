#[doc = "Reader of register MTXD"]
pub type R = crate::R<u32, super::MTXD>;
#[doc = "Writer for register MTXD"]
pub type W = crate::W<u32, super::MTXD>;
#[doc = "Register MTXD `reset()`'s with value 0"]
impl crate::ResetValue for super::MTXD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_MTXD_MDTX`"]
pub type MAC_MTXD_MDTX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAC_MTXD_MDTX`"]
pub struct MAC_MTXD_MDTX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_MTXD_MDTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MII Register Transmit Data"]
    #[inline(always)]
    pub fn mac_mtxd_mdtx(&self) -> MAC_MTXD_MDTX_R {
        MAC_MTXD_MDTX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII Register Transmit Data"]
    #[inline(always)]
    pub fn mac_mtxd_mdtx(&mut self) -> MAC_MTXD_MDTX_W {
        MAC_MTXD_MDTX_W { w: self }
    }
}
