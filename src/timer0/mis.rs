#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Writer for register MIS"]
pub type W = crate::W<u32, super::MIS>;
#[doc = "Register MIS `reset()`'s with value 0"]
impl crate::ResetValue for super::MIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER_MIS_TATOMIS`"]
pub type TIMER_MIS_TATOMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_MIS_TATOMIS`"]
pub struct TIMER_MIS_TATOMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_MIS_TATOMIS_W<'a> {
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
#[doc = "Reader of field `TIMER_MIS_CAMMIS`"]
pub type TIMER_MIS_CAMMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_MIS_CAMMIS`"]
pub struct TIMER_MIS_CAMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_MIS_CAMMIS_W<'a> {
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
#[doc = "Reader of field `TIMER_MIS_CAEMIS`"]
pub type TIMER_MIS_CAEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_MIS_CAEMIS`"]
pub struct TIMER_MIS_CAEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_MIS_CAEMIS_W<'a> {
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
#[doc = "Reader of field `TIMER_MIS_RTCMIS`"]
pub type TIMER_MIS_RTCMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_MIS_RTCMIS`"]
pub struct TIMER_MIS_RTCMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_MIS_RTCMIS_W<'a> {
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
#[doc = "Reader of field `TIMER_MIS_TBTOMIS`"]
pub type TIMER_MIS_TBTOMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_MIS_TBTOMIS`"]
pub struct TIMER_MIS_TBTOMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_MIS_TBTOMIS_W<'a> {
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
#[doc = "Reader of field `TIMER_MIS_CBMMIS`"]
pub type TIMER_MIS_CBMMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_MIS_CBMMIS`"]
pub struct TIMER_MIS_CBMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_MIS_CBMMIS_W<'a> {
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
#[doc = "Reader of field `TIMER_MIS_CBEMIS`"]
pub type TIMER_MIS_CBEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_MIS_CBEMIS`"]
pub struct TIMER_MIS_CBEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_MIS_CBEMIS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tatomis(&self) -> TIMER_MIS_TATOMIS_R {
        TIMER_MIS_TATOMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPTM Capture A Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cammis(&self) -> TIMER_MIS_CAMMIS_R {
        TIMER_MIS_CAMMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPTM Capture A Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_caemis(&self) -> TIMER_MIS_CAEMIS_R {
        TIMER_MIS_CAEMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPTM RTC Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_rtcmis(&self) -> TIMER_MIS_RTCMIS_R {
        TIMER_MIS_RTCMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tbtomis(&self) -> TIMER_MIS_TBTOMIS_R {
        TIMER_MIS_TBTOMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM Capture B Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbmmis(&self) -> TIMER_MIS_CBMMIS_R {
        TIMER_MIS_CBMMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTM Capture B Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbemis(&self) -> TIMER_MIS_CBEMIS_R {
        TIMER_MIS_CBEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tatomis(&mut self) -> TIMER_MIS_TATOMIS_W {
        TIMER_MIS_TATOMIS_W { w: self }
    }
    #[doc = "Bit 1 - GPTM Capture A Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cammis(&mut self) -> TIMER_MIS_CAMMIS_W {
        TIMER_MIS_CAMMIS_W { w: self }
    }
    #[doc = "Bit 2 - GPTM Capture A Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_caemis(&mut self) -> TIMER_MIS_CAEMIS_W {
        TIMER_MIS_CAEMIS_W { w: self }
    }
    #[doc = "Bit 3 - GPTM RTC Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_rtcmis(&mut self) -> TIMER_MIS_RTCMIS_W {
        TIMER_MIS_RTCMIS_W { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tbtomis(&mut self) -> TIMER_MIS_TBTOMIS_W {
        TIMER_MIS_TBTOMIS_W { w: self }
    }
    #[doc = "Bit 9 - GPTM Capture B Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbmmis(&mut self) -> TIMER_MIS_CBMMIS_W {
        TIMER_MIS_CBMMIS_W { w: self }
    }
    #[doc = "Bit 10 - GPTM Capture B Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbemis(&mut self) -> TIMER_MIS_CBEMIS_W {
        TIMER_MIS_CBEMIS_W { w: self }
    }
}
