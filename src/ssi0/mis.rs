#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Writer for register MIS"]
pub type W = crate::W<u32, super::MIS>;
#[doc = "Register MIS `reset()`'s with value 0"]
impl crate::ResetValue for super::MIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSI_MIS_RORMIS`"]
pub type SSI_MIS_RORMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_MIS_RORMIS`"]
pub struct SSI_MIS_RORMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_MIS_RORMIS_W<'a> {
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
#[doc = "Reader of field `SSI_MIS_RTMIS`"]
pub type SSI_MIS_RTMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_MIS_RTMIS`"]
pub struct SSI_MIS_RTMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_MIS_RTMIS_W<'a> {
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
#[doc = "Reader of field `SSI_MIS_RXMIS`"]
pub type SSI_MIS_RXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_MIS_RXMIS`"]
pub struct SSI_MIS_RXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_MIS_RXMIS_W<'a> {
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
#[doc = "Reader of field `SSI_MIS_TXMIS`"]
pub type SSI_MIS_TXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_MIS_TXMIS`"]
pub struct SSI_MIS_TXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_MIS_TXMIS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SSI Receive Overrun Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rormis(&self) -> SSI_MIS_RORMIS_R {
        SSI_MIS_RORMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rtmis(&self) -> SSI_MIS_RTMIS_R {
        SSI_MIS_RTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rxmis(&self) -> SSI_MIS_RXMIS_R {
        SSI_MIS_RXMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_txmis(&self) -> SSI_MIS_TXMIS_R {
        SSI_MIS_TXMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Receive Overrun Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rormis(&mut self) -> SSI_MIS_RORMIS_W {
        SSI_MIS_RORMIS_W { w: self }
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rtmis(&mut self) -> SSI_MIS_RTMIS_W {
        SSI_MIS_RTMIS_W { w: self }
    }
    #[doc = "Bit 2 - SSI Receive FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rxmis(&mut self) -> SSI_MIS_RXMIS_W {
        SSI_MIS_RXMIS_W { w: self }
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_txmis(&mut self) -> SSI_MIS_TXMIS_W {
        SSI_MIS_TXMIS_W { w: self }
    }
}
