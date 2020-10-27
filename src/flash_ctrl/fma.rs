#[doc = "Reader of register FMA"]
pub type R = crate::R<u32, super::FMA>;
#[doc = "Writer for register FMA"]
pub type W = crate::W<u32, super::FMA>;
#[doc = "Register FMA `reset()`'s with value 0"]
impl crate::ResetValue for super::FMA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_FMA_OFFSET`"]
pub type FLASH_FMA_OFFSET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FLASH_FMA_OFFSET`"]
pub struct FLASH_FMA_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_FMA_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Address Offset"]
    #[inline(always)]
    pub fn flash_fma_offset(&self) -> FLASH_FMA_OFFSET_R {
        FLASH_FMA_OFFSET_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Address Offset"]
    #[inline(always)]
    pub fn flash_fma_offset(&mut self) -> FLASH_FMA_OFFSET_W {
        FLASH_FMA_OFFSET_W { w: self }
    }
}
