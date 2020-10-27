#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Writer for register IMR"]
pub type W = crate::W<u32, super::IMR>;
#[doc = "Register IMR `reset()`'s with value 0"]
impl crate::ResetValue for super::IMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER_IMR_TATOIM`"]
pub type TIMER_IMR_TATOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_IMR_TATOIM`"]
pub struct TIMER_IMR_TATOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_IMR_TATOIM_W<'a> {
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
#[doc = "Reader of field `TIMER_IMR_CAMIM`"]
pub type TIMER_IMR_CAMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_IMR_CAMIM`"]
pub struct TIMER_IMR_CAMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_IMR_CAMIM_W<'a> {
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
#[doc = "Reader of field `TIMER_IMR_CAEIM`"]
pub type TIMER_IMR_CAEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_IMR_CAEIM`"]
pub struct TIMER_IMR_CAEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_IMR_CAEIM_W<'a> {
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
#[doc = "Reader of field `TIMER_IMR_RTCIM`"]
pub type TIMER_IMR_RTCIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_IMR_RTCIM`"]
pub struct TIMER_IMR_RTCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_IMR_RTCIM_W<'a> {
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
#[doc = "Reader of field `TIMER_IMR_TBTOIM`"]
pub type TIMER_IMR_TBTOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_IMR_TBTOIM`"]
pub struct TIMER_IMR_TBTOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_IMR_TBTOIM_W<'a> {
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
#[doc = "Reader of field `TIMER_IMR_CBMIM`"]
pub type TIMER_IMR_CBMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_IMR_CBMIM`"]
pub struct TIMER_IMR_CBMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_IMR_CBMIM_W<'a> {
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
#[doc = "Reader of field `TIMER_IMR_CBEIM`"]
pub type TIMER_IMR_CBEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_IMR_CBEIM`"]
pub struct TIMER_IMR_CBEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_IMR_CBEIM_W<'a> {
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
    #[doc = "Bit 0 - GPTM Timer A Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tatoim(&self) -> TIMER_IMR_TATOIM_R {
        TIMER_IMR_TATOIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPTM Capture A Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_camim(&self) -> TIMER_IMR_CAMIM_R {
        TIMER_IMR_CAMIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPTM Capture A Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_caeim(&self) -> TIMER_IMR_CAEIM_R {
        TIMER_IMR_CAEIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPTM RTC Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_rtcim(&self) -> TIMER_IMR_RTCIM_R {
        TIMER_IMR_RTCIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tbtoim(&self) -> TIMER_IMR_TBTOIM_R {
        TIMER_IMR_TBTOIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM Capture B Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbmim(&self) -> TIMER_IMR_CBMIM_R {
        TIMER_IMR_CBMIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTM Capture B Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbeim(&self) -> TIMER_IMR_CBEIM_R {
        TIMER_IMR_CBEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tatoim(&mut self) -> TIMER_IMR_TATOIM_W {
        TIMER_IMR_TATOIM_W { w: self }
    }
    #[doc = "Bit 1 - GPTM Capture A Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_camim(&mut self) -> TIMER_IMR_CAMIM_W {
        TIMER_IMR_CAMIM_W { w: self }
    }
    #[doc = "Bit 2 - GPTM Capture A Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_caeim(&mut self) -> TIMER_IMR_CAEIM_W {
        TIMER_IMR_CAEIM_W { w: self }
    }
    #[doc = "Bit 3 - GPTM RTC Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_rtcim(&mut self) -> TIMER_IMR_RTCIM_W {
        TIMER_IMR_RTCIM_W { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tbtoim(&mut self) -> TIMER_IMR_TBTOIM_W {
        TIMER_IMR_TBTOIM_W { w: self }
    }
    #[doc = "Bit 9 - GPTM Capture B Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbmim(&mut self) -> TIMER_IMR_CBMIM_W {
        TIMER_IMR_CBMIM_W { w: self }
    }
    #[doc = "Bit 10 - GPTM Capture B Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbeim(&mut self) -> TIMER_IMR_CBEIM_W {
        TIMER_IMR_CBEIM_W { w: self }
    }
}
