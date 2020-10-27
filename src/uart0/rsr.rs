#[doc = "Reader of register RSR"]
pub type R = crate::R<u32, super::RSR>;
#[doc = "Writer for register RSR"]
pub type W = crate::W<u32, super::RSR>;
#[doc = "Register RSR `reset()`'s with value 0"]
impl crate::ResetValue for super::RSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_RSR_FE`"]
pub type UART_RSR_FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RSR_FE`"]
pub struct UART_RSR_FE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RSR_FE_W<'a> {
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
#[doc = "Reader of field `UART_RSR_PE`"]
pub type UART_RSR_PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RSR_PE`"]
pub struct UART_RSR_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RSR_PE_W<'a> {
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
#[doc = "Reader of field `UART_RSR_BE`"]
pub type UART_RSR_BE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RSR_BE`"]
pub struct UART_RSR_BE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RSR_BE_W<'a> {
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
#[doc = "Reader of field `UART_RSR_OE`"]
pub type UART_RSR_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RSR_OE`"]
pub struct UART_RSR_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RSR_OE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - UART Framing Error"]
    #[inline(always)]
    pub fn uart_rsr_fe(&self) -> UART_RSR_FE_R {
        UART_RSR_FE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART Parity Error"]
    #[inline(always)]
    pub fn uart_rsr_pe(&self) -> UART_RSR_PE_R {
        UART_RSR_PE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART Break Error"]
    #[inline(always)]
    pub fn uart_rsr_be(&self) -> UART_RSR_BE_R {
        UART_RSR_BE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UART Overrun Error"]
    #[inline(always)]
    pub fn uart_rsr_oe(&self) -> UART_RSR_OE_R {
        UART_RSR_OE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Framing Error"]
    #[inline(always)]
    pub fn uart_rsr_fe(&mut self) -> UART_RSR_FE_W {
        UART_RSR_FE_W { w: self }
    }
    #[doc = "Bit 1 - UART Parity Error"]
    #[inline(always)]
    pub fn uart_rsr_pe(&mut self) -> UART_RSR_PE_W {
        UART_RSR_PE_W { w: self }
    }
    #[doc = "Bit 2 - UART Break Error"]
    #[inline(always)]
    pub fn uart_rsr_be(&mut self) -> UART_RSR_BE_W {
        UART_RSR_BE_W { w: self }
    }
    #[doc = "Bit 3 - UART Overrun Error"]
    #[inline(always)]
    pub fn uart_rsr_oe(&mut self) -> UART_RSR_OE_W {
        UART_RSR_OE_W { w: self }
    }
}
