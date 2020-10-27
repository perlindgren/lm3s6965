#[doc = "Reader of register ACSTAT0"]
pub type R = crate::R<u32, super::ACSTAT0>;
#[doc = "Writer for register ACSTAT0"]
pub type W = crate::W<u32, super::ACSTAT0>;
#[doc = "Register ACSTAT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ACSTAT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP_ACSTAT0_OVAL`"]
pub type COMP_ACSTAT0_OVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP_ACSTAT0_OVAL`"]
pub struct COMP_ACSTAT0_OVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACSTAT0_OVAL_W<'a> {
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
    #[doc = "Bit 1 - Comparator Output Value"]
    #[inline(always)]
    pub fn comp_acstat0_oval(&self) -> COMP_ACSTAT0_OVAL_R {
        COMP_ACSTAT0_OVAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator Output Value"]
    #[inline(always)]
    pub fn comp_acstat0_oval(&mut self) -> COMP_ACSTAT0_OVAL_W {
        COMP_ACSTAT0_OVAL_W { w: self }
    }
}
