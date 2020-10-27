#[doc = "Reader of register TAILR"]
pub type R = crate::R<u32, super::TAILR>;
#[doc = "Writer for register TAILR"]
pub type W = crate::W<u32, super::TAILR>;
#[doc = "Register TAILR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAILR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER_TAILR_TAILRL`"]
pub type TIMER_TAILR_TAILRL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMER_TAILR_TAILRL`"]
pub struct TIMER_TAILR_TAILRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TAILR_TAILRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `TIMER_TAILR_TAILRH`"]
pub type TIMER_TAILR_TAILRH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMER_TAILR_TAILRH`"]
pub struct TIMER_TAILR_TAILRH_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TAILR_TAILRH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GPTM Timer A Interval Load Register Low"]
    #[inline(always)]
    pub fn timer_tailr_tailrl(&self) -> TIMER_TAILR_TAILRL_R {
        TIMER_TAILR_TAILRL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - GPTM Timer A Interval Load Register High"]
    #[inline(always)]
    pub fn timer_tailr_tailrh(&self) -> TIMER_TAILR_TAILRH_R {
        TIMER_TAILR_TAILRH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer A Interval Load Register Low"]
    #[inline(always)]
    pub fn timer_tailr_tailrl(&mut self) -> TIMER_TAILR_TAILRL_W {
        TIMER_TAILR_TAILRL_W { w: self }
    }
    #[doc = "Bits 16:31 - GPTM Timer A Interval Load Register High"]
    #[inline(always)]
    pub fn timer_tailr_tailrh(&mut self) -> TIMER_TAILR_TAILRH_W {
        TIMER_TAILR_TAILRH_W { w: self }
    }
}
