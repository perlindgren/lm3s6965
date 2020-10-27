#[doc = "Reader of register OSTAT"]
pub type R = crate::R<u32, super::OSTAT>;
#[doc = "Writer for register OSTAT"]
pub type W = crate::W<u32, super::OSTAT>;
#[doc = "Register OSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::OSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_OSTAT_OV0`"]
pub type ADC_OSTAT_OV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_OSTAT_OV0`"]
pub struct ADC_OSTAT_OV0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OSTAT_OV0_W<'a> {
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
#[doc = "Reader of field `ADC_OSTAT_OV1`"]
pub type ADC_OSTAT_OV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_OSTAT_OV1`"]
pub struct ADC_OSTAT_OV1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OSTAT_OV1_W<'a> {
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
#[doc = "Reader of field `ADC_OSTAT_OV2`"]
pub type ADC_OSTAT_OV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_OSTAT_OV2`"]
pub struct ADC_OSTAT_OV2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OSTAT_OV2_W<'a> {
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
#[doc = "Reader of field `ADC_OSTAT_OV3`"]
pub type ADC_OSTAT_OV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_OSTAT_OV3`"]
pub struct ADC_OSTAT_OV3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OSTAT_OV3_W<'a> {
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
    #[doc = "Bit 0 - SS0 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov0(&self) -> ADC_OSTAT_OV0_R {
        ADC_OSTAT_OV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SS1 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov1(&self) -> ADC_OSTAT_OV1_R {
        ADC_OSTAT_OV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SS2 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov2(&self) -> ADC_OSTAT_OV2_R {
        ADC_OSTAT_OV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SS3 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov3(&self) -> ADC_OSTAT_OV3_R {
        ADC_OSTAT_OV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov0(&mut self) -> ADC_OSTAT_OV0_W {
        ADC_OSTAT_OV0_W { w: self }
    }
    #[doc = "Bit 1 - SS1 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov1(&mut self) -> ADC_OSTAT_OV1_W {
        ADC_OSTAT_OV1_W { w: self }
    }
    #[doc = "Bit 2 - SS2 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov2(&mut self) -> ADC_OSTAT_OV2_W {
        ADC_OSTAT_OV2_W { w: self }
    }
    #[doc = "Bit 3 - SS3 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov3(&mut self) -> ADC_OSTAT_OV3_W {
        ADC_OSTAT_OV3_W { w: self }
    }
}
