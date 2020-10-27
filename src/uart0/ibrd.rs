#[doc = "Reader of register IBRD"]
pub type R = crate::R<u32, super::IBRD>;
#[doc = "Writer for register IBRD"]
pub type W = crate::W<u32, super::IBRD>;
#[doc = "Register IBRD `reset()`'s with value 0"]
impl crate::ResetValue for super::IBRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_IBRD_DIVINT`"]
pub type UART_IBRD_DIVINT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_IBRD_DIVINT`"]
pub struct UART_IBRD_DIVINT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IBRD_DIVINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Integer Baud-Rate Divisor"]
    #[inline(always)]
    pub fn uart_ibrd_divint(&self) -> UART_IBRD_DIVINT_R {
        UART_IBRD_DIVINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Integer Baud-Rate Divisor"]
    #[inline(always)]
    pub fn uart_ibrd_divint(&mut self) -> UART_IBRD_DIVINT_W {
        UART_IBRD_DIVINT_W { w: self }
    }
}
