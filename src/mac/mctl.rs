#[doc = "Reader of register MCTL"]
pub type R = crate::R<u32, super::MCTL>;
#[doc = "Writer for register MCTL"]
pub type W = crate::W<u32, super::MCTL>;
#[doc = "Register MCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::MCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_MCTL_START`"]
pub type MAC_MCTL_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_MCTL_START`"]
pub struct MAC_MCTL_START_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_MCTL_START_W<'a> {
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
#[doc = "Reader of field `MAC_MCTL_WRITE`"]
pub type MAC_MCTL_WRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_MCTL_WRITE`"]
pub struct MAC_MCTL_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_MCTL_WRITE_W<'a> {
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
#[doc = "Reader of field `MAC_MCTL_REGADR`"]
pub type MAC_MCTL_REGADR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAC_MCTL_REGADR`"]
pub struct MAC_MCTL_REGADR_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_MCTL_REGADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MII Register Transaction Enable"]
    #[inline(always)]
    pub fn mac_mctl_start(&self) -> MAC_MCTL_START_R {
        MAC_MCTL_START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MII Register Transaction Type"]
    #[inline(always)]
    pub fn mac_mctl_write(&self) -> MAC_MCTL_WRITE_R {
        MAC_MCTL_WRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 3:7 - MII Register Address"]
    #[inline(always)]
    pub fn mac_mctl_regadr(&self) -> MAC_MCTL_REGADR_R {
        MAC_MCTL_REGADR_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MII Register Transaction Enable"]
    #[inline(always)]
    pub fn mac_mctl_start(&mut self) -> MAC_MCTL_START_W {
        MAC_MCTL_START_W { w: self }
    }
    #[doc = "Bit 1 - MII Register Transaction Type"]
    #[inline(always)]
    pub fn mac_mctl_write(&mut self) -> MAC_MCTL_WRITE_W {
        MAC_MCTL_WRITE_W { w: self }
    }
    #[doc = "Bits 3:7 - MII Register Address"]
    #[inline(always)]
    pub fn mac_mctl_regadr(&mut self) -> MAC_MCTL_REGADR_W {
        MAC_MCTL_REGADR_W { w: self }
    }
}
