#[doc = "Reader of register FCRIS"]
pub type R = crate::R<u32, super::FCRIS>;
#[doc = "Writer for register FCRIS"]
pub type W = crate::W<u32, super::FCRIS>;
#[doc = "Register FCRIS `reset()`'s with value 0"]
impl crate::ResetValue for super::FCRIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_FCRIS_ARIS`"]
pub type FLASH_FCRIS_ARIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_FCRIS_ARIS`"]
pub struct FLASH_FCRIS_ARIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_FCRIS_ARIS_W<'a> {
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
#[doc = "Reader of field `FLASH_FCRIS_PRIS`"]
pub type FLASH_FCRIS_PRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_FCRIS_PRIS`"]
pub struct FLASH_FCRIS_PRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_FCRIS_PRIS_W<'a> {
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
    #[doc = "Bit 0 - Access Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_aris(&self) -> FLASH_FCRIS_ARIS_R {
        FLASH_FCRIS_ARIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Programming Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_pris(&self) -> FLASH_FCRIS_PRIS_R {
        FLASH_FCRIS_PRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_aris(&mut self) -> FLASH_FCRIS_ARIS_W {
        FLASH_FCRIS_ARIS_W { w: self }
    }
    #[doc = "Bit 1 - Programming Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_pris(&mut self) -> FLASH_FCRIS_PRIS_W {
        FLASH_FCRIS_PRIS_W { w: self }
    }
}
