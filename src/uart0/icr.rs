#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `UART_ICR_RXIC`"]
pub struct UART_ICR_RXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ICR_RXIC_W<'a> {
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
#[doc = "Write proxy for field `UART_ICR_TXIC`"]
pub struct UART_ICR_TXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ICR_TXIC_W<'a> {
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
#[doc = "Write proxy for field `UART_ICR_RTIC`"]
pub struct UART_ICR_RTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ICR_RTIC_W<'a> {
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
#[doc = "Write proxy for field `UART_ICR_FEIC`"]
pub struct UART_ICR_FEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ICR_FEIC_W<'a> {
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
#[doc = "Write proxy for field `UART_ICR_PEIC`"]
pub struct UART_ICR_PEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ICR_PEIC_W<'a> {
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
#[doc = "Write proxy for field `UART_ICR_BEIC`"]
pub struct UART_ICR_BEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ICR_BEIC_W<'a> {
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
#[doc = "Write proxy for field `UART_ICR_OEIC`"]
pub struct UART_ICR_OEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ICR_OEIC_W<'a> {
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
impl W {
    #[doc = "Bit 4 - Receive Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_rxic(&mut self) -> UART_ICR_RXIC_W {
        UART_ICR_RXIC_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_txic(&mut self) -> UART_ICR_TXIC_W {
        UART_ICR_TXIC_W { w: self }
    }
    #[doc = "Bit 6 - Receive Time-Out Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_rtic(&mut self) -> UART_ICR_RTIC_W {
        UART_ICR_RTIC_W { w: self }
    }
    #[doc = "Bit 7 - Framing Error Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_feic(&mut self) -> UART_ICR_FEIC_W {
        UART_ICR_FEIC_W { w: self }
    }
    #[doc = "Bit 8 - Parity Error Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_peic(&mut self) -> UART_ICR_PEIC_W {
        UART_ICR_PEIC_W { w: self }
    }
    #[doc = "Bit 9 - Break Error Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_beic(&mut self) -> UART_ICR_BEIC_W {
        UART_ICR_BEIC_W { w: self }
    }
    #[doc = "Bit 10 - Overrun Error Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_oeic(&mut self) -> UART_ICR_OEIC_W {
        UART_ICR_OEIC_W { w: self }
    }
}
