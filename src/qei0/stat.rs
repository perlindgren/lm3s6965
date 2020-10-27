#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QEI_STAT_ERROR`"]
pub type QEI_STAT_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_STAT_ERROR`"]
pub struct QEI_STAT_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_STAT_ERROR_W<'a> {
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
#[doc = "Reader of field `QEI_STAT_DIRECTION`"]
pub type QEI_STAT_DIRECTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_STAT_DIRECTION`"]
pub struct QEI_STAT_DIRECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_STAT_DIRECTION_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Error Detected"]
    #[inline(always)]
    pub fn qei_stat_error(&self) -> QEI_STAT_ERROR_R {
        QEI_STAT_ERROR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Direction of Rotation"]
    #[inline(always)]
    pub fn qei_stat_direction(&self) -> QEI_STAT_DIRECTION_R {
        QEI_STAT_DIRECTION_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Detected"]
    #[inline(always)]
    pub fn qei_stat_error(&mut self) -> QEI_STAT_ERROR_W {
        QEI_STAT_ERROR_W { w: self }
    }
    #[doc = "Bit 1 - Direction of Rotation"]
    #[inline(always)]
    pub fn qei_stat_direction(&mut self) -> QEI_STAT_DIRECTION_W {
        QEI_STAT_DIRECTION_W { w: self }
    }
}
