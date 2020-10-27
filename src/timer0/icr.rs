#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TIMER_ICR_TATOCINT`"]
pub struct TIMER_ICR_TATOCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_ICR_TATOCINT_W<'a> {
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
#[doc = "Write proxy for field `TIMER_ICR_CAMCINT`"]
pub struct TIMER_ICR_CAMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_ICR_CAMCINT_W<'a> {
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
#[doc = "Write proxy for field `TIMER_ICR_CAECINT`"]
pub struct TIMER_ICR_CAECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_ICR_CAECINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `TIMER_ICR_RTCCINT`"]
pub struct TIMER_ICR_RTCCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_ICR_RTCCINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `TIMER_ICR_TBTOCINT`"]
pub struct TIMER_ICR_TBTOCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_ICR_TBTOCINT_W<'a> {
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
#[doc = "Write proxy for field `TIMER_ICR_CBMCINT`"]
pub struct TIMER_ICR_CBMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_ICR_CBMCINT_W<'a> {
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
#[doc = "Write proxy for field `TIMER_ICR_CBECINT`"]
pub struct TIMER_ICR_CBECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_ICR_CBECINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_icr_tatocint(&mut self) -> TIMER_ICR_TATOCINT_W {
        TIMER_ICR_TATOCINT_W { w: self }
    }
    #[doc = "Bit 1 - GPTM Capture A Match Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_camcint(&mut self) -> TIMER_ICR_CAMCINT_W {
        TIMER_ICR_CAMCINT_W { w: self }
    }
    #[doc = "Bit 2 - GPTM Capture A Event Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_caecint(&mut self) -> TIMER_ICR_CAECINT_W {
        TIMER_ICR_CAECINT_W { w: self }
    }
    #[doc = "Bit 3 - GPTM RTC Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_rtccint(&mut self) -> TIMER_ICR_RTCCINT_W {
        TIMER_ICR_RTCCINT_W { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_tbtocint(&mut self) -> TIMER_ICR_TBTOCINT_W {
        TIMER_ICR_TBTOCINT_W { w: self }
    }
    #[doc = "Bit 9 - GPTM Capture B Match Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_cbmcint(&mut self) -> TIMER_ICR_CBMCINT_W {
        TIMER_ICR_CBMCINT_W { w: self }
    }
    #[doc = "Bit 10 - GPTM Capture B Event Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_cbecint(&mut self) -> TIMER_ICR_CBECINT_W {
        TIMER_ICR_CBECINT_W { w: self }
    }
}
