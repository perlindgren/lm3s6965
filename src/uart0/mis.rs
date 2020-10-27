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
#[doc = "Reader of field `UART_MIS_RXMIS`"]
pub type UART_MIS_RXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_MIS_RXMIS`"]
pub struct UART_MIS_RXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MIS_RXMIS_W<'a> {
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
#[doc = "Reader of field `UART_MIS_TXMIS`"]
pub type UART_MIS_TXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_MIS_TXMIS`"]
pub struct UART_MIS_TXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MIS_TXMIS_W<'a> {
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
#[doc = "Reader of field `UART_MIS_RTMIS`"]
pub type UART_MIS_RTMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_MIS_RTMIS`"]
pub struct UART_MIS_RTMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MIS_RTMIS_W<'a> {
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
#[doc = "Reader of field `UART_MIS_FEMIS`"]
pub type UART_MIS_FEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_MIS_FEMIS`"]
pub struct UART_MIS_FEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MIS_FEMIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `UART_MIS_PEMIS`"]
pub type UART_MIS_PEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_MIS_PEMIS`"]
pub struct UART_MIS_PEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MIS_PEMIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `UART_MIS_BEMIS`"]
pub type UART_MIS_BEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_MIS_BEMIS`"]
pub struct UART_MIS_BEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MIS_BEMIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `UART_MIS_OEMIS`"]
pub type UART_MIS_OEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_MIS_OEMIS`"]
pub struct UART_MIS_OEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MIS_OEMIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - UART Receive Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rxmis(&self) -> UART_MIS_RXMIS_R {
        UART_MIS_RXMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART Transmit Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_txmis(&self) -> UART_MIS_TXMIS_R {
        UART_MIS_TXMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rtmis(&self) -> UART_MIS_RTMIS_R {
        UART_MIS_RTMIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART Framing Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_femis(&self) -> UART_MIS_FEMIS_R {
        UART_MIS_FEMIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART Parity Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_pemis(&self) -> UART_MIS_PEMIS_R {
        UART_MIS_PEMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART Break Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_bemis(&self) -> UART_MIS_BEMIS_R {
        UART_MIS_BEMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART Overrun Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_oemis(&self) -> UART_MIS_OEMIS_R {
        UART_MIS_OEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - UART Receive Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rxmis(&mut self) -> UART_MIS_RXMIS_W {
        UART_MIS_RXMIS_W { w: self }
    }
    #[doc = "Bit 5 - UART Transmit Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_txmis(&mut self) -> UART_MIS_TXMIS_W {
        UART_MIS_TXMIS_W { w: self }
    }
    #[doc = "Bit 6 - UART Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rtmis(&mut self) -> UART_MIS_RTMIS_W {
        UART_MIS_RTMIS_W { w: self }
    }
    #[doc = "Bit 7 - UART Framing Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_femis(&mut self) -> UART_MIS_FEMIS_W {
        UART_MIS_FEMIS_W { w: self }
    }
    #[doc = "Bit 8 - UART Parity Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_pemis(&mut self) -> UART_MIS_PEMIS_W {
        UART_MIS_PEMIS_W { w: self }
    }
    #[doc = "Bit 9 - UART Break Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_bemis(&mut self) -> UART_MIS_BEMIS_W {
        UART_MIS_BEMIS_W { w: self }
    }
    #[doc = "Bit 10 - UART Overrun Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_oemis(&mut self) -> UART_MIS_OEMIS_W {
        UART_MIS_OEMIS_W { w: self }
    }
}
