#[doc = "Reader of register FMC"]
pub type R = crate::R<u32, super::FMC>;
#[doc = "Writer for register FMC"]
pub type W = crate::W<u32, super::FMC>;
#[doc = "Register FMC `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_FMC_WRITE`"]
pub type FLASH_FMC_WRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_FMC_WRITE`"]
pub struct FLASH_FMC_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_FMC_WRITE_W<'a> {
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
#[doc = "Reader of field `FLASH_FMC_ERASE`"]
pub type FLASH_FMC_ERASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_FMC_ERASE`"]
pub struct FLASH_FMC_ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_FMC_ERASE_W<'a> {
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
#[doc = "Reader of field `FLASH_FMC_MERASE`"]
pub type FLASH_FMC_MERASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_FMC_MERASE`"]
pub struct FLASH_FMC_MERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_FMC_MERASE_W<'a> {
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
#[doc = "Reader of field `FLASH_FMC_COMT`"]
pub type FLASH_FMC_COMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_FMC_COMT`"]
pub struct FLASH_FMC_COMT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_FMC_COMT_W<'a> {
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
#[doc = "Reader of field `FLASH_FMC_WRKEY`"]
pub type FLASH_FMC_WRKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FLASH_FMC_WRKEY`"]
pub struct FLASH_FMC_WRKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_FMC_WRKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | (((value as u32) & 0x7fff) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write a Word into Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_write(&self) -> FLASH_FMC_WRITE_R {
        FLASH_FMC_WRITE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Erase a Page of Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_erase(&self) -> FLASH_FMC_ERASE_R {
        FLASH_FMC_ERASE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mass Erase Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_merase(&self) -> FLASH_FMC_MERASE_R {
        FLASH_FMC_MERASE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Commit Register Value"]
    #[inline(always)]
    pub fn flash_fmc_comt(&self) -> FLASH_FMC_COMT_R {
        FLASH_FMC_COMT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 17:31 - FLASH write key"]
    #[inline(always)]
    pub fn flash_fmc_wrkey(&self) -> FLASH_FMC_WRKEY_R {
        FLASH_FMC_WRKEY_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Write a Word into Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_write(&mut self) -> FLASH_FMC_WRITE_W {
        FLASH_FMC_WRITE_W { w: self }
    }
    #[doc = "Bit 1 - Erase a Page of Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_erase(&mut self) -> FLASH_FMC_ERASE_W {
        FLASH_FMC_ERASE_W { w: self }
    }
    #[doc = "Bit 2 - Mass Erase Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_merase(&mut self) -> FLASH_FMC_MERASE_W {
        FLASH_FMC_MERASE_W { w: self }
    }
    #[doc = "Bit 3 - Commit Register Value"]
    #[inline(always)]
    pub fn flash_fmc_comt(&mut self) -> FLASH_FMC_COMT_W {
        FLASH_FMC_COMT_W { w: self }
    }
    #[doc = "Bits 17:31 - FLASH write key"]
    #[inline(always)]
    pub fn flash_fmc_wrkey(&mut self) -> FLASH_FMC_WRKEY_W {
        FLASH_FMC_WRKEY_W { w: self }
    }
}
