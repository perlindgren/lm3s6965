#[doc = "Reader of register RCTL"]
pub type R = crate::R<u32, super::RCTL>;
#[doc = "Writer for register RCTL"]
pub type W = crate::W<u32, super::RCTL>;
#[doc = "Register RCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_RCTL_RXEN`"]
pub type MAC_RCTL_RXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_RCTL_RXEN`"]
pub struct MAC_RCTL_RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_RCTL_RXEN_W<'a> {
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
#[doc = "Reader of field `MAC_RCTL_AMUL`"]
pub type MAC_RCTL_AMUL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_RCTL_AMUL`"]
pub struct MAC_RCTL_AMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_RCTL_AMUL_W<'a> {
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
#[doc = "Reader of field `MAC_RCTL_PRMS`"]
pub type MAC_RCTL_PRMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_RCTL_PRMS`"]
pub struct MAC_RCTL_PRMS_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_RCTL_PRMS_W<'a> {
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
#[doc = "Reader of field `MAC_RCTL_BADCRC`"]
pub type MAC_RCTL_BADCRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_RCTL_BADCRC`"]
pub struct MAC_RCTL_BADCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_RCTL_BADCRC_W<'a> {
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
#[doc = "Reader of field `MAC_RCTL_RSTFIFO`"]
pub type MAC_RCTL_RSTFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_RCTL_RSTFIFO`"]
pub struct MAC_RCTL_RSTFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_RCTL_RSTFIFO_W<'a> {
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
    #[doc = "Bit 0 - Enable Receiver"]
    #[inline(always)]
    pub fn mac_rctl_rxen(&self) -> MAC_RCTL_RXEN_R {
        MAC_RCTL_RXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Multicast Frames"]
    #[inline(always)]
    pub fn mac_rctl_amul(&self) -> MAC_RCTL_AMUL_R {
        MAC_RCTL_AMUL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Promiscuous Mode"]
    #[inline(always)]
    pub fn mac_rctl_prms(&self) -> MAC_RCTL_PRMS_R {
        MAC_RCTL_PRMS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Reject Bad CRC"]
    #[inline(always)]
    pub fn mac_rctl_badcrc(&self) -> MAC_RCTL_BADCRC_R {
        MAC_RCTL_BADCRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear Receive FIFO"]
    #[inline(always)]
    pub fn mac_rctl_rstfifo(&self) -> MAC_RCTL_RSTFIFO_R {
        MAC_RCTL_RSTFIFO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Receiver"]
    #[inline(always)]
    pub fn mac_rctl_rxen(&mut self) -> MAC_RCTL_RXEN_W {
        MAC_RCTL_RXEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable Multicast Frames"]
    #[inline(always)]
    pub fn mac_rctl_amul(&mut self) -> MAC_RCTL_AMUL_W {
        MAC_RCTL_AMUL_W { w: self }
    }
    #[doc = "Bit 2 - Enable Promiscuous Mode"]
    #[inline(always)]
    pub fn mac_rctl_prms(&mut self) -> MAC_RCTL_PRMS_W {
        MAC_RCTL_PRMS_W { w: self }
    }
    #[doc = "Bit 3 - Enable Reject Bad CRC"]
    #[inline(always)]
    pub fn mac_rctl_badcrc(&mut self) -> MAC_RCTL_BADCRC_W {
        MAC_RCTL_BADCRC_W { w: self }
    }
    #[doc = "Bit 4 - Clear Receive FIFO"]
    #[inline(always)]
    pub fn mac_rctl_rstfifo(&mut self) -> MAC_RCTL_RSTFIFO_W {
        MAC_RCTL_RSTFIFO_W { w: self }
    }
}
