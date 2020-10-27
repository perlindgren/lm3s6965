#[doc = "Reader of register IFLS"]
pub type R = crate::R<u32, super::IFLS>;
#[doc = "Writer for register IFLS"]
pub type W = crate::W<u32, super::IFLS>;
#[doc = "Register IFLS `reset()`'s with value 0"]
impl crate::ResetValue for super::IFLS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "UART Transmit Interrupt FIFO Level Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART_IFLS_TX_A {
    #[doc = "0: TX FIFO &lt;= 1/8 full"]
    UART_IFLS_TX1_8 = 0,
    #[doc = "1: TX FIFO &lt;= 1/4 full"]
    UART_IFLS_TX2_8 = 1,
    #[doc = "2: TX FIFO &lt;= 1/2 full (default)"]
    UART_IFLS_TX4_8 = 2,
    #[doc = "3: TX FIFO &lt;= 3/4 full"]
    UART_IFLS_TX6_8 = 3,
    #[doc = "4: TX FIFO &lt;= 7/8 full"]
    UART_IFLS_TX7_8 = 4,
}
impl From<UART_IFLS_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: UART_IFLS_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UART_IFLS_TX`"]
pub type UART_IFLS_TX_R = crate::R<u8, UART_IFLS_TX_A>;
impl UART_IFLS_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UART_IFLS_TX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UART_IFLS_TX_A::UART_IFLS_TX1_8),
            1 => Val(UART_IFLS_TX_A::UART_IFLS_TX2_8),
            2 => Val(UART_IFLS_TX_A::UART_IFLS_TX4_8),
            3 => Val(UART_IFLS_TX_A::UART_IFLS_TX6_8),
            4 => Val(UART_IFLS_TX_A::UART_IFLS_TX7_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX1_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx1_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX1_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX2_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx2_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX2_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX4_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx4_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX4_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX6_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx6_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX6_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX7_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx7_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX7_8
    }
}
#[doc = "Write proxy for field `UART_IFLS_TX`"]
pub struct UART_IFLS_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IFLS_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_IFLS_TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TX FIFO &lt;= 1/8 full"]
    #[inline(always)]
    pub fn uart_ifls_tx1_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX1_8)
    }
    #[doc = "TX FIFO &lt;= 1/4 full"]
    #[inline(always)]
    pub fn uart_ifls_tx2_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX2_8)
    }
    #[doc = "TX FIFO &lt;= 1/2 full (default)"]
    #[inline(always)]
    pub fn uart_ifls_tx4_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX4_8)
    }
    #[doc = "TX FIFO &lt;= 3/4 full"]
    #[inline(always)]
    pub fn uart_ifls_tx6_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX6_8)
    }
    #[doc = "TX FIFO &lt;= 7/8 full"]
    #[inline(always)]
    pub fn uart_ifls_tx7_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX7_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "UART Receive Interrupt FIFO Level Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART_IFLS_RX_A {
    #[doc = "0: RX FIFO >= 1/8 full"]
    UART_IFLS_RX1_8 = 0,
    #[doc = "1: RX FIFO >= 1/4 full"]
    UART_IFLS_RX2_8 = 1,
    #[doc = "2: RX FIFO >= 1/2 full (default)"]
    UART_IFLS_RX4_8 = 2,
    #[doc = "3: RX FIFO >= 3/4 full"]
    UART_IFLS_RX6_8 = 3,
    #[doc = "4: RX FIFO >= 7/8 full"]
    UART_IFLS_RX7_8 = 4,
}
impl From<UART_IFLS_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: UART_IFLS_RX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UART_IFLS_RX`"]
pub type UART_IFLS_RX_R = crate::R<u8, UART_IFLS_RX_A>;
impl UART_IFLS_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UART_IFLS_RX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UART_IFLS_RX_A::UART_IFLS_RX1_8),
            1 => Val(UART_IFLS_RX_A::UART_IFLS_RX2_8),
            2 => Val(UART_IFLS_RX_A::UART_IFLS_RX4_8),
            3 => Val(UART_IFLS_RX_A::UART_IFLS_RX6_8),
            4 => Val(UART_IFLS_RX_A::UART_IFLS_RX7_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX1_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx1_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX1_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX2_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx2_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX2_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX4_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx4_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX4_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX6_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx6_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX6_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX7_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx7_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX7_8
    }
}
#[doc = "Write proxy for field `UART_IFLS_RX`"]
pub struct UART_IFLS_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IFLS_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_IFLS_RX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "RX FIFO >= 1/8 full"]
    #[inline(always)]
    pub fn uart_ifls_rx1_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX1_8)
    }
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline(always)]
    pub fn uart_ifls_rx2_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX2_8)
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline(always)]
    pub fn uart_ifls_rx4_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX4_8)
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline(always)]
    pub fn uart_ifls_rx6_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX6_8)
    }
    #[doc = "RX FIFO >= 7/8 full"]
    #[inline(always)]
    pub fn uart_ifls_rx7_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX7_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_tx(&self) -> UART_IFLS_TX_R {
        UART_IFLS_TX_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_rx(&self) -> UART_IFLS_RX_R {
        UART_IFLS_RX_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_tx(&mut self) -> UART_IFLS_TX_W {
        UART_IFLS_TX_W { w: self }
    }
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_rx(&mut self) -> UART_IFLS_RX_W {
        UART_IFLS_RX_W { w: self }
    }
}
