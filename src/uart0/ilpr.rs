#[doc = "Reader of register ILPR"]
pub type R = crate::R<u32, super::ILPR>;
#[doc = "Writer for register ILPR"]
pub type W = crate::W<u32, super::ILPR>;
#[doc = "Register ILPR `reset()`'s with value 0"]
impl crate::ResetValue for super::ILPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_ILPR_ILPDVSR`"]
pub type UART_ILPR_ILPDVSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_ILPR_ILPDVSR`"]
pub struct UART_ILPR_ILPDVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ILPR_ILPDVSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - IrDA Low-Power Divisor"]
    #[inline(always)]
    pub fn uart_ilpr_ilpdvsr(&self) -> UART_ILPR_ILPDVSR_R {
        UART_ILPR_ILPDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IrDA Low-Power Divisor"]
    #[inline(always)]
    pub fn uart_ilpr_ilpdvsr(&mut self) -> UART_ILPR_ILPDVSR_W {
        UART_ILPR_ILPDVSR_W { w: self }
    }
}
