#[doc = "Reader of register ECR"]
pub type R = crate::R<u32, super::ECR>;
#[doc = "Writer for register ECR"]
pub type W = crate::W<u32, super::ECR>;
#[doc = "Register ECR `reset()`'s with value 0"]
impl crate::ResetValue for super::ECR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_ECR_DATA`"]
pub type UART_ECR_DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_ECR_DATA`"]
pub struct UART_ECR_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ECR_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Error Clear"]
    #[inline(always)]
    pub fn uart_ecr_data(&self) -> UART_ECR_DATA_R {
        UART_ECR_DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Error Clear"]
    #[inline(always)]
    pub fn uart_ecr_data(&mut self) -> UART_ECR_DATA_W {
        UART_ECR_DATA_W { w: self }
    }
}
