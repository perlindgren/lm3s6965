#[doc = "Reader of register ACRIS"]
pub type R = crate::R<u32, super::ACRIS>;
#[doc = "Writer for register ACRIS"]
pub type W = crate::W<u32, super::ACRIS>;
#[doc = "Register ACRIS `reset()`'s with value 0"]
impl crate::ResetValue for super::ACRIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP_ACRIS_IN0`"]
pub type COMP_ACRIS_IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP_ACRIS_IN0`"]
pub struct COMP_ACRIS_IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACRIS_IN0_W<'a> {
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
#[doc = "Reader of field `COMP_ACRIS_IN1`"]
pub type COMP_ACRIS_IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP_ACRIS_IN1`"]
pub struct COMP_ACRIS_IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACRIS_IN1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Comparator 0 Interrupt Status"]
    #[inline(always)]
    pub fn comp_acris_in0(&self) -> COMP_ACRIS_IN0_R {
        COMP_ACRIS_IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Status"]
    #[inline(always)]
    pub fn comp_acris_in1(&self) -> COMP_ACRIS_IN1_R {
        COMP_ACRIS_IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Interrupt Status"]
    #[inline(always)]
    pub fn comp_acris_in0(&mut self) -> COMP_ACRIS_IN0_W {
        COMP_ACRIS_IN0_W { w: self }
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Status"]
    #[inline(always)]
    pub fn comp_acris_in1(&mut self) -> COMP_ACRIS_IN1_W {
        COMP_ACRIS_IN1_W { w: self }
    }
}
