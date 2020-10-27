#[doc = "Reader of register IM"]
pub type R = crate::R<u32, super::IM>;
#[doc = "Writer for register IM"]
pub type W = crate::W<u32, super::IM>;
#[doc = "Register IM `reset()`'s with value 0"]
impl crate::ResetValue for super::IM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_IM_MASK0`"]
pub type ADC_IM_MASK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_IM_MASK0`"]
pub struct ADC_IM_MASK0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_IM_MASK0_W<'a> {
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
#[doc = "Reader of field `ADC_IM_MASK1`"]
pub type ADC_IM_MASK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_IM_MASK1`"]
pub struct ADC_IM_MASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_IM_MASK1_W<'a> {
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
#[doc = "Reader of field `ADC_IM_MASK2`"]
pub type ADC_IM_MASK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_IM_MASK2`"]
pub struct ADC_IM_MASK2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_IM_MASK2_W<'a> {
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
#[doc = "Reader of field `ADC_IM_MASK3`"]
pub type ADC_IM_MASK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_IM_MASK3`"]
pub struct ADC_IM_MASK3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_IM_MASK3_W<'a> {
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
    #[doc = "Bit 0 - SS0 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask0(&self) -> ADC_IM_MASK0_R {
        ADC_IM_MASK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SS1 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask1(&self) -> ADC_IM_MASK1_R {
        ADC_IM_MASK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SS2 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask2(&self) -> ADC_IM_MASK2_R {
        ADC_IM_MASK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SS3 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask3(&self) -> ADC_IM_MASK3_R {
        ADC_IM_MASK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask0(&mut self) -> ADC_IM_MASK0_W {
        ADC_IM_MASK0_W { w: self }
    }
    #[doc = "Bit 1 - SS1 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask1(&mut self) -> ADC_IM_MASK1_W {
        ADC_IM_MASK1_W { w: self }
    }
    #[doc = "Bit 2 - SS2 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask2(&mut self) -> ADC_IM_MASK2_W {
        ADC_IM_MASK2_W { w: self }
    }
    #[doc = "Bit 3 - SS3 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask3(&mut self) -> ADC_IM_MASK3_W {
        ADC_IM_MASK3_W { w: self }
    }
}
