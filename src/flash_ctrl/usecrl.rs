#[doc = "Reader of register USECRL"]
pub type R = crate::R<u32, super::USECRL>;
#[doc = "Writer for register USECRL"]
pub type W = crate::W<u32, super::USECRL>;
#[doc = "Register USECRL `reset()`'s with value 0"]
impl crate::ResetValue for super::USECRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_USECRL`"]
pub type FLASH_USECRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLASH_USECRL`"]
pub struct FLASH_USECRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_USECRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Microsecond Reload Value"]
    #[inline(always)]
    pub fn flash_usecrl(&self) -> FLASH_USECRL_R {
        FLASH_USECRL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Microsecond Reload Value"]
    #[inline(always)]
    pub fn flash_usecrl(&mut self) -> FLASH_USECRL_W {
        FLASH_USECRL_W { w: self }
    }
}
