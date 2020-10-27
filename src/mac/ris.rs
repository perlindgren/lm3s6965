#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Writer for register RIS"]
pub type W = crate::W<u32, super::RIS>;
#[doc = "Register RIS `reset()`'s with value 0"]
impl crate::ResetValue for super::RIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_RIS_RXINT`"]
pub type MAC_RIS_RXINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_RIS_RXINT`"]
pub struct MAC_RIS_RXINT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_RIS_RXINT_W<'a> {
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
#[doc = "Reader of field `MAC_RIS_TXER`"]
pub type MAC_RIS_TXER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_RIS_TXER`"]
pub struct MAC_RIS_TXER_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_RIS_TXER_W<'a> {
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
#[doc = "Reader of field `MAC_RIS_TXEMP`"]
pub type MAC_RIS_TXEMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_RIS_TXEMP`"]
pub struct MAC_RIS_TXEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_RIS_TXEMP_W<'a> {
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
#[doc = "Reader of field `MAC_RIS_FOV`"]
pub type MAC_RIS_FOV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_RIS_FOV`"]
pub struct MAC_RIS_FOV_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_RIS_FOV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `MAC_RIS_RXER`"]
pub type MAC_RIS_RXER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_RIS_RXER`"]
pub struct MAC_RIS_RXER_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_RIS_RXER_W<'a> {
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
#[doc = "Reader of field `MAC_RIS_MDINT`"]
pub type MAC_RIS_MDINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_RIS_MDINT`"]
pub struct MAC_RIS_MDINT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_RIS_MDINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `MAC_RIS_PHYINT`"]
pub type MAC_RIS_PHYINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_RIS_PHYINT`"]
pub struct MAC_RIS_PHYINT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_RIS_PHYINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Packet Received"]
    #[inline(always)]
    pub fn mac_ris_rxint(&self) -> MAC_RIS_RXINT_R {
        MAC_RIS_RXINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Error"]
    #[inline(always)]
    pub fn mac_ris_txer(&self) -> MAC_RIS_TXER_R {
        MAC_RIS_TXER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Empty"]
    #[inline(always)]
    pub fn mac_ris_txemp(&self) -> MAC_RIS_TXEMP_R {
        MAC_RIS_TXEMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO Overrun"]
    #[inline(always)]
    pub fn mac_ris_fov(&self) -> MAC_RIS_FOV_R {
        MAC_RIS_FOV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Error"]
    #[inline(always)]
    pub fn mac_ris_rxer(&self) -> MAC_RIS_RXER_R {
        MAC_RIS_RXER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MII Transaction Complete"]
    #[inline(always)]
    pub fn mac_ris_mdint(&self) -> MAC_RIS_MDINT_R {
        MAC_RIS_MDINT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PHY Interrupt"]
    #[inline(always)]
    pub fn mac_ris_phyint(&self) -> MAC_RIS_PHYINT_R {
        MAC_RIS_PHYINT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Packet Received"]
    #[inline(always)]
    pub fn mac_ris_rxint(&mut self) -> MAC_RIS_RXINT_W {
        MAC_RIS_RXINT_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Error"]
    #[inline(always)]
    pub fn mac_ris_txer(&mut self) -> MAC_RIS_TXER_W {
        MAC_RIS_TXER_W { w: self }
    }
    #[doc = "Bit 2 - Transmit FIFO Empty"]
    #[inline(always)]
    pub fn mac_ris_txemp(&mut self) -> MAC_RIS_TXEMP_W {
        MAC_RIS_TXEMP_W { w: self }
    }
    #[doc = "Bit 3 - FIFO Overrun"]
    #[inline(always)]
    pub fn mac_ris_fov(&mut self) -> MAC_RIS_FOV_W {
        MAC_RIS_FOV_W { w: self }
    }
    #[doc = "Bit 4 - Receive Error"]
    #[inline(always)]
    pub fn mac_ris_rxer(&mut self) -> MAC_RIS_RXER_W {
        MAC_RIS_RXER_W { w: self }
    }
    #[doc = "Bit 5 - MII Transaction Complete"]
    #[inline(always)]
    pub fn mac_ris_mdint(&mut self) -> MAC_RIS_MDINT_W {
        MAC_RIS_MDINT_W { w: self }
    }
    #[doc = "Bit 6 - PHY Interrupt"]
    #[inline(always)]
    pub fn mac_ris_phyint(&mut self) -> MAC_RIS_PHYINT_W {
        MAC_RIS_PHYINT_W { w: self }
    }
}
