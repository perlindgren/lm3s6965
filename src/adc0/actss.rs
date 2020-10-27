#[doc = "Reader of register ACTSS"]
pub type R = crate::R<u32, super::ACTSS>;
#[doc = "Writer for register ACTSS"]
pub type W = crate::W<u32, super::ACTSS>;
#[doc = "Register ACTSS `reset()`'s with value 0"]
impl crate::ResetValue for super::ACTSS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_ACTSS_ASEN0`"]
pub type ADC_ACTSS_ASEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_ACTSS_ASEN0`"]
pub struct ADC_ACTSS_ASEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ACTSS_ASEN0_W<'a> {
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
#[doc = "Reader of field `ADC_ACTSS_ASEN1`"]
pub type ADC_ACTSS_ASEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_ACTSS_ASEN1`"]
pub struct ADC_ACTSS_ASEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ACTSS_ASEN1_W<'a> {
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
#[doc = "Reader of field `ADC_ACTSS_ASEN2`"]
pub type ADC_ACTSS_ASEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_ACTSS_ASEN2`"]
pub struct ADC_ACTSS_ASEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ACTSS_ASEN2_W<'a> {
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
#[doc = "Reader of field `ADC_ACTSS_ASEN3`"]
pub type ADC_ACTSS_ASEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_ACTSS_ASEN3`"]
pub struct ADC_ACTSS_ASEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ACTSS_ASEN3_W<'a> {
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
    #[doc = "Bit 0 - ADC SS0 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen0(&self) -> ADC_ACTSS_ASEN0_R {
        ADC_ACTSS_ASEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC SS1 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen1(&self) -> ADC_ACTSS_ASEN1_R {
        ADC_ACTSS_ASEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC SS2 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen2(&self) -> ADC_ACTSS_ASEN2_R {
        ADC_ACTSS_ASEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC SS3 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen3(&self) -> ADC_ACTSS_ASEN3_R {
        ADC_ACTSS_ASEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC SS0 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen0(&mut self) -> ADC_ACTSS_ASEN0_W {
        ADC_ACTSS_ASEN0_W { w: self }
    }
    #[doc = "Bit 1 - ADC SS1 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen1(&mut self) -> ADC_ACTSS_ASEN1_W {
        ADC_ACTSS_ASEN1_W { w: self }
    }
    #[doc = "Bit 2 - ADC SS2 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen2(&mut self) -> ADC_ACTSS_ASEN2_W {
        ADC_ACTSS_ASEN2_W { w: self }
    }
    #[doc = "Bit 3 - ADC SS3 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen3(&mut self) -> ADC_ACTSS_ASEN3_W {
        ADC_ACTSS_ASEN3_W { w: self }
    }
}
