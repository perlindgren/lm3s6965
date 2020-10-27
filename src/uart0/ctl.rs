#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_CTL_UARTEN`"]
pub type UART_CTL_UARTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_CTL_UARTEN`"]
pub struct UART_CTL_UARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CTL_UARTEN_W<'a> {
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
#[doc = "Reader of field `UART_CTL_SIREN`"]
pub type UART_CTL_SIREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_CTL_SIREN`"]
pub struct UART_CTL_SIREN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CTL_SIREN_W<'a> {
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
#[doc = "Reader of field `UART_CTL_SIRLP`"]
pub type UART_CTL_SIRLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_CTL_SIRLP`"]
pub struct UART_CTL_SIRLP_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CTL_SIRLP_W<'a> {
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
#[doc = "Reader of field `UART_CTL_LBE`"]
pub type UART_CTL_LBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_CTL_LBE`"]
pub struct UART_CTL_LBE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CTL_LBE_W<'a> {
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
#[doc = "Reader of field `UART_CTL_TXE`"]
pub type UART_CTL_TXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_CTL_TXE`"]
pub struct UART_CTL_TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CTL_TXE_W<'a> {
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
#[doc = "Reader of field `UART_CTL_RXE`"]
pub type UART_CTL_RXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_CTL_RXE`"]
pub struct UART_CTL_RXE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CTL_RXE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - UART Enable"]
    #[inline(always)]
    pub fn uart_ctl_uarten(&self) -> UART_CTL_UARTEN_R {
        UART_CTL_UARTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART SIR Enable"]
    #[inline(always)]
    pub fn uart_ctl_siren(&self) -> UART_CTL_SIREN_R {
        UART_CTL_SIREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART SIR Low-Power Mode"]
    #[inline(always)]
    pub fn uart_ctl_sirlp(&self) -> UART_CTL_SIRLP_R {
        UART_CTL_SIRLP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART Loop Back Enable"]
    #[inline(always)]
    pub fn uart_ctl_lbe(&self) -> UART_CTL_LBE_R {
        UART_CTL_LBE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART Transmit Enable"]
    #[inline(always)]
    pub fn uart_ctl_txe(&self) -> UART_CTL_TXE_R {
        UART_CTL_TXE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART Receive Enable"]
    #[inline(always)]
    pub fn uart_ctl_rxe(&self) -> UART_CTL_RXE_R {
        UART_CTL_RXE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Enable"]
    #[inline(always)]
    pub fn uart_ctl_uarten(&mut self) -> UART_CTL_UARTEN_W {
        UART_CTL_UARTEN_W { w: self }
    }
    #[doc = "Bit 1 - UART SIR Enable"]
    #[inline(always)]
    pub fn uart_ctl_siren(&mut self) -> UART_CTL_SIREN_W {
        UART_CTL_SIREN_W { w: self }
    }
    #[doc = "Bit 2 - UART SIR Low-Power Mode"]
    #[inline(always)]
    pub fn uart_ctl_sirlp(&mut self) -> UART_CTL_SIRLP_W {
        UART_CTL_SIRLP_W { w: self }
    }
    #[doc = "Bit 7 - UART Loop Back Enable"]
    #[inline(always)]
    pub fn uart_ctl_lbe(&mut self) -> UART_CTL_LBE_W {
        UART_CTL_LBE_W { w: self }
    }
    #[doc = "Bit 8 - UART Transmit Enable"]
    #[inline(always)]
    pub fn uart_ctl_txe(&mut self) -> UART_CTL_TXE_W {
        UART_CTL_TXE_W { w: self }
    }
    #[doc = "Bit 9 - UART Receive Enable"]
    #[inline(always)]
    pub fn uart_ctl_rxe(&mut self) -> UART_CTL_RXE_W {
        UART_CTL_RXE_W { w: self }
    }
}
