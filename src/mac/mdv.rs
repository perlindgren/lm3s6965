#[doc = "Reader of register MDV"]
pub type R = crate::R<u32, super::MDV>;
#[doc = "Writer for register MDV"]
pub type W = crate::W<u32, super::MDV>;
#[doc = "Register MDV `reset()`'s with value 0"]
impl crate::ResetValue for super::MDV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_MDV_DIV`"]
pub type MAC_MDV_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAC_MDV_DIV`"]
pub struct MAC_MDV_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_MDV_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    pub fn mac_mdv_div(&self) -> MAC_MDV_DIV_R {
        MAC_MDV_DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    pub fn mac_mdv_div(&mut self) -> MAC_MDV_DIV_W {
        MAC_MDV_DIV_W { w: self }
    }
}
