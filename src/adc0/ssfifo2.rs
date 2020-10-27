#[doc = "Reader of register SSFIFO2"]
pub type R = crate::R<u32, super::SSFIFO2>;
#[doc = "Writer for register SSFIFO2"]
pub type W = crate::W<u32, super::SSFIFO2>;
#[doc = "Register SSFIFO2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSFIFO2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_SSFIFO2_DATA`"]
pub type ADC_SSFIFO2_DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADC_SSFIFO2_DATA`"]
pub struct ADC_SSFIFO2_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSFIFO2_DATA_W<'a> {
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
    pub fn adc_ssfifo2_data(&self) -> ADC_SSFIFO2_DATA_R {
        ADC_SSFIFO2_DATA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Conversion Result Data"]
    #[inline(always)]
    pub fn adc_ssfifo2_data(&mut self) -> ADC_SSFIFO2_DATA_W {
        ADC_SSFIFO2_DATA_W { w: self }
    }
}
