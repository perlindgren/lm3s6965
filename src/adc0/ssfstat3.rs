#[doc = "Reader of register SSFSTAT3"]
pub type R = crate::R<u32, super::SSFSTAT3>;
#[doc = "Writer for register SSFSTAT3"]
pub type W = crate::W<u32, super::SSFSTAT3>;
#[doc = "Register SSFSTAT3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSFSTAT3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_SSFSTAT3_TPTR`"]
pub type ADC_SSFSTAT3_TPTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_SSFSTAT3_TPTR`"]
pub struct ADC_SSFSTAT3_TPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSFSTAT3_TPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSFSTAT3_HPTR`"]
pub type ADC_SSFSTAT3_HPTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_SSFSTAT3_HPTR`"]
pub struct ADC_SSFSTAT3_HPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSFSTAT3_HPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSFSTAT3_EMPTY`"]
pub type ADC_SSFSTAT3_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSFSTAT3_EMPTY`"]
pub struct ADC_SSFSTAT3_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSFSTAT3_EMPTY_W<'a> {
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
#[doc = "Reader of field `ADC_SSFSTAT3_FULL`"]
pub type ADC_SSFSTAT3_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSFSTAT3_FULL`"]
pub struct ADC_SSFSTAT3_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSFSTAT3_FULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - FIFO Tail Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat3_tptr(&self) -> ADC_SSFSTAT3_TPTR_R {
        ADC_SSFSTAT3_TPTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FIFO Head Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat3_hptr(&self) -> ADC_SSFSTAT3_HPTR_R {
        ADC_SSFSTAT3_HPTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - FIFO Empty"]
    #[inline(always)]
    pub fn adc_ssfstat3_empty(&self) -> ADC_SSFSTAT3_EMPTY_R {
        ADC_SSFSTAT3_EMPTY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FIFO Full"]
    #[inline(always)]
    pub fn adc_ssfstat3_full(&self) -> ADC_SSFSTAT3_FULL_R {
        ADC_SSFSTAT3_FULL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FIFO Tail Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat3_tptr(&mut self) -> ADC_SSFSTAT3_TPTR_W {
        ADC_SSFSTAT3_TPTR_W { w: self }
    }
    #[doc = "Bits 4:7 - FIFO Head Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat3_hptr(&mut self) -> ADC_SSFSTAT3_HPTR_W {
        ADC_SSFSTAT3_HPTR_W { w: self }
    }
    #[doc = "Bit 8 - FIFO Empty"]
    #[inline(always)]
    pub fn adc_ssfstat3_empty(&mut self) -> ADC_SSFSTAT3_EMPTY_W {
        ADC_SSFSTAT3_EMPTY_W { w: self }
    }
    #[doc = "Bit 12 - FIFO Full"]
    #[inline(always)]
    pub fn adc_ssfstat3_full(&mut self) -> ADC_SSFSTAT3_FULL_W {
        ADC_SSFSTAT3_FULL_W { w: self }
    }
}
