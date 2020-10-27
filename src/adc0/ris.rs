#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Writer for register RIS"]
pub type W = crate::W<u32, super::RIS>;
#[doc = "Register RIS `reset()`'s with value 0"]
impl crate::ResetValue for super::RIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_RIS_INR0`"]
pub type ADC_RIS_INR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_RIS_INR0`"]
pub struct ADC_RIS_INR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_RIS_INR0_W<'a> {
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
#[doc = "Reader of field `ADC_RIS_INR1`"]
pub type ADC_RIS_INR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_RIS_INR1`"]
pub struct ADC_RIS_INR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_RIS_INR1_W<'a> {
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
#[doc = "Reader of field `ADC_RIS_INR2`"]
pub type ADC_RIS_INR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_RIS_INR2`"]
pub struct ADC_RIS_INR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_RIS_INR2_W<'a> {
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
#[doc = "Reader of field `ADC_RIS_INR3`"]
pub type ADC_RIS_INR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_RIS_INR3`"]
pub struct ADC_RIS_INR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_RIS_INR3_W<'a> {
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
    #[doc = "Bit 0 - SS0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr0(&self) -> ADC_RIS_INR0_R {
        ADC_RIS_INR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SS1 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr1(&self) -> ADC_RIS_INR1_R {
        ADC_RIS_INR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SS2 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr2(&self) -> ADC_RIS_INR2_R {
        ADC_RIS_INR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SS3 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr3(&self) -> ADC_RIS_INR3_R {
        ADC_RIS_INR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr0(&mut self) -> ADC_RIS_INR0_W {
        ADC_RIS_INR0_W { w: self }
    }
    #[doc = "Bit 1 - SS1 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr1(&mut self) -> ADC_RIS_INR1_W {
        ADC_RIS_INR1_W { w: self }
    }
    #[doc = "Bit 2 - SS2 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr2(&mut self) -> ADC_RIS_INR2_W {
        ADC_RIS_INR2_W { w: self }
    }
    #[doc = "Bit 3 - SS3 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr3(&mut self) -> ADC_RIS_INR3_W {
        ADC_RIS_INR3_W { w: self }
    }
}
