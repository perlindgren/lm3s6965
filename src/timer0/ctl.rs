#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER_CTL_TAEN`"]
pub type TIMER_CTL_TAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_CTL_TAEN`"]
pub struct TIMER_CTL_TAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CTL_TAEN_W<'a> {
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
#[doc = "Reader of field `TIMER_CTL_TASTALL`"]
pub type TIMER_CTL_TASTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_CTL_TASTALL`"]
pub struct TIMER_CTL_TASTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CTL_TASTALL_W<'a> {
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
#[doc = "GPTM Timer A Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_CTL_TAEVENT_A {
    #[doc = "0: Positive edge"]
    TIMER_CTL_TAEVENT_POS = 0,
    #[doc = "1: Negative edge"]
    TIMER_CTL_TAEVENT_NEG = 1,
    #[doc = "3: Both edges"]
    TIMER_CTL_TAEVENT_BOTH = 3,
}
impl From<TIMER_CTL_TAEVENT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_CTL_TAEVENT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMER_CTL_TAEVENT`"]
pub type TIMER_CTL_TAEVENT_R = crate::R<u8, TIMER_CTL_TAEVENT_A>;
impl TIMER_CTL_TAEVENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMER_CTL_TAEVENT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_POS),
            1 => Val(TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_NEG),
            3 => Val(TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_BOTH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TAEVENT_POS`"]
    #[inline(always)]
    pub fn is_timer_ctl_taevent_pos(&self) -> bool {
        *self == TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_POS
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TAEVENT_NEG`"]
    #[inline(always)]
    pub fn is_timer_ctl_taevent_neg(&self) -> bool {
        *self == TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_NEG
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TAEVENT_BOTH`"]
    #[inline(always)]
    pub fn is_timer_ctl_taevent_both(&self) -> bool {
        *self == TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_BOTH
    }
}
#[doc = "Write proxy for field `TIMER_CTL_TAEVENT`"]
pub struct TIMER_CTL_TAEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CTL_TAEVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_CTL_TAEVENT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn timer_ctl_taevent_pos(self) -> &'a mut W {
        self.variant(TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_POS)
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn timer_ctl_taevent_neg(self) -> &'a mut W {
        self.variant(TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_NEG)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn timer_ctl_taevent_both(self) -> &'a mut W {
        self.variant(TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `TIMER_CTL_RTCEN`"]
pub type TIMER_CTL_RTCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_CTL_RTCEN`"]
pub struct TIMER_CTL_RTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CTL_RTCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TIMER_CTL_TAOTE`"]
pub type TIMER_CTL_TAOTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_CTL_TAOTE`"]
pub struct TIMER_CTL_TAOTE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CTL_TAOTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TIMER_CTL_TAPWML`"]
pub type TIMER_CTL_TAPWML_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_CTL_TAPWML`"]
pub struct TIMER_CTL_TAPWML_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CTL_TAPWML_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TIMER_CTL_TBEN`"]
pub type TIMER_CTL_TBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_CTL_TBEN`"]
pub struct TIMER_CTL_TBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CTL_TBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TIMER_CTL_TBSTALL`"]
pub type TIMER_CTL_TBSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_CTL_TBSTALL`"]
pub struct TIMER_CTL_TBSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CTL_TBSTALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "GPTM Timer B Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_CTL_TBEVENT_A {
    #[doc = "0: Positive edge"]
    TIMER_CTL_TBEVENT_POS = 0,
    #[doc = "1: Negative edge"]
    TIMER_CTL_TBEVENT_NEG = 1,
    #[doc = "3: Both edges"]
    TIMER_CTL_TBEVENT_BOTH = 3,
}
impl From<TIMER_CTL_TBEVENT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_CTL_TBEVENT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMER_CTL_TBEVENT`"]
pub type TIMER_CTL_TBEVENT_R = crate::R<u8, TIMER_CTL_TBEVENT_A>;
impl TIMER_CTL_TBEVENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMER_CTL_TBEVENT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_POS),
            1 => Val(TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_NEG),
            3 => Val(TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_BOTH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TBEVENT_POS`"]
    #[inline(always)]
    pub fn is_timer_ctl_tbevent_pos(&self) -> bool {
        *self == TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_POS
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TBEVENT_NEG`"]
    #[inline(always)]
    pub fn is_timer_ctl_tbevent_neg(&self) -> bool {
        *self == TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_NEG
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TBEVENT_BOTH`"]
    #[inline(always)]
    pub fn is_timer_ctl_tbevent_both(&self) -> bool {
        *self == TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_BOTH
    }
}
#[doc = "Write proxy for field `TIMER_CTL_TBEVENT`"]
pub struct TIMER_CTL_TBEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CTL_TBEVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_CTL_TBEVENT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn timer_ctl_tbevent_pos(self) -> &'a mut W {
        self.variant(TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_POS)
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn timer_ctl_tbevent_neg(self) -> &'a mut W {
        self.variant(TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_NEG)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn timer_ctl_tbevent_both(self) -> &'a mut W {
        self.variant(TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `TIMER_CTL_TBOTE`"]
pub type TIMER_CTL_TBOTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_CTL_TBOTE`"]
pub struct TIMER_CTL_TBOTE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CTL_TBOTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TIMER_CTL_TBPWML`"]
pub type TIMER_CTL_TBPWML_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_CTL_TBPWML`"]
pub struct TIMER_CTL_TBPWML_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CTL_TBPWML_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GPTM Timer A Enable"]
    #[inline(always)]
    pub fn timer_ctl_taen(&self) -> TIMER_CTL_TAEN_R {
        TIMER_CTL_TAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_tastall(&self) -> TIMER_CTL_TASTALL_R {
        TIMER_CTL_TASTALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - GPTM Timer A Event Mode"]
    #[inline(always)]
    pub fn timer_ctl_taevent(&self) -> TIMER_CTL_TAEVENT_R {
        TIMER_CTL_TAEVENT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPTM RTC Enable"]
    #[inline(always)]
    pub fn timer_ctl_rtcen(&self) -> TIMER_CTL_RTCEN_R {
        TIMER_CTL_RTCEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer A Output Trigger Enable"]
    #[inline(always)]
    pub fn timer_ctl_taote(&self) -> TIMER_CTL_TAOTE_R {
        TIMER_CTL_TAOTE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer A PWM Output Level"]
    #[inline(always)]
    pub fn timer_ctl_tapwml(&self) -> TIMER_CTL_TAPWML_R {
        TIMER_CTL_TAPWML_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Enable"]
    #[inline(always)]
    pub fn timer_ctl_tben(&self) -> TIMER_CTL_TBEN_R {
        TIMER_CTL_TBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_tbstall(&self) -> TIMER_CTL_TBSTALL_R {
        TIMER_CTL_TBSTALL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - GPTM Timer B Event Mode"]
    #[inline(always)]
    pub fn timer_ctl_tbevent(&self) -> TIMER_CTL_TBEVENT_R {
        TIMER_CTL_TBEVENT_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 13 - GPTM Timer B Output Trigger Enable"]
    #[inline(always)]
    pub fn timer_ctl_tbote(&self) -> TIMER_CTL_TBOTE_R {
        TIMER_CTL_TBOTE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPTM Timer B PWM Output Level"]
    #[inline(always)]
    pub fn timer_ctl_tbpwml(&self) -> TIMER_CTL_TBPWML_R {
        TIMER_CTL_TBPWML_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A Enable"]
    #[inline(always)]
    pub fn timer_ctl_taen(&mut self) -> TIMER_CTL_TAEN_W {
        TIMER_CTL_TAEN_W { w: self }
    }
    #[doc = "Bit 1 - GPTM Timer A Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_tastall(&mut self) -> TIMER_CTL_TASTALL_W {
        TIMER_CTL_TASTALL_W { w: self }
    }
    #[doc = "Bits 2:3 - GPTM Timer A Event Mode"]
    #[inline(always)]
    pub fn timer_ctl_taevent(&mut self) -> TIMER_CTL_TAEVENT_W {
        TIMER_CTL_TAEVENT_W { w: self }
    }
    #[doc = "Bit 4 - GPTM RTC Enable"]
    #[inline(always)]
    pub fn timer_ctl_rtcen(&mut self) -> TIMER_CTL_RTCEN_W {
        TIMER_CTL_RTCEN_W { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A Output Trigger Enable"]
    #[inline(always)]
    pub fn timer_ctl_taote(&mut self) -> TIMER_CTL_TAOTE_W {
        TIMER_CTL_TAOTE_W { w: self }
    }
    #[doc = "Bit 6 - GPTM Timer A PWM Output Level"]
    #[inline(always)]
    pub fn timer_ctl_tapwml(&mut self) -> TIMER_CTL_TAPWML_W {
        TIMER_CTL_TAPWML_W { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Enable"]
    #[inline(always)]
    pub fn timer_ctl_tben(&mut self) -> TIMER_CTL_TBEN_W {
        TIMER_CTL_TBEN_W { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_tbstall(&mut self) -> TIMER_CTL_TBSTALL_W {
        TIMER_CTL_TBSTALL_W { w: self }
    }
    #[doc = "Bits 10:11 - GPTM Timer B Event Mode"]
    #[inline(always)]
    pub fn timer_ctl_tbevent(&mut self) -> TIMER_CTL_TBEVENT_W {
        TIMER_CTL_TBEVENT_W { w: self }
    }
    #[doc = "Bit 13 - GPTM Timer B Output Trigger Enable"]
    #[inline(always)]
    pub fn timer_ctl_tbote(&mut self) -> TIMER_CTL_TBOTE_W {
        TIMER_CTL_TBOTE_W { w: self }
    }
    #[doc = "Bit 14 - GPTM Timer B PWM Output Level"]
    #[inline(always)]
    pub fn timer_ctl_tbpwml(&mut self) -> TIMER_CTL_TBPWML_W {
        TIMER_CTL_TBPWML_W { w: self }
    }
}
