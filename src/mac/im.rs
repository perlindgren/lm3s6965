#[doc = "Reader of register IM"]
pub type R = crate::R<u32, super::IM>;
#[doc = "Writer for register IM"]
pub type W = crate::W<u32, super::IM>;
#[doc = "Register IM `reset()`'s with value 0"]
impl crate::ResetValue for super::IM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_IM_RXINTM`"]
pub type MAC_IM_RXINTM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IM_RXINTM`"]
pub struct MAC_IM_RXINTM_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IM_RXINTM_W<'a> {
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
#[doc = "Reader of field `MAC_IM_TXERM`"]
pub type MAC_IM_TXERM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IM_TXERM`"]
pub struct MAC_IM_TXERM_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IM_TXERM_W<'a> {
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
#[doc = "Reader of field `MAC_IM_TXEMPM`"]
pub type MAC_IM_TXEMPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IM_TXEMPM`"]
pub struct MAC_IM_TXEMPM_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IM_TXEMPM_W<'a> {
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
#[doc = "Reader of field `MAC_IM_FOVM`"]
pub type MAC_IM_FOVM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IM_FOVM`"]
pub struct MAC_IM_FOVM_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IM_FOVM_W<'a> {
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
#[doc = "Reader of field `MAC_IM_RXERM`"]
pub type MAC_IM_RXERM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IM_RXERM`"]
pub struct MAC_IM_RXERM_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IM_RXERM_W<'a> {
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
#[doc = "Reader of field `MAC_IM_MDINTM`"]
pub type MAC_IM_MDINTM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IM_MDINTM`"]
pub struct MAC_IM_MDINTM_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IM_MDINTM_W<'a> {
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
#[doc = "Reader of field `MAC_IM_PHYINTM`"]
pub type MAC_IM_PHYINTM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_IM_PHYINTM`"]
pub struct MAC_IM_PHYINTM_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IM_PHYINTM_W<'a> {
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
    #[doc = "Bit 0 - Mask Packet Received"]
    #[inline(always)]
    pub fn mac_im_rxintm(&self) -> MAC_IM_RXINTM_R {
        MAC_IM_RXINTM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mask Transmit Error"]
    #[inline(always)]
    pub fn mac_im_txerm(&self) -> MAC_IM_TXERM_R {
        MAC_IM_TXERM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mask Transmit FIFO Empty"]
    #[inline(always)]
    pub fn mac_im_txempm(&self) -> MAC_IM_TXEMPM_R {
        MAC_IM_TXEMPM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mask FIFO Overrun"]
    #[inline(always)]
    pub fn mac_im_fovm(&self) -> MAC_IM_FOVM_R {
        MAC_IM_FOVM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask Receive Error"]
    #[inline(always)]
    pub fn mac_im_rxerm(&self) -> MAC_IM_RXERM_R {
        MAC_IM_RXERM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask MII Transaction Complete"]
    #[inline(always)]
    pub fn mac_im_mdintm(&self) -> MAC_IM_MDINTM_R {
        MAC_IM_MDINTM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mask PHY Interrupt"]
    #[inline(always)]
    pub fn mac_im_phyintm(&self) -> MAC_IM_PHYINTM_R {
        MAC_IM_PHYINTM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask Packet Received"]
    #[inline(always)]
    pub fn mac_im_rxintm(&mut self) -> MAC_IM_RXINTM_W {
        MAC_IM_RXINTM_W { w: self }
    }
    #[doc = "Bit 1 - Mask Transmit Error"]
    #[inline(always)]
    pub fn mac_im_txerm(&mut self) -> MAC_IM_TXERM_W {
        MAC_IM_TXERM_W { w: self }
    }
    #[doc = "Bit 2 - Mask Transmit FIFO Empty"]
    #[inline(always)]
    pub fn mac_im_txempm(&mut self) -> MAC_IM_TXEMPM_W {
        MAC_IM_TXEMPM_W { w: self }
    }
    #[doc = "Bit 3 - Mask FIFO Overrun"]
    #[inline(always)]
    pub fn mac_im_fovm(&mut self) -> MAC_IM_FOVM_W {
        MAC_IM_FOVM_W { w: self }
    }
    #[doc = "Bit 4 - Mask Receive Error"]
    #[inline(always)]
    pub fn mac_im_rxerm(&mut self) -> MAC_IM_RXERM_W {
        MAC_IM_RXERM_W { w: self }
    }
    #[doc = "Bit 5 - Mask MII Transaction Complete"]
    #[inline(always)]
    pub fn mac_im_mdintm(&mut self) -> MAC_IM_MDINTM_W {
        MAC_IM_MDINTM_W { w: self }
    }
    #[doc = "Bit 6 - Mask PHY Interrupt"]
    #[inline(always)]
    pub fn mac_im_phyintm(&mut self) -> MAC_IM_PHYINTM_W {
        MAC_IM_PHYINTM_W { w: self }
    }
}
