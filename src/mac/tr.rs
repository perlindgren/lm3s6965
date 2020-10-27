#[doc = "Reader of register TR"]
pub type R = crate::R<u32, super::TR>;
#[doc = "Writer for register TR"]
pub type W = crate::W<u32, super::TR>;
#[doc = "Register TR `reset()`'s with value 0"]
impl crate::ResetValue for super::TR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_TR_NEWTX`"]
pub type MAC_TR_NEWTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_TR_NEWTX`"]
pub struct MAC_TR_NEWTX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_TR_NEWTX_W<'a> {
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
impl R {
    #[doc = "Bit 0 - New Transmission"]
    #[inline(always)]
    pub fn mac_tr_newtx(&self) -> MAC_TR_NEWTX_R {
        MAC_TR_NEWTX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - New Transmission"]
    #[inline(always)]
    pub fn mac_tr_newtx(&mut self) -> MAC_TR_NEWTX_W {
        MAC_TR_NEWTX_W { w: self }
    }
}
