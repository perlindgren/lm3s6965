#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Writer for register RIS"]
pub type W = crate::W<u32, super::RIS>;
#[doc = "Register RIS `reset()`'s with value 0"]
impl crate::ResetValue for super::RIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER_RIS_TATORIS`"]
pub type TIMER_RIS_TATORIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_RIS_TATORIS`"]
pub struct TIMER_RIS_TATORIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_RIS_TATORIS_W<'a> {
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
#[doc = "Reader of field `TIMER_RIS_CAMRIS`"]
pub type TIMER_RIS_CAMRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_RIS_CAMRIS`"]
pub struct TIMER_RIS_CAMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_RIS_CAMRIS_W<'a> {
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
#[doc = "Reader of field `TIMER_RIS_CAERIS`"]
pub type TIMER_RIS_CAERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_RIS_CAERIS`"]
pub struct TIMER_RIS_CAERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_RIS_CAERIS_W<'a> {
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
#[doc = "Reader of field `TIMER_RIS_RTCRIS`"]
pub type TIMER_RIS_RTCRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_RIS_RTCRIS`"]
pub struct TIMER_RIS_RTCRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_RIS_RTCRIS_W<'a> {
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
#[doc = "Reader of field `TIMER_RIS_TBTORIS`"]
pub type TIMER_RIS_TBTORIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_RIS_TBTORIS`"]
pub struct TIMER_RIS_TBTORIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_RIS_TBTORIS_W<'a> {
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
#[doc = "Reader of field `TIMER_RIS_CBMRIS`"]
pub type TIMER_RIS_CBMRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_RIS_CBMRIS`"]
pub struct TIMER_RIS_CBMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_RIS_CBMRIS_W<'a> {
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
#[doc = "Reader of field `TIMER_RIS_CBERIS`"]
pub type TIMER_RIS_CBERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_RIS_CBERIS`"]
pub struct TIMER_RIS_CBERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_RIS_CBERIS_W<'a> {
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
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tatoris(&self) -> TIMER_RIS_TATORIS_R {
        TIMER_RIS_TATORIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPTM Capture A Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_camris(&self) -> TIMER_RIS_CAMRIS_R {
        TIMER_RIS_CAMRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPTM Capture A Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_caeris(&self) -> TIMER_RIS_CAERIS_R {
        TIMER_RIS_CAERIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPTM RTC Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_rtcris(&self) -> TIMER_RIS_RTCRIS_R {
        TIMER_RIS_RTCRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tbtoris(&self) -> TIMER_RIS_TBTORIS_R {
        TIMER_RIS_TBTORIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM Capture B Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cbmris(&self) -> TIMER_RIS_CBMRIS_R {
        TIMER_RIS_CBMRIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTM Capture B Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cberis(&self) -> TIMER_RIS_CBERIS_R {
        TIMER_RIS_CBERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tatoris(&mut self) -> TIMER_RIS_TATORIS_W {
        TIMER_RIS_TATORIS_W { w: self }
    }
    #[doc = "Bit 1 - GPTM Capture A Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_camris(&mut self) -> TIMER_RIS_CAMRIS_W {
        TIMER_RIS_CAMRIS_W { w: self }
    }
    #[doc = "Bit 2 - GPTM Capture A Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_caeris(&mut self) -> TIMER_RIS_CAERIS_W {
        TIMER_RIS_CAERIS_W { w: self }
    }
    #[doc = "Bit 3 - GPTM RTC Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_rtcris(&mut self) -> TIMER_RIS_RTCRIS_W {
        TIMER_RIS_RTCRIS_W { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tbtoris(&mut self) -> TIMER_RIS_TBTORIS_W {
        TIMER_RIS_TBTORIS_W { w: self }
    }
    #[doc = "Bit 9 - GPTM Capture B Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cbmris(&mut self) -> TIMER_RIS_CBMRIS_W {
        TIMER_RIS_CBMRIS_W { w: self }
    }
    #[doc = "Bit 10 - GPTM Capture B Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cberis(&mut self) -> TIMER_RIS_CBERIS_W {
        TIMER_RIS_CBERIS_W { w: self }
    }
}
