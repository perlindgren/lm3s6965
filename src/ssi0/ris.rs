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
#[doc = "Reader of field `SSI_RIS_RORRIS`"]
pub type SSI_RIS_RORRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_RIS_RORRIS`"]
pub struct SSI_RIS_RORRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_RIS_RORRIS_W<'a> {
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
#[doc = "Reader of field `SSI_RIS_RTRIS`"]
pub type SSI_RIS_RTRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_RIS_RTRIS`"]
pub struct SSI_RIS_RTRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_RIS_RTRIS_W<'a> {
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
#[doc = "Reader of field `SSI_RIS_RXRIS`"]
pub type SSI_RIS_RXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_RIS_RXRIS`"]
pub struct SSI_RIS_RXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_RIS_RXRIS_W<'a> {
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
#[doc = "Reader of field `SSI_RIS_TXRIS`"]
pub type SSI_RIS_TXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_RIS_TXRIS`"]
pub struct SSI_RIS_TXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_RIS_TXRIS_W<'a> {
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
    #[doc = "Bit 0 - SSI Receive Overrun Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rorris(&self) -> SSI_RIS_RORRIS_R {
        SSI_RIS_RORRIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rtris(&self) -> SSI_RIS_RTRIS_R {
        SSI_RIS_RTRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rxris(&self) -> SSI_RIS_RXRIS_R {
        SSI_RIS_RXRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_txris(&self) -> SSI_RIS_TXRIS_R {
        SSI_RIS_TXRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Receive Overrun Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rorris(&mut self) -> SSI_RIS_RORRIS_W {
        SSI_RIS_RORRIS_W { w: self }
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rtris(&mut self) -> SSI_RIS_RTRIS_W {
        SSI_RIS_RTRIS_W { w: self }
    }
    #[doc = "Bit 2 - SSI Receive FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rxris(&mut self) -> SSI_RIS_RXRIS_W {
        SSI_RIS_RXRIS_W { w: self }
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_txris(&mut self) -> SSI_RIS_TXRIS_W {
        SSI_RIS_TXRIS_W { w: self }
    }
}
