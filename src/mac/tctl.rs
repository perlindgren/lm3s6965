#[doc = "Reader of register TCTL"]
pub type R = crate::R<u32, super::TCTL>;
#[doc = "Writer for register TCTL"]
pub type W = crate::W<u32, super::TCTL>;
#[doc = "Register TCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_TCTL_TXEN`"]
pub type MAC_TCTL_TXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_TCTL_TXEN`"]
pub struct MAC_TCTL_TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_TCTL_TXEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `MAC_TCTL_PADEN`"]
pub type MAC_TCTL_PADEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_TCTL_PADEN`"]
pub struct MAC_TCTL_PADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_TCTL_PADEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MAC_TCTL_CRC`"]
pub type MAC_TCTL_CRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_TCTL_CRC`"]
pub struct MAC_TCTL_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_TCTL_CRC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `MAC_TCTL_DUPLEX`"]
pub type MAC_TCTL_DUPLEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_TCTL_DUPLEX`"]
pub struct MAC_TCTL_DUPLEX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_TCTL_DUPLEX_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Transmitter"]
    #[inline(always)]
    pub fn mac_tctl_txen(&self) -> MAC_TCTL_TXEN_R {
        MAC_TCTL_TXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Packet Padding"]
    #[inline(always)]
    pub fn mac_tctl_paden(&self) -> MAC_TCTL_PADEN_R {
        MAC_TCTL_PADEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable CRC Generation"]
    #[inline(always)]
    pub fn mac_tctl_crc(&self) -> MAC_TCTL_CRC_R {
        MAC_TCTL_CRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Duplex Mode"]
    #[inline(always)]
    pub fn mac_tctl_duplex(&self) -> MAC_TCTL_DUPLEX_R {
        MAC_TCTL_DUPLEX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Transmitter"]
    #[inline(always)]
    pub fn mac_tctl_txen(&mut self) -> MAC_TCTL_TXEN_W {
        MAC_TCTL_TXEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable Packet Padding"]
    #[inline(always)]
    pub fn mac_tctl_paden(&mut self) -> MAC_TCTL_PADEN_W {
        MAC_TCTL_PADEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable CRC Generation"]
    #[inline(always)]
    pub fn mac_tctl_crc(&mut self) -> MAC_TCTL_CRC_W {
        MAC_TCTL_CRC_W { w: self }
    }
    #[doc = "Bit 4 - Enable Duplex Mode"]
    #[inline(always)]
    pub fn mac_tctl_duplex(&mut self) -> MAC_TCTL_DUPLEX_W {
        MAC_TCTL_DUPLEX_W { w: self }
    }
}
