#[doc = "Reader of register DR"]
pub type R = crate::R<u32, super::DR>;
#[doc = "Writer for register DR"]
pub type W = crate::W<u32, super::DR>;
#[doc = "Register DR `reset()`'s with value 0"]
impl crate::ResetValue for super::DR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_DR_DATA`"]
pub type UART_DR_DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_DR_DATA`"]
pub struct UART_DR_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DR_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `UART_DR_FE`"]
pub type UART_DR_FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DR_FE`"]
pub struct UART_DR_FE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DR_FE_W<'a> {
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
#[doc = "Reader of field `UART_DR_PE`"]
pub type UART_DR_PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DR_PE`"]
pub struct UART_DR_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DR_PE_W<'a> {
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
#[doc = "Reader of field `UART_DR_BE`"]
pub type UART_DR_BE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DR_BE`"]
pub struct UART_DR_BE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DR_BE_W<'a> {
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
#[doc = "Reader of field `UART_DR_OE`"]
pub type UART_DR_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DR_OE`"]
pub struct UART_DR_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DR_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Transmitted or Received"]
    #[inline(always)]
    pub fn uart_dr_data(&self) -> UART_DR_DATA_R {
        UART_DR_DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - UART Framing Error"]
    #[inline(always)]
    pub fn uart_dr_fe(&self) -> UART_DR_FE_R {
        UART_DR_FE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART Parity Error"]
    #[inline(always)]
    pub fn uart_dr_pe(&self) -> UART_DR_PE_R {
        UART_DR_PE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART Break Error"]
    #[inline(always)]
    pub fn uart_dr_be(&self) -> UART_DR_BE_R {
        UART_DR_BE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - UART Overrun Error"]
    #[inline(always)]
    pub fn uart_dr_oe(&self) -> UART_DR_OE_R {
        UART_DR_OE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Transmitted or Received"]
    #[inline(always)]
    pub fn uart_dr_data(&mut self) -> UART_DR_DATA_W {
        UART_DR_DATA_W { w: self }
    }
    #[doc = "Bit 8 - UART Framing Error"]
    #[inline(always)]
    pub fn uart_dr_fe(&mut self) -> UART_DR_FE_W {
        UART_DR_FE_W { w: self }
    }
    #[doc = "Bit 9 - UART Parity Error"]
    #[inline(always)]
    pub fn uart_dr_pe(&mut self) -> UART_DR_PE_W {
        UART_DR_PE_W { w: self }
    }
    #[doc = "Bit 10 - UART Break Error"]
    #[inline(always)]
    pub fn uart_dr_be(&mut self) -> UART_DR_BE_W {
        UART_DR_BE_W { w: self }
    }
    #[doc = "Bit 11 - UART Overrun Error"]
    #[inline(always)]
    pub fn uart_dr_oe(&mut self) -> UART_DR_OE_W {
        UART_DR_OE_W { w: self }
    }
}
