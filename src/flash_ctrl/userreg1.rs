#[doc = "Reader of register USERREG1"]
pub type R = crate::R<u32, super::USERREG1>;
#[doc = "Writer for register USERREG1"]
pub type W = crate::W<u32, super::USERREG1>;
#[doc = "Register USERREG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::USERREG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_USERREG1_DATA`"]
pub type FLASH_USERREG1_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FLASH_USERREG1_DATA`"]
pub struct FLASH_USERREG1_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_USERREG1_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `FLASH_USERREG1_NW`"]
pub type FLASH_USERREG1_NW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_USERREG1_NW`"]
pub struct FLASH_USERREG1_NW_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_USERREG1_NW_W<'a> {
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
    #[doc = "Bits 0:30 - User Data"]
    #[inline(always)]
    pub fn flash_userreg1_data(&self) -> FLASH_USERREG1_DATA_R {
        FLASH_USERREG1_DATA_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Not Written"]
    #[inline(always)]
    pub fn flash_userreg1_nw(&self) -> FLASH_USERREG1_NW_R {
        FLASH_USERREG1_NW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - User Data"]
    #[inline(always)]
    pub fn flash_userreg1_data(&mut self) -> FLASH_USERREG1_DATA_W {
        FLASH_USERREG1_DATA_W { w: self }
    }
    #[doc = "Bit 31 - Not Written"]
    #[inline(always)]
    pub fn flash_userreg1_nw(&mut self) -> FLASH_USERREG1_NW_W {
        FLASH_USERREG1_NW_W { w: self }
    }
}
