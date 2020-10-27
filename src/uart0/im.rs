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
#[doc = "Reader of field `UART_IM_RXIM`"]
pub type UART_IM_RXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IM_RXIM`"]
pub struct UART_IM_RXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IM_RXIM_W<'a> {
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
#[doc = "Reader of field `UART_IM_TXIM`"]
pub type UART_IM_TXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IM_TXIM`"]
pub struct UART_IM_TXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IM_TXIM_W<'a> {
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
#[doc = "Reader of field `UART_IM_RTIM`"]
pub type UART_IM_RTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IM_RTIM`"]
pub struct UART_IM_RTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IM_RTIM_W<'a> {
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
#[doc = "Reader of field `UART_IM_FEIM`"]
pub type UART_IM_FEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IM_FEIM`"]
pub struct UART_IM_FEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IM_FEIM_W<'a> {
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
#[doc = "Reader of field `UART_IM_PEIM`"]
pub type UART_IM_PEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IM_PEIM`"]
pub struct UART_IM_PEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IM_PEIM_W<'a> {
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
#[doc = "Reader of field `UART_IM_BEIM`"]
pub type UART_IM_BEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IM_BEIM`"]
pub struct UART_IM_BEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IM_BEIM_W<'a> {
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
#[doc = "Reader of field `UART_IM_OEIM`"]
pub type UART_IM_OEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IM_OEIM`"]
pub struct UART_IM_OEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IM_OEIM_W<'a> {
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
    #[doc = "Bit 4 - UART Receive Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rxim(&self) -> UART_IM_RXIM_R {
        UART_IM_RXIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_txim(&self) -> UART_IM_TXIM_R {
        UART_IM_TXIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rtim(&self) -> UART_IM_RTIM_R {
        UART_IM_RTIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_feim(&self) -> UART_IM_FEIM_R {
        UART_IM_FEIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_peim(&self) -> UART_IM_PEIM_R {
        UART_IM_PEIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART Break Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_beim(&self) -> UART_IM_BEIM_R {
        UART_IM_BEIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_oeim(&self) -> UART_IM_OEIM_R {
        UART_IM_OEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - UART Receive Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rxim(&mut self) -> UART_IM_RXIM_W {
        UART_IM_RXIM_W { w: self }
    }
    #[doc = "Bit 5 - UART Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_txim(&mut self) -> UART_IM_TXIM_W {
        UART_IM_TXIM_W { w: self }
    }
    #[doc = "Bit 6 - UART Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rtim(&mut self) -> UART_IM_RTIM_W {
        UART_IM_RTIM_W { w: self }
    }
    #[doc = "Bit 7 - UART Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_feim(&mut self) -> UART_IM_FEIM_W {
        UART_IM_FEIM_W { w: self }
    }
    #[doc = "Bit 8 - UART Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_peim(&mut self) -> UART_IM_PEIM_W {
        UART_IM_PEIM_W { w: self }
    }
    #[doc = "Bit 9 - UART Break Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_beim(&mut self) -> UART_IM_BEIM_W {
        UART_IM_BEIM_W { w: self }
    }
    #[doc = "Bit 10 - UART Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_oeim(&mut self) -> UART_IM_OEIM_W {
        UART_IM_OEIM_W { w: self }
    }
}
