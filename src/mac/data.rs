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
#[doc = "Reader of field `MAC_DATA_TXDATA`"]
pub type MAC_DATA_TXDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MAC_DATA_TXDATA`"]
pub struct MAC_DATA_TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_DATA_TXDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmit FIFO Data"]
    #[inline(always)]
    pub fn mac_data_txdata(&self) -> MAC_DATA_TXDATA_R {
        MAC_DATA_TXDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit FIFO Data"]
    #[inline(always)]
    pub fn mac_data_txdata(&mut self) -> MAC_DATA_TXDATA_W {
        MAC_DATA_TXDATA_W { w: self }
    }
}
