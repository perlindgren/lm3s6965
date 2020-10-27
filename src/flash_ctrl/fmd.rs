#[doc = "Reader of register FMD"]
pub type R = crate::R<u32, super::FMD>;
#[doc = "Writer for register FMD"]
pub type W = crate::W<u32, super::FMD>;
#[doc = "Register FMD `reset()`'s with value 0"]
impl crate::ResetValue for super::FMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_FMD_DATA`"]
pub type FLASH_FMD_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FLASH_FMD_DATA`"]
pub struct FLASH_FMD_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_FMD_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data Value"]
    #[inline(always)]
    pub fn flash_fmd_data(&self) -> FLASH_FMD_DATA_R {
        FLASH_FMD_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Value"]
    #[inline(always)]
    pub fn flash_fmd_data(&mut self) -> FLASH_FMD_DATA_W {
        FLASH_FMD_DATA_W { w: self }
    }
}
