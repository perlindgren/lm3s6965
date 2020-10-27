#[doc = "Reader of register SSMUX3"]
pub type R = crate::R<u32, super::SSMUX3>;
#[doc = "Writer for register SSMUX3"]
pub type W = crate::W<u32, super::SSMUX3>;
#[doc = "Register SSMUX3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSMUX3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_SSMUX3_MUX0`"]
pub type ADC_SSMUX3_MUX0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_SSMUX3_MUX0`"]
pub struct ADC_SSMUX3_MUX0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSMUX3_MUX0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux3_mux0(&self) -> ADC_SSMUX3_MUX0_R {
        ADC_SSMUX3_MUX0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux3_mux0(&mut self) -> ADC_SSMUX3_MUX0_W {
        ADC_SSMUX3_MUX0_W { w: self }
    }
}
