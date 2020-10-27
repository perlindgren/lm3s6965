#[doc = "Reader of register FBRD"]
pub type R = crate::R<u32, super::FBRD>;
#[doc = "Writer for register FBRD"]
pub type W = crate::W<u32, super::FBRD>;
#[doc = "Register FBRD `reset()`'s with value 0"]
impl crate::ResetValue for super::FBRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_FBRD_DIVFRAC`"]
pub type UART_FBRD_DIVFRAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_FBRD_DIVFRAC`"]
pub struct UART_FBRD_DIVFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FBRD_DIVFRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Fractional Baud-Rate Divisor"]
    #[inline(always)]
    pub fn uart_fbrd_divfrac(&self) -> UART_FBRD_DIVFRAC_R {
        UART_FBRD_DIVFRAC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fractional Baud-Rate Divisor"]
    #[inline(always)]
    pub fn uart_fbrd_divfrac(&mut self) -> UART_FBRD_DIVFRAC_W {
        UART_FBRD_DIVFRAC_W { w: self }
    }
}
