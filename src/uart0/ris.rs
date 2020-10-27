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
#[doc = "Reader of field `UART_RIS_RXRIS`"]
pub type UART_RIS_RXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RIS_RXRIS`"]
pub struct UART_RIS_RXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RIS_RXRIS_W<'a> {
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
#[doc = "Reader of field `UART_RIS_TXRIS`"]
pub type UART_RIS_TXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RIS_TXRIS`"]
pub struct UART_RIS_TXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RIS_TXRIS_W<'a> {
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
#[doc = "Reader of field `UART_RIS_RTRIS`"]
pub type UART_RIS_RTRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RIS_RTRIS`"]
pub struct UART_RIS_RTRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RIS_RTRIS_W<'a> {
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
#[doc = "Reader of field `UART_RIS_FERIS`"]
pub type UART_RIS_FERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RIS_FERIS`"]
pub struct UART_RIS_FERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RIS_FERIS_W<'a> {
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
#[doc = "Reader of field `UART_RIS_PERIS`"]
pub type UART_RIS_PERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RIS_PERIS`"]
pub struct UART_RIS_PERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RIS_PERIS_W<'a> {
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
#[doc = "Reader of field `UART_RIS_BERIS`"]
pub type UART_RIS_BERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RIS_BERIS`"]
pub struct UART_RIS_BERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RIS_BERIS_W<'a> {
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
#[doc = "Reader of field `UART_RIS_OERIS`"]
pub type UART_RIS_OERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RIS_OERIS`"]
pub struct UART_RIS_OERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RIS_OERIS_W<'a> {
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
    #[doc = "Bit 4 - UART Receive Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rxris(&self) -> UART_RIS_RXRIS_R {
        UART_RIS_RXRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART Transmit Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_txris(&self) -> UART_RIS_TXRIS_R {
        UART_RIS_TXRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rtris(&self) -> UART_RIS_RTRIS_R {
        UART_RIS_RTRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART Framing Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_feris(&self) -> UART_RIS_FERIS_R {
        UART_RIS_FERIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART Parity Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_peris(&self) -> UART_RIS_PERIS_R {
        UART_RIS_PERIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART Break Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_beris(&self) -> UART_RIS_BERIS_R {
        UART_RIS_BERIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART Overrun Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_oeris(&self) -> UART_RIS_OERIS_R {
        UART_RIS_OERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - UART Receive Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rxris(&mut self) -> UART_RIS_RXRIS_W {
        UART_RIS_RXRIS_W { w: self }
    }
    #[doc = "Bit 5 - UART Transmit Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_txris(&mut self) -> UART_RIS_TXRIS_W {
        UART_RIS_TXRIS_W { w: self }
    }
    #[doc = "Bit 6 - UART Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rtris(&mut self) -> UART_RIS_RTRIS_W {
        UART_RIS_RTRIS_W { w: self }
    }
    #[doc = "Bit 7 - UART Framing Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_feris(&mut self) -> UART_RIS_FERIS_W {
        UART_RIS_FERIS_W { w: self }
    }
    #[doc = "Bit 8 - UART Parity Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_peris(&mut self) -> UART_RIS_PERIS_W {
        UART_RIS_PERIS_W { w: self }
    }
    #[doc = "Bit 9 - UART Break Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_beris(&mut self) -> UART_RIS_BERIS_W {
        UART_RIS_BERIS_W { w: self }
    }
    #[doc = "Bit 10 - UART Overrun Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_oeris(&mut self) -> UART_RIS_OERIS_W {
        UART_RIS_OERIS_W { w: self }
    }
}
