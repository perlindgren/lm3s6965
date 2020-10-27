#[doc = "Reader of register IACK"]
pub type R = crate::R<u32, super::IACK>;
#[doc = "Writer for register IACK"]
pub type W = crate::W<u32, super::IACK>;
#[doc = "Register IACK `reset()`'s with value 0"]
impl crate::ResetValue for super::IACK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_IACK_RXINT`"]
pub type MAC_IACK_RXINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IACK_RXINT`"]
pub struct MAC_IACK_RXINT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IACK_RXINT_W<'a> {
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
#[doc = "Reader of field `MAC_IACK_TXER`"]
pub type MAC_IACK_TXER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IACK_TXER`"]
pub struct MAC_IACK_TXER_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IACK_TXER_W<'a> {
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
#[doc = "Reader of field `MAC_IACK_TXEMP`"]
pub type MAC_IACK_TXEMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IACK_TXEMP`"]
pub struct MAC_IACK_TXEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IACK_TXEMP_W<'a> {
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
#[doc = "Reader of field `MAC_IACK_FOV`"]
pub type MAC_IACK_FOV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IACK_FOV`"]
pub struct MAC_IACK_FOV_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IACK_FOV_W<'a> {
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
#[doc = "Reader of field `MAC_IACK_RXER`"]
pub type MAC_IACK_RXER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IACK_RXER`"]
pub struct MAC_IACK_RXER_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IACK_RXER_W<'a> {
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
#[doc = "Reader of field `MAC_IACK_MDINT`"]
pub type MAC_IACK_MDINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IACK_MDINT`"]
pub struct MAC_IACK_MDINT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IACK_MDINT_W<'a> {
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
#[doc = "Reader of field `MAC_IACK_PHYINT`"]
pub type MAC_IACK_PHYINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IACK_PHYINT`"]
pub struct MAC_IACK_PHYINT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IACK_PHYINT_W<'a> {
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
    #[doc = "Bit 0 - Clear Packet Received"]
    #[inline(always)]
    pub fn mac_iack_rxint(&self) -> MAC_IACK_RXINT_R {
        MAC_IACK_RXINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear Transmit Error"]
    #[inline(always)]
    pub fn mac_iack_txer(&self) -> MAC_IACK_TXER_R {
        MAC_IACK_TXER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear Transmit FIFO Empty"]
    #[inline(always)]
    pub fn mac_iack_txemp(&self) -> MAC_IACK_TXEMP_R {
        MAC_IACK_TXEMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear FIFO Overrun"]
    #[inline(always)]
    pub fn mac_iack_fov(&self) -> MAC_IACK_FOV_R {
        MAC_IACK_FOV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear Receive Error"]
    #[inline(always)]
    pub fn mac_iack_rxer(&self) -> MAC_IACK_RXER_R {
        MAC_IACK_RXER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clear MII Transaction Complete"]
    #[inline(always)]
    pub fn mac_iack_mdint(&self) -> MAC_IACK_MDINT_R {
        MAC_IACK_MDINT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clear PHY Interrupt"]
    #[inline(always)]
    pub fn mac_iack_phyint(&self) -> MAC_IACK_PHYINT_R {
        MAC_IACK_PHYINT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Packet Received"]
    #[inline(always)]
    pub fn mac_iack_rxint(&mut self) -> MAC_IACK_RXINT_W {
        MAC_IACK_RXINT_W { w: self }
    }
    #[doc = "Bit 1 - Clear Transmit Error"]
    #[inline(always)]
    pub fn mac_iack_txer(&mut self) -> MAC_IACK_TXER_W {
        MAC_IACK_TXER_W { w: self }
    }
    #[doc = "Bit 2 - Clear Transmit FIFO Empty"]
    #[inline(always)]
    pub fn mac_iack_txemp(&mut self) -> MAC_IACK_TXEMP_W {
        MAC_IACK_TXEMP_W { w: self }
    }
    #[doc = "Bit 3 - Clear FIFO Overrun"]
    #[inline(always)]
    pub fn mac_iack_fov(&mut self) -> MAC_IACK_FOV_W {
        MAC_IACK_FOV_W { w: self }
    }
    #[doc = "Bit 4 - Clear Receive Error"]
    #[inline(always)]
    pub fn mac_iack_rxer(&mut self) -> MAC_IACK_RXER_W {
        MAC_IACK_RXER_W { w: self }
    }
    #[doc = "Bit 5 - Clear MII Transaction Complete"]
    #[inline(always)]
    pub fn mac_iack_mdint(&mut self) -> MAC_IACK_MDINT_W {
        MAC_IACK_MDINT_W { w: self }
    }
    #[doc = "Bit 6 - Clear PHY Interrupt"]
    #[inline(always)]
    pub fn mac_iack_phyint(&mut self) -> MAC_IACK_PHYINT_W {
        MAC_IACK_PHYINT_W { w: self }
    }
}
