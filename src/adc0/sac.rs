#[doc = "Reader of register SAC"]
pub type R = crate::R<u32, super::SAC>;
#[doc = "Writer for register SAC"]
pub type W = crate::W<u32, super::SAC>;
#[doc = "Register SAC `reset()`'s with value 0"]
impl crate::ResetValue for super::SAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Hardware Averaging Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_SAC_AVG_A {
    #[doc = "0: No hardware oversampling"]
    ADC_SAC_AVG_OFF = 0,
    #[doc = "1: 2x hardware oversampling"]
    ADC_SAC_AVG_2X = 1,
    #[doc = "2: 4x hardware oversampling"]
    ADC_SAC_AVG_4X = 2,
    #[doc = "3: 8x hardware oversampling"]
    ADC_SAC_AVG_8X = 3,
    #[doc = "4: 16x hardware oversampling"]
    ADC_SAC_AVG_16X = 4,
    #[doc = "5: 32x hardware oversampling"]
    ADC_SAC_AVG_32X = 5,
    #[doc = "6: 64x hardware oversampling"]
    ADC_SAC_AVG_64X = 6,
}
impl From<ADC_SAC_AVG_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_SAC_AVG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_SAC_AVG`"]
pub type ADC_SAC_AVG_R = crate::R<u8, ADC_SAC_AVG_A>;
impl ADC_SAC_AVG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADC_SAC_AVG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC_SAC_AVG_A::ADC_SAC_AVG_OFF),
            1 => Val(ADC_SAC_AVG_A::ADC_SAC_AVG_2X),
            2 => Val(ADC_SAC_AVG_A::ADC_SAC_AVG_4X),
            3 => Val(ADC_SAC_AVG_A::ADC_SAC_AVG_8X),
            4 => Val(ADC_SAC_AVG_A::ADC_SAC_AVG_16X),
            5 => Val(ADC_SAC_AVG_A::ADC_SAC_AVG_32X),
            6 => Val(ADC_SAC_AVG_A::ADC_SAC_AVG_64X),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_OFF`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_off(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_OFF
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_2X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_2x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_2X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_4X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_4x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_4X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_8X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_8x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_8X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_16X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_16x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_16X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_32X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_32x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_32X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_64X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_64x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_64X
    }
}
#[doc = "Write proxy for field `ADC_SAC_AVG`"]
pub struct ADC_SAC_AVG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SAC_AVG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_SAC_AVG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_off(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_OFF)
    }
    #[doc = "2x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_2x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_2X)
    }
    #[doc = "4x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_4x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_4X)
    }
    #[doc = "8x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_8x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_8X)
    }
    #[doc = "16x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_16x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_16X)
    }
    #[doc = "32x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_32x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_32X)
    }
    #[doc = "64x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_64x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_64X)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Hardware Averaging Control"]
    #[inline(always)]
    pub fn adc_sac_avg(&self) -> ADC_SAC_AVG_R {
        ADC_SAC_AVG_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Hardware Averaging Control"]
    #[inline(always)]
    pub fn adc_sac_avg(&mut self) -> ADC_SAC_AVG_W {
        ADC_SAC_AVG_W { w: self }
    }
}
