#[doc = "Reader of register SSCTL3"]
pub type R = crate::R<u32, super::SSCTL3>;
#[doc = "Writer for register SSCTL3"]
pub type W = crate::W<u32, super::SSCTL3>;
#[doc = "Register SSCTL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSCTL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_SSCTL3_D0`"]
pub type ADC_SSCTL3_D0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL3_D0`"]
pub struct ADC_SSCTL3_D0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL3_D0_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL3_END0`"]
pub type ADC_SSCTL3_END0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL3_END0`"]
pub struct ADC_SSCTL3_END0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL3_END0_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL3_IE0`"]
pub type ADC_SSCTL3_IE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL3_IE0`"]
pub struct ADC_SSCTL3_IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL3_IE0_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL3_TS0`"]
pub type ADC_SSCTL3_TS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL3_TS0`"]
pub struct ADC_SSCTL3_TS0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL3_TS0_W<'a> {
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
    #[doc = "Bit 0 - 1st Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl3_d0(&self) -> ADC_SSCTL3_D0_R {
        ADC_SSCTL3_D0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl3_end0(&self) -> ADC_SSCTL3_END0_R {
        ADC_SSCTL3_END0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl3_ie0(&self) -> ADC_SSCTL3_IE0_R {
        ADC_SSCTL3_IE0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl3_ts0(&self) -> ADC_SSCTL3_TS0_R {
        ADC_SSCTL3_TS0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl3_d0(&mut self) -> ADC_SSCTL3_D0_W {
        ADC_SSCTL3_D0_W { w: self }
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl3_end0(&mut self) -> ADC_SSCTL3_END0_W {
        ADC_SSCTL3_END0_W { w: self }
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl3_ie0(&mut self) -> ADC_SSCTL3_IE0_W {
        ADC_SSCTL3_IE0_W { w: self }
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl3_ts0(&mut self) -> ADC_SSCTL3_TS0_W {
        ADC_SSCTL3_TS0_W { w: self }
    }
}
