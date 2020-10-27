#[doc = "Reader of register ISC"]
pub type R = crate::R<u32, super::ISC>;
#[doc = "Writer for register ISC"]
pub type W = crate::W<u32, super::ISC>;
#[doc = "Register ISC `reset()`'s with value 0"]
impl crate::ResetValue for super::ISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QEI_ISC_INDEX`"]
pub type QEI_ISC_INDEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_ISC_INDEX`"]
pub struct QEI_ISC_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_ISC_INDEX_W<'a> {
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
#[doc = "Reader of field `QEI_ISC_TIMER`"]
pub type QEI_ISC_TIMER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_ISC_TIMER`"]
pub struct QEI_ISC_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_ISC_TIMER_W<'a> {
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
#[doc = "Reader of field `QEI_ISC_DIR`"]
pub type QEI_ISC_DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_ISC_DIR`"]
pub struct QEI_ISC_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_ISC_DIR_W<'a> {
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
#[doc = "Reader of field `QEI_ISC_ERROR`"]
pub type QEI_ISC_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_ISC_ERROR`"]
pub struct QEI_ISC_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_ISC_ERROR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Index Pulse Interrupt"]
    #[inline(always)]
    pub fn qei_isc_index(&self) -> QEI_ISC_INDEX_R {
        QEI_ISC_INDEX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Velocity Timer Expired Interrupt"]
    #[inline(always)]
    pub fn qei_isc_timer(&self) -> QEI_ISC_TIMER_R {
        QEI_ISC_TIMER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Direction Change Interrupt"]
    #[inline(always)]
    pub fn qei_isc_dir(&self) -> QEI_ISC_DIR_R {
        QEI_ISC_DIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Phase Error Interrupt"]
    #[inline(always)]
    pub fn qei_isc_error(&self) -> QEI_ISC_ERROR_R {
        QEI_ISC_ERROR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Index Pulse Interrupt"]
    #[inline(always)]
    pub fn qei_isc_index(&mut self) -> QEI_ISC_INDEX_W {
        QEI_ISC_INDEX_W { w: self }
    }
    #[doc = "Bit 1 - Velocity Timer Expired Interrupt"]
    #[inline(always)]
    pub fn qei_isc_timer(&mut self) -> QEI_ISC_TIMER_W {
        QEI_ISC_TIMER_W { w: self }
    }
    #[doc = "Bit 2 - Direction Change Interrupt"]
    #[inline(always)]
    pub fn qei_isc_dir(&mut self) -> QEI_ISC_DIR_W {
        QEI_ISC_DIR_W { w: self }
    }
    #[doc = "Bit 3 - Phase Error Interrupt"]
    #[inline(always)]
    pub fn qei_isc_error(&mut self) -> QEI_ISC_ERROR_W {
        QEI_ISC_ERROR_W { w: self }
    }
}
