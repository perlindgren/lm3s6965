#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QEI_INTEN_INDEX`"]
pub type QEI_INTEN_INDEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_INTEN_INDEX`"]
pub struct QEI_INTEN_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_INTEN_INDEX_W<'a> {
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
#[doc = "Reader of field `QEI_INTEN_TIMER`"]
pub type QEI_INTEN_TIMER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_INTEN_TIMER`"]
pub struct QEI_INTEN_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_INTEN_TIMER_W<'a> {
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
#[doc = "Reader of field `QEI_INTEN_DIR`"]
pub type QEI_INTEN_DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_INTEN_DIR`"]
pub struct QEI_INTEN_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_INTEN_DIR_W<'a> {
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
#[doc = "Reader of field `QEI_INTEN_ERROR`"]
pub type QEI_INTEN_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_INTEN_ERROR`"]
pub struct QEI_INTEN_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_INTEN_ERROR_W<'a> {
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
    #[doc = "Bit 0 - Index Pulse Detected Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_index(&self) -> QEI_INTEN_INDEX_R {
        QEI_INTEN_INDEX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer Expires Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_timer(&self) -> QEI_INTEN_TIMER_R {
        QEI_INTEN_TIMER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Direction Change Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_dir(&self) -> QEI_INTEN_DIR_R {
        QEI_INTEN_DIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Phase Error Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_error(&self) -> QEI_INTEN_ERROR_R {
        QEI_INTEN_ERROR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Index Pulse Detected Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_index(&mut self) -> QEI_INTEN_INDEX_W {
        QEI_INTEN_INDEX_W { w: self }
    }
    #[doc = "Bit 1 - Timer Expires Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_timer(&mut self) -> QEI_INTEN_TIMER_W {
        QEI_INTEN_TIMER_W { w: self }
    }
    #[doc = "Bit 2 - Direction Change Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_dir(&mut self) -> QEI_INTEN_DIR_W {
        QEI_INTEN_DIR_W { w: self }
    }
    #[doc = "Bit 3 - Phase Error Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_error(&mut self) -> QEI_INTEN_ERROR_W {
        QEI_INTEN_ERROR_W { w: self }
    }
}
