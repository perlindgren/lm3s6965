#[doc = "Reader of register USTAT"]
pub type R = crate::R<u32, super::USTAT>;
#[doc = "Writer for register USTAT"]
pub type W = crate::W<u32, super::USTAT>;
#[doc = "Register USTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::USTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_USTAT_UV0`"]
pub type ADC_USTAT_UV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_USTAT_UV0`"]
pub struct ADC_USTAT_UV0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_USTAT_UV0_W<'a> {
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
#[doc = "Reader of field `ADC_USTAT_UV1`"]
pub type ADC_USTAT_UV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_USTAT_UV1`"]
pub struct ADC_USTAT_UV1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_USTAT_UV1_W<'a> {
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
#[doc = "Reader of field `ADC_USTAT_UV2`"]
pub type ADC_USTAT_UV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_USTAT_UV2`"]
pub struct ADC_USTAT_UV2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_USTAT_UV2_W<'a> {
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
#[doc = "Reader of field `ADC_USTAT_UV3`"]
pub type ADC_USTAT_UV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_USTAT_UV3`"]
pub struct ADC_USTAT_UV3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_USTAT_UV3_W<'a> {
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
    #[doc = "Bit 0 - SS0 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv0(&self) -> ADC_USTAT_UV0_R {
        ADC_USTAT_UV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SS1 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv1(&self) -> ADC_USTAT_UV1_R {
        ADC_USTAT_UV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SS2 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv2(&self) -> ADC_USTAT_UV2_R {
        ADC_USTAT_UV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SS3 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv3(&self) -> ADC_USTAT_UV3_R {
        ADC_USTAT_UV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv0(&mut self) -> ADC_USTAT_UV0_W {
        ADC_USTAT_UV0_W { w: self }
    }
    #[doc = "Bit 1 - SS1 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv1(&mut self) -> ADC_USTAT_UV1_W {
        ADC_USTAT_UV1_W { w: self }
    }
    #[doc = "Bit 2 - SS2 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv2(&mut self) -> ADC_USTAT_UV2_W {
        ADC_USTAT_UV2_W { w: self }
    }
    #[doc = "Bit 3 - SS3 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv3(&mut self) -> ADC_USTAT_UV3_W {
        ADC_USTAT_UV3_W { w: self }
    }
}
