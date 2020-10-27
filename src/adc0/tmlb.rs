#[doc = "Reader of register TMLB"]
pub type R = crate::R<u32, super::TMLB>;
#[doc = "Writer for register TMLB"]
pub type W = crate::W<u32, super::TMLB>;
#[doc = "Register TMLB `reset()`'s with value 0"]
impl crate::ResetValue for super::TMLB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_TMLB_LB`"]
pub type ADC_TMLB_LB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_TMLB_LB`"]
pub struct ADC_TMLB_LB_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_TMLB_LB_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Loopback Mode Enable"]
    #[inline(always)]
    pub fn adc_tmlb_lb(&self) -> ADC_TMLB_LB_R {
        ADC_TMLB_LB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Loopback Mode Enable"]
    #[inline(always)]
    pub fn adc_tmlb_lb(&mut self) -> ADC_TMLB_LB_W {
        ADC_TMLB_LB_W { w: self }
    }
}
