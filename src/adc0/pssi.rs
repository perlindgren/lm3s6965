#[doc = "Reader of register PSSI"]
pub type R = crate::R<u32, super::PSSI>;
#[doc = "Writer for register PSSI"]
pub type W = crate::W<u32, super::PSSI>;
#[doc = "Register PSSI `reset()`'s with value 0"]
impl crate::ResetValue for super::PSSI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_PSSI_SS0`"]
pub type ADC_PSSI_SS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_PSSI_SS0`"]
pub struct ADC_PSSI_SS0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PSSI_SS0_W<'a> {
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
#[doc = "Reader of field `ADC_PSSI_SS1`"]
pub type ADC_PSSI_SS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_PSSI_SS1`"]
pub struct ADC_PSSI_SS1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PSSI_SS1_W<'a> {
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
#[doc = "Reader of field `ADC_PSSI_SS2`"]
pub type ADC_PSSI_SS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_PSSI_SS2`"]
pub struct ADC_PSSI_SS2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PSSI_SS2_W<'a> {
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
#[doc = "Reader of field `ADC_PSSI_SS3`"]
pub type ADC_PSSI_SS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_PSSI_SS3`"]
pub struct ADC_PSSI_SS3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PSSI_SS3_W<'a> {
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
    #[doc = "Bit 0 - SS0 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss0(&self) -> ADC_PSSI_SS0_R {
        ADC_PSSI_SS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SS1 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss1(&self) -> ADC_PSSI_SS1_R {
        ADC_PSSI_SS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SS2 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss2(&self) -> ADC_PSSI_SS2_R {
        ADC_PSSI_SS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SS3 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss3(&self) -> ADC_PSSI_SS3_R {
        ADC_PSSI_SS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss0(&mut self) -> ADC_PSSI_SS0_W {
        ADC_PSSI_SS0_W { w: self }
    }
    #[doc = "Bit 1 - SS1 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss1(&mut self) -> ADC_PSSI_SS1_W {
        ADC_PSSI_SS1_W { w: self }
    }
    #[doc = "Bit 2 - SS2 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss2(&mut self) -> ADC_PSSI_SS2_W {
        ADC_PSSI_SS2_W { w: self }
    }
    #[doc = "Bit 3 - SS3 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss3(&mut self) -> ADC_PSSI_SS3_W {
        ADC_PSSI_SS3_W { w: self }
    }
}
