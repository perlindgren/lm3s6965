#[doc = "Reader of register DC0"]
pub type R = crate::R<u32, super::DC0>;
#[doc = "Writer for register DC0"]
pub type W = crate::W<u32, super::DC0>;
#[doc = "Register DC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Flash Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SYSCTL_DC0_FLASHSZ_A {
    #[doc = "127: 256 KB of Flash"]
    SYSCTL_DC0_FLASHSZ_256K = 127,
}
impl From<SYSCTL_DC0_FLASHSZ_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSCTL_DC0_FLASHSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DC0_FLASHSZ`"]
pub type SYSCTL_DC0_FLASHSZ_R = crate::R<u16, SYSCTL_DC0_FLASHSZ_A>;
impl SYSCTL_DC0_FLASHSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SYSCTL_DC0_FLASHSZ_A> {
        use crate::Variant::*;
        match self.bits {
            127 => Val(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_256K),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DC0_FLASHSZ_256K`"]
    #[inline(always)]
    pub fn is_sysctl_dc0_flashsz_256k(&self) -> bool {
        *self == SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_256K
    }
}
#[doc = "Write proxy for field `SYSCTL_DC0_FLASHSZ`"]
pub struct SYSCTL_DC0_FLASHSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC0_FLASHSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DC0_FLASHSZ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256 KB of Flash"]
    #[inline(always)]
    pub fn sysctl_dc0_flashsz_256k(self) -> &'a mut W {
        self.variant(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_256K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "SRAM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SYSCTL_DC0_SRAMSZ_A {
    #[doc = "255: 64 KB of SRAM"]
    SYSCTL_DC0_SRAMSZ_64KB = 255,
}
impl From<SYSCTL_DC0_SRAMSZ_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSCTL_DC0_SRAMSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DC0_SRAMSZ`"]
pub type SYSCTL_DC0_SRAMSZ_R = crate::R<u16, SYSCTL_DC0_SRAMSZ_A>;
impl SYSCTL_DC0_SRAMSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SYSCTL_DC0_SRAMSZ_A> {
        use crate::Variant::*;
        match self.bits {
            255 => Val(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_64KB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DC0_SRAMSZ_64KB`"]
    #[inline(always)]
    pub fn is_sysctl_dc0_sramsz_64kb(&self) -> bool {
        *self == SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_64KB
    }
}
#[doc = "Write proxy for field `SYSCTL_DC0_SRAMSZ`"]
pub struct SYSCTL_DC0_SRAMSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC0_SRAMSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DC0_SRAMSZ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "64 KB of SRAM"]
    #[inline(always)]
    pub fn sysctl_dc0_sramsz_64kb(self) -> &'a mut W {
        self.variant(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_64KB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    pub fn sysctl_dc0_flashsz(&self) -> SYSCTL_DC0_FLASHSZ_R {
        SYSCTL_DC0_FLASHSZ_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRAM Size"]
    #[inline(always)]
    pub fn sysctl_dc0_sramsz(&self) -> SYSCTL_DC0_SRAMSZ_R {
        SYSCTL_DC0_SRAMSZ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    pub fn sysctl_dc0_flashsz(&mut self) -> SYSCTL_DC0_FLASHSZ_W {
        SYSCTL_DC0_FLASHSZ_W { w: self }
    }
    #[doc = "Bits 16:31 - SRAM Size"]
    #[inline(always)]
    pub fn sysctl_dc0_sramsz(&mut self) -> SYSCTL_DC0_SRAMSZ_W {
        SYSCTL_DC0_SRAMSZ_W { w: self }
    }
}
