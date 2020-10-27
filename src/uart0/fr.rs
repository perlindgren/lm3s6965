#[doc = "Reader of register FR"]
pub type R = crate::R<u32, super::FR>;
#[doc = "Writer for register FR"]
pub type W = crate::W<u32, super::FR>;
#[doc = "Register FR `reset()`'s with value 0"]
impl crate::ResetValue for super::FR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_FR_BUSY`"]
pub type UART_FR_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_FR_BUSY`"]
pub struct UART_FR_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FR_BUSY_W<'a> {
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
#[doc = "Reader of field `UART_FR_RXFE`"]
pub type UART_FR_RXFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_FR_RXFE`"]
pub struct UART_FR_RXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FR_RXFE_W<'a> {
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
#[doc = "Reader of field `UART_FR_TXFF`"]
pub type UART_FR_TXFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_FR_TXFF`"]
pub struct UART_FR_TXFF_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FR_TXFF_W<'a> {
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
#[doc = "Reader of field `UART_FR_RXFF`"]
pub type UART_FR_RXFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_FR_RXFF`"]
pub struct UART_FR_RXFF_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FR_RXFF_W<'a> {
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
#[doc = "Reader of field `UART_FR_TXFE`"]
pub type UART_FR_TXFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_FR_TXFE`"]
pub struct UART_FR_TXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FR_TXFE_W<'a> {
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
impl R {
    #[doc = "Bit 3 - UART Busy"]
    #[inline(always)]
    pub fn uart_fr_busy(&self) -> UART_FR_BUSY_R {
        UART_FR_BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART Receive FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_rxfe(&self) -> UART_FR_RXFE_R {
        UART_FR_RXFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART Transmit FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_txff(&self) -> UART_FR_TXFF_R {
        UART_FR_TXFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART Receive FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_rxff(&self) -> UART_FR_RXFF_R {
        UART_FR_RXFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART Transmit FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_txfe(&self) -> UART_FR_TXFE_R {
        UART_FR_TXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - UART Busy"]
    #[inline(always)]
    pub fn uart_fr_busy(&mut self) -> UART_FR_BUSY_W {
        UART_FR_BUSY_W { w: self }
    }
    #[doc = "Bit 4 - UART Receive FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_rxfe(&mut self) -> UART_FR_RXFE_W {
        UART_FR_RXFE_W { w: self }
    }
    #[doc = "Bit 5 - UART Transmit FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_txff(&mut self) -> UART_FR_TXFF_W {
        UART_FR_TXFF_W { w: self }
    }
    #[doc = "Bit 6 - UART Receive FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_rxff(&mut self) -> UART_FR_RXFF_W {
        UART_FR_RXFF_W { w: self }
    }
    #[doc = "Bit 7 - UART Transmit FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_txfe(&mut self) -> UART_FR_TXFE_W {
        UART_FR_TXFE_W { w: self }
    }
}
