#[doc = "Reader of register ACMIS"]
pub type R = crate::R<u32, super::ACMIS>;
#[doc = "Writer for register ACMIS"]
pub type W = crate::W<u32, super::ACMIS>;
#[doc = "Register ACMIS `reset()`'s with value 0"]
impl crate::ResetValue for super::ACMIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP_ACMIS_IN0`"]
pub type COMP_ACMIS_IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP_ACMIS_IN0`"]
pub struct COMP_ACMIS_IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACMIS_IN0_W<'a> {
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
#[doc = "Reader of field `COMP_ACMIS_IN1`"]
pub type COMP_ACMIS_IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP_ACMIS_IN1`"]
pub struct COMP_ACMIS_IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACMIS_IN1_W<'a> {
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
    #[doc = "Bit 0 - Comparator 0 Masked Interrupt Status"]
    #[inline(always)]
    pub fn comp_acmis_in0(&self) -> COMP_ACMIS_IN0_R {
        COMP_ACMIS_IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Masked Interrupt Status"]
    #[inline(always)]
    pub fn comp_acmis_in1(&self) -> COMP_ACMIS_IN1_R {
        COMP_ACMIS_IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Masked Interrupt Status"]
    #[inline(always)]
    pub fn comp_acmis_in0(&mut self) -> COMP_ACMIS_IN0_W {
        COMP_ACMIS_IN0_W { w: self }
    }
    #[doc = "Bit 1 - Comparator 1 Masked Interrupt Status"]
    #[inline(always)]
    pub fn comp_acmis_in1(&mut self) -> COMP_ACMIS_IN1_W {
        COMP_ACMIS_IN1_W { w: self }
    }
}
