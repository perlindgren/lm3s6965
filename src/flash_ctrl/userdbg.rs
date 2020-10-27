#[doc = "Reader of register USERDBG"]
pub type R = crate::R<u32, super::USERDBG>;
#[doc = "Writer for register USERDBG"]
pub type W = crate::W<u32, super::USERDBG>;
#[doc = "Register USERDBG `reset()`'s with value 0"]
impl crate::ResetValue for super::USERDBG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_USERDBG_DBG0`"]
pub type FLASH_USERDBG_DBG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_USERDBG_DBG0`"]
pub struct FLASH_USERDBG_DBG0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_USERDBG_DBG0_W<'a> {
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
#[doc = "Reader of field `FLASH_USERDBG_DBG1`"]
pub type FLASH_USERDBG_DBG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_USERDBG_DBG1`"]
pub struct FLASH_USERDBG_DBG1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_USERDBG_DBG1_W<'a> {
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
#[doc = "Reader of field `FLASH_USERDBG_DATA`"]
pub type FLASH_USERDBG_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FLASH_USERDBG_DATA`"]
pub struct FLASH_USERDBG_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_USERDBG_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 2)) | (((value as u32) & 0x1fff_ffff) << 2);
        self.w
    }
}
#[doc = "Reader of field `FLASH_USERDBG_NW`"]
pub type FLASH_USERDBG_NW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_USERDBG_NW`"]
pub struct FLASH_USERDBG_NW_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_USERDBG_NW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Debug Control 0"]
    #[inline(always)]
    pub fn flash_userdbg_dbg0(&self) -> FLASH_USERDBG_DBG0_R {
        FLASH_USERDBG_DBG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Debug Control 1"]
    #[inline(always)]
    pub fn flash_userdbg_dbg1(&self) -> FLASH_USERDBG_DBG1_R {
        FLASH_USERDBG_DBG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:30 - User Data"]
    #[inline(always)]
    pub fn flash_userdbg_data(&self) -> FLASH_USERDBG_DATA_R {
        FLASH_USERDBG_DATA_R::new(((self.bits >> 2) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 31 - User Debug Not Written"]
    #[inline(always)]
    pub fn flash_userdbg_nw(&self) -> FLASH_USERDBG_NW_R {
        FLASH_USERDBG_NW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Control 0"]
    #[inline(always)]
    pub fn flash_userdbg_dbg0(&mut self) -> FLASH_USERDBG_DBG0_W {
        FLASH_USERDBG_DBG0_W { w: self }
    }
    #[doc = "Bit 1 - Debug Control 1"]
    #[inline(always)]
    pub fn flash_userdbg_dbg1(&mut self) -> FLASH_USERDBG_DBG1_W {
        FLASH_USERDBG_DBG1_W { w: self }
    }
    #[doc = "Bits 2:30 - User Data"]
    #[inline(always)]
    pub fn flash_userdbg_data(&mut self) -> FLASH_USERDBG_DATA_W {
        FLASH_USERDBG_DATA_W { w: self }
    }
    #[doc = "Bit 31 - User Debug Not Written"]
    #[inline(always)]
    pub fn flash_userdbg_nw(&mut self) -> FLASH_USERDBG_NW_W {
        FLASH_USERDBG_NW_W { w: self }
    }
}
