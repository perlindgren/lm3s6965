#[doc = "Reader of register ISC"]
pub type R = crate::R<u32, super::ISC>;
#[doc = "Writer for register ISC"]
pub type W = crate::W<u32, super::ISC>;
#[doc = "Register ISC `reset()`'s with value 0"]
impl crate::ResetValue for super::ISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_ISC_IN0`"]
pub type ADC_ISC_IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_ISC_IN0`"]
pub struct ADC_ISC_IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ISC_IN0_W<'a> {
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
#[doc = "Reader of field `ADC_ISC_IN1`"]
pub type ADC_ISC_IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_ISC_IN1`"]
pub struct ADC_ISC_IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ISC_IN1_W<'a> {
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
#[doc = "Reader of field `ADC_ISC_IN2`"]
pub type ADC_ISC_IN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_ISC_IN2`"]
pub struct ADC_ISC_IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ISC_IN2_W<'a> {
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
#[doc = "Reader of field `ADC_ISC_IN3`"]
pub type ADC_ISC_IN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_ISC_IN3`"]
pub struct ADC_ISC_IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ISC_IN3_W<'a> {
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
    #[doc = "Bit 0 - SS0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in0(&self) -> ADC_ISC_IN0_R {
        ADC_ISC_IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SS1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in1(&self) -> ADC_ISC_IN1_R {
        ADC_ISC_IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SS2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in2(&self) -> ADC_ISC_IN2_R {
        ADC_ISC_IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SS3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in3(&self) -> ADC_ISC_IN3_R {
        ADC_ISC_IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in0(&mut self) -> ADC_ISC_IN0_W {
        ADC_ISC_IN0_W { w: self }
    }
    #[doc = "Bit 1 - SS1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in1(&mut self) -> ADC_ISC_IN1_W {
        ADC_ISC_IN1_W { w: self }
    }
    #[doc = "Bit 2 - SS2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in2(&mut self) -> ADC_ISC_IN2_W {
        ADC_ISC_IN2_W { w: self }
    }
    #[doc = "Bit 3 - SS3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in3(&mut self) -> ADC_ISC_IN3_W {
        ADC_ISC_IN3_W { w: self }
    }
}
