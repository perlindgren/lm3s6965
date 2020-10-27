#[doc = "Reader of register RESC"]
pub type R = crate::R<u32, super::RESC>;
#[doc = "Writer for register RESC"]
pub type W = crate::W<u32, super::RESC>;
#[doc = "Register RESC `reset()`'s with value 0"]
impl crate::ResetValue for super::RESC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCTL_RESC_EXT`"]
pub type SYSCTL_RESC_EXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RESC_EXT`"]
pub struct SYSCTL_RESC_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RESC_EXT_W<'a> {
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
#[doc = "Reader of field `SYSCTL_RESC_POR`"]
pub type SYSCTL_RESC_POR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RESC_POR`"]
pub struct SYSCTL_RESC_POR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RESC_POR_W<'a> {
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
#[doc = "Reader of field `SYSCTL_RESC_BOR`"]
pub type SYSCTL_RESC_BOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RESC_BOR`"]
pub struct SYSCTL_RESC_BOR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RESC_BOR_W<'a> {
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
#[doc = "Reader of field `SYSCTL_RESC_SW`"]
pub type SYSCTL_RESC_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RESC_SW`"]
pub struct SYSCTL_RESC_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RESC_SW_W<'a> {
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
impl R {
    #[doc = "Bit 0 - External Reset"]
    #[inline(always)]
    pub fn sysctl_resc_ext(&self) -> SYSCTL_RESC_EXT_R {
        SYSCTL_RESC_EXT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power-On Reset"]
    #[inline(always)]
    pub fn sysctl_resc_por(&self) -> SYSCTL_RESC_POR_R {
        SYSCTL_RESC_POR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Brown-Out Reset"]
    #[inline(always)]
    pub fn sysctl_resc_bor(&self) -> SYSCTL_RESC_BOR_R {
        SYSCTL_RESC_BOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software Reset"]
    #[inline(always)]
    pub fn sysctl_resc_sw(&self) -> SYSCTL_RESC_SW_R {
        SYSCTL_RESC_SW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Reset"]
    #[inline(always)]
    pub fn sysctl_resc_ext(&mut self) -> SYSCTL_RESC_EXT_W {
        SYSCTL_RESC_EXT_W { w: self }
    }
    #[doc = "Bit 1 - Power-On Reset"]
    #[inline(always)]
    pub fn sysctl_resc_por(&mut self) -> SYSCTL_RESC_POR_W {
        SYSCTL_RESC_POR_W { w: self }
    }
    #[doc = "Bit 2 - Brown-Out Reset"]
    #[inline(always)]
    pub fn sysctl_resc_bor(&mut self) -> SYSCTL_RESC_BOR_W {
        SYSCTL_RESC_BOR_W { w: self }
    }
    #[doc = "Bit 4 - Software Reset"]
    #[inline(always)]
    pub fn sysctl_resc_sw(&mut self) -> SYSCTL_RESC_SW_W {
        SYSCTL_RESC_SW_W { w: self }
    }
}
