#[doc = "Reader of register FCIM"]
pub type R = crate::R<u32, super::FCIM>;
#[doc = "Writer for register FCIM"]
pub type W = crate::W<u32, super::FCIM>;
#[doc = "Register FCIM `reset()`'s with value 0"]
impl crate::ResetValue for super::FCIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_FCIM_AMASK`"]
pub type FLASH_FCIM_AMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_FCIM_AMASK`"]
pub struct FLASH_FCIM_AMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_FCIM_AMASK_W<'a> {
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
#[doc = "Reader of field `FLASH_FCIM_PMASK`"]
pub type FLASH_FCIM_PMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_FCIM_PMASK`"]
pub struct FLASH_FCIM_PMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_FCIM_PMASK_W<'a> {
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
    #[doc = "Bit 0 - Access Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_amask(&self) -> FLASH_FCIM_AMASK_R {
        FLASH_FCIM_AMASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Programming Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_pmask(&self) -> FLASH_FCIM_PMASK_R {
        FLASH_FCIM_PMASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_amask(&mut self) -> FLASH_FCIM_AMASK_W {
        FLASH_FCIM_AMASK_W { w: self }
    }
    #[doc = "Bit 1 - Programming Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_pmask(&mut self) -> FLASH_FCIM_PMASK_W {
        FLASH_FCIM_PMASK_W { w: self }
    }
}
