#[doc = "Reader of register FCMISC"]
pub type R = crate::R<u32, super::FCMISC>;
#[doc = "Writer for register FCMISC"]
pub type W = crate::W<u32, super::FCMISC>;
#[doc = "Register FCMISC `reset()`'s with value 0"]
impl crate::ResetValue for super::FCMISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_FCMISC_AMISC`"]
pub type FLASH_FCMISC_AMISC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_FCMISC_AMISC`"]
pub struct FLASH_FCMISC_AMISC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_FCMISC_AMISC_W<'a> {
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
#[doc = "Reader of field `FLASH_FCMISC_PMISC`"]
pub type FLASH_FCMISC_PMISC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_FCMISC_PMISC`"]
pub struct FLASH_FCMISC_PMISC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_FCMISC_PMISC_W<'a> {
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
    #[doc = "Bit 0 - Access Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_amisc(&self) -> FLASH_FCMISC_AMISC_R {
        FLASH_FCMISC_AMISC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Programming Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_pmisc(&self) -> FLASH_FCMISC_PMISC_R {
        FLASH_FCMISC_PMISC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_amisc(&mut self) -> FLASH_FCMISC_AMISC_W {
        FLASH_FCMISC_AMISC_W { w: self }
    }
    #[doc = "Bit 1 - Programming Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_pmisc(&mut self) -> FLASH_FCMISC_PMISC_W {
        FLASH_FCMISC_PMISC_W { w: self }
    }
}
