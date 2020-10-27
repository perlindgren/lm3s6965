#[doc = "Reader of register ACREFCTL"]
pub type R = crate::R<u32, super::ACREFCTL>;
#[doc = "Writer for register ACREFCTL"]
pub type W = crate::W<u32, super::ACREFCTL>;
#[doc = "Register ACREFCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ACREFCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP_ACREFCTL_VREF`"]
pub type COMP_ACREFCTL_VREF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP_ACREFCTL_VREF`"]
pub struct COMP_ACREFCTL_VREF_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACREFCTL_VREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `COMP_ACREFCTL_RNG`"]
pub type COMP_ACREFCTL_RNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP_ACREFCTL_RNG`"]
pub struct COMP_ACREFCTL_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACREFCTL_RNG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `COMP_ACREFCTL_EN`"]
pub type COMP_ACREFCTL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP_ACREFCTL_EN`"]
pub struct COMP_ACREFCTL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACREFCTL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Resistor Ladder Voltage Ref"]
    #[inline(always)]
    pub fn comp_acrefctl_vref(&self) -> COMP_ACREFCTL_VREF_R {
        COMP_ACREFCTL_VREF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Resistor Ladder Range"]
    #[inline(always)]
    pub fn comp_acrefctl_rng(&self) -> COMP_ACREFCTL_RNG_R {
        COMP_ACREFCTL_RNG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Resistor Ladder Enable"]
    #[inline(always)]
    pub fn comp_acrefctl_en(&self) -> COMP_ACREFCTL_EN_R {
        COMP_ACREFCTL_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Resistor Ladder Voltage Ref"]
    #[inline(always)]
    pub fn comp_acrefctl_vref(&mut self) -> COMP_ACREFCTL_VREF_W {
        COMP_ACREFCTL_VREF_W { w: self }
    }
    #[doc = "Bit 8 - Resistor Ladder Range"]
    #[inline(always)]
    pub fn comp_acrefctl_rng(&mut self) -> COMP_ACREFCTL_RNG_W {
        COMP_ACREFCTL_RNG_W { w: self }
    }
    #[doc = "Bit 9 - Resistor Ladder Enable"]
    #[inline(always)]
    pub fn comp_acrefctl_en(&mut self) -> COMP_ACREFCTL_EN_W {
        COMP_ACREFCTL_EN_W { w: self }
    }
}
