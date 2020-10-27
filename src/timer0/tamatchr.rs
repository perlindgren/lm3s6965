#[doc = "Reader of register TAMATCHR"]
pub type R = crate::R<u32, super::TAMATCHR>;
#[doc = "Writer for register TAMATCHR"]
pub type W = crate::W<u32, super::TAMATCHR>;
#[doc = "Register TAMATCHR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAMATCHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER_TAMATCHR_TAMRL`"]
pub type TIMER_TAMATCHR_TAMRL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMER_TAMATCHR_TAMRL`"]
pub struct TIMER_TAMATCHR_TAMRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TAMATCHR_TAMRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `TIMER_TAMATCHR_TAMRH`"]
pub type TIMER_TAMATCHR_TAMRH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMER_TAMATCHR_TAMRH`"]
pub struct TIMER_TAMATCHR_TAMRH_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TAMATCHR_TAMRH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GPTM Timer A Match Register Low"]
    #[inline(always)]
    pub fn timer_tamatchr_tamrl(&self) -> TIMER_TAMATCHR_TAMRL_R {
        TIMER_TAMATCHR_TAMRL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - GPTM Timer A Match Register High"]
    #[inline(always)]
    pub fn timer_tamatchr_tamrh(&self) -> TIMER_TAMATCHR_TAMRH_R {
        TIMER_TAMATCHR_TAMRH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer A Match Register Low"]
    #[inline(always)]
    pub fn timer_tamatchr_tamrl(&mut self) -> TIMER_TAMATCHR_TAMRL_W {
        TIMER_TAMATCHR_TAMRL_W { w: self }
    }
    #[doc = "Bits 16:31 - GPTM Timer A Match Register High"]
    #[inline(always)]
    pub fn timer_tamatchr_tamrh(&mut self) -> TIMER_TAMATCHR_TAMRH_W {
        TIMER_TAMATCHR_TAMRH_W { w: self }
    }
}
