#[doc = "Reader of register LCRH"]
pub type R = crate::R<u32, super::LCRH>;
#[doc = "Writer for register LCRH"]
pub type W = crate::W<u32, super::LCRH>;
#[doc = "Register LCRH `reset()`'s with value 0"]
impl crate::ResetValue for super::LCRH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_LCRH_BRK`"]
pub type UART_LCRH_BRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_LCRH_BRK`"]
pub struct UART_LCRH_BRK_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_LCRH_BRK_W<'a> {
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
#[doc = "Reader of field `UART_LCRH_PEN`"]
pub type UART_LCRH_PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_LCRH_PEN`"]
pub struct UART_LCRH_PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_LCRH_PEN_W<'a> {
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
#[doc = "Reader of field `UART_LCRH_EPS`"]
pub type UART_LCRH_EPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_LCRH_EPS`"]
pub struct UART_LCRH_EPS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_LCRH_EPS_W<'a> {
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
#[doc = "Reader of field `UART_LCRH_STP2`"]
pub type UART_LCRH_STP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_LCRH_STP2`"]
pub struct UART_LCRH_STP2_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_LCRH_STP2_W<'a> {
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
#[doc = "Reader of field `UART_LCRH_FEN`"]
pub type UART_LCRH_FEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_LCRH_FEN`"]
pub struct UART_LCRH_FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_LCRH_FEN_W<'a> {
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
#[doc = "UART Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART_LCRH_WLEN_A {
    #[doc = "0: 5 bits (default)"]
    UART_LCRH_WLEN_5 = 0,
    #[doc = "1: 6 bits"]
    UART_LCRH_WLEN_6 = 1,
    #[doc = "2: 7 bits"]
    UART_LCRH_WLEN_7 = 2,
    #[doc = "3: 8 bits"]
    UART_LCRH_WLEN_8 = 3,
}
impl From<UART_LCRH_WLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: UART_LCRH_WLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UART_LCRH_WLEN`"]
pub type UART_LCRH_WLEN_R = crate::R<u8, UART_LCRH_WLEN_A>;
impl UART_LCRH_WLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART_LCRH_WLEN_A {
        match self.bits {
            0 => UART_LCRH_WLEN_A::UART_LCRH_WLEN_5,
            1 => UART_LCRH_WLEN_A::UART_LCRH_WLEN_6,
            2 => UART_LCRH_WLEN_A::UART_LCRH_WLEN_7,
            3 => UART_LCRH_WLEN_A::UART_LCRH_WLEN_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UART_LCRH_WLEN_5`"]
    #[inline(always)]
    pub fn is_uart_lcrh_wlen_5(&self) -> bool {
        *self == UART_LCRH_WLEN_A::UART_LCRH_WLEN_5
    }
    #[doc = "Checks if the value of the field is `UART_LCRH_WLEN_6`"]
    #[inline(always)]
    pub fn is_uart_lcrh_wlen_6(&self) -> bool {
        *self == UART_LCRH_WLEN_A::UART_LCRH_WLEN_6
    }
    #[doc = "Checks if the value of the field is `UART_LCRH_WLEN_7`"]
    #[inline(always)]
    pub fn is_uart_lcrh_wlen_7(&self) -> bool {
        *self == UART_LCRH_WLEN_A::UART_LCRH_WLEN_7
    }
    #[doc = "Checks if the value of the field is `UART_LCRH_WLEN_8`"]
    #[inline(always)]
    pub fn is_uart_lcrh_wlen_8(&self) -> bool {
        *self == UART_LCRH_WLEN_A::UART_LCRH_WLEN_8
    }
}
#[doc = "Write proxy for field `UART_LCRH_WLEN`"]
pub struct UART_LCRH_WLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_LCRH_WLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_LCRH_WLEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "5 bits (default)"]
    #[inline(always)]
    pub fn uart_lcrh_wlen_5(self) -> &'a mut W {
        self.variant(UART_LCRH_WLEN_A::UART_LCRH_WLEN_5)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn uart_lcrh_wlen_6(self) -> &'a mut W {
        self.variant(UART_LCRH_WLEN_A::UART_LCRH_WLEN_6)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn uart_lcrh_wlen_7(self) -> &'a mut W {
        self.variant(UART_LCRH_WLEN_A::UART_LCRH_WLEN_7)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn uart_lcrh_wlen_8(self) -> &'a mut W {
        self.variant(UART_LCRH_WLEN_A::UART_LCRH_WLEN_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `UART_LCRH_SPS`"]
pub type UART_LCRH_SPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_LCRH_SPS`"]
pub struct UART_LCRH_SPS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_LCRH_SPS_W<'a> {
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
    #[doc = "Bit 0 - UART Send Break"]
    #[inline(always)]
    pub fn uart_lcrh_brk(&self) -> UART_LCRH_BRK_R {
        UART_LCRH_BRK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART Parity Enable"]
    #[inline(always)]
    pub fn uart_lcrh_pen(&self) -> UART_LCRH_PEN_R {
        UART_LCRH_PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART Even Parity Select"]
    #[inline(always)]
    pub fn uart_lcrh_eps(&self) -> UART_LCRH_EPS_R {
        UART_LCRH_EPS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UART Two Stop Bits Select"]
    #[inline(always)]
    pub fn uart_lcrh_stp2(&self) -> UART_LCRH_STP2_R {
        UART_LCRH_STP2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART Enable FIFOs"]
    #[inline(always)]
    pub fn uart_lcrh_fen(&self) -> UART_LCRH_FEN_R {
        UART_LCRH_FEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - UART Word Length"]
    #[inline(always)]
    pub fn uart_lcrh_wlen(&self) -> UART_LCRH_WLEN_R {
        UART_LCRH_WLEN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - UART Stick Parity Select"]
    #[inline(always)]
    pub fn uart_lcrh_sps(&self) -> UART_LCRH_SPS_R {
        UART_LCRH_SPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Send Break"]
    #[inline(always)]
    pub fn uart_lcrh_brk(&mut self) -> UART_LCRH_BRK_W {
        UART_LCRH_BRK_W { w: self }
    }
    #[doc = "Bit 1 - UART Parity Enable"]
    #[inline(always)]
    pub fn uart_lcrh_pen(&mut self) -> UART_LCRH_PEN_W {
        UART_LCRH_PEN_W { w: self }
    }
    #[doc = "Bit 2 - UART Even Parity Select"]
    #[inline(always)]
    pub fn uart_lcrh_eps(&mut self) -> UART_LCRH_EPS_W {
        UART_LCRH_EPS_W { w: self }
    }
    #[doc = "Bit 3 - UART Two Stop Bits Select"]
    #[inline(always)]
    pub fn uart_lcrh_stp2(&mut self) -> UART_LCRH_STP2_W {
        UART_LCRH_STP2_W { w: self }
    }
    #[doc = "Bit 4 - UART Enable FIFOs"]
    #[inline(always)]
    pub fn uart_lcrh_fen(&mut self) -> UART_LCRH_FEN_W {
        UART_LCRH_FEN_W { w: self }
    }
    #[doc = "Bits 5:6 - UART Word Length"]
    #[inline(always)]
    pub fn uart_lcrh_wlen(&mut self) -> UART_LCRH_WLEN_W {
        UART_LCRH_WLEN_W { w: self }
    }
    #[doc = "Bit 7 - UART Stick Parity Select"]
    #[inline(always)]
    pub fn uart_lcrh_sps(&mut self) -> UART_LCRH_SPS_W {
        UART_LCRH_SPS_W { w: self }
    }
}
