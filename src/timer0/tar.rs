#[doc = "Reader of register TAR"]
pub type R = crate::R<u32, super::TAR>;
#[doc = "Writer for register TAR"]
pub type W = crate::W<u32, super::TAR>;
#[doc = "Register TAR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER_TAR_TARL`"]
pub type TIMER_TAR_TARL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMER_TAR_TARL`"]
pub struct TIMER_TAR_TARL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TAR_TARL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `TIMER_TAR_TARH`"]
pub type TIMER_TAR_TARH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMER_TAR_TARH`"]
pub struct TIMER_TAR_TARH_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TAR_TARH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GPTM Timer A Register Low"]
    #[inline(always)]
    pub fn timer_tar_tarl(&self) -> TIMER_TAR_TARL_R {
        TIMER_TAR_TARL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - GPTM Timer A Register High"]
    #[inline(always)]
    pub fn timer_tar_tarh(&self) -> TIMER_TAR_TARH_R {
        TIMER_TAR_TARH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer A Register Low"]
    #[inline(always)]
    pub fn timer_tar_tarl(&mut self) -> TIMER_TAR_TARL_W {
        TIMER_TAR_TARL_W { w: self }
    }
    #[doc = "Bits 16:31 - GPTM Timer A Register High"]
    #[inline(always)]
    pub fn timer_tar_tarh(&mut self) -> TIMER_TAR_TARH_W {
        TIMER_TAR_TARH_W { w: self }
    }
}
