#[doc = "Reader of register SSFIFO3"]
pub type R = crate::R<u32, super::SSFIFO3>;
#[doc = "Writer for register SSFIFO3"]
pub type W = crate::W<u32, super::SSFIFO3>;
#[doc = "Register SSFIFO3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSFIFO3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_SSFIFO3_DATA`"]
pub type ADC_SSFIFO3_DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADC_SSFIFO3_DATA`"]
pub struct ADC_SSFIFO3_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSFIFO3_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Conversion Result Data"]
    #[inline(always)]
    pub fn adc_ssfifo3_data(&self) -> ADC_SSFIFO3_DATA_R {
        ADC_SSFIFO3_DATA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Conversion Result Data"]
    #[inline(always)]
    pub fn adc_ssfifo3_data(&mut self) -> ADC_SSFIFO3_DATA_W {
        ADC_SSFIFO3_DATA_W { w: self }
    }
}
