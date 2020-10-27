#[doc = "Reader of register IA1"]
pub type R = crate::R<u32, super::IA1>;
#[doc = "Writer for register IA1"]
pub type W = crate::W<u32, super::IA1>;
#[doc = "Register IA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_IA1_MACOCT5`"]
pub type MAC_IA1_MACOCT5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAC_IA1_MACOCT5`"]
pub struct MAC_IA1_MACOCT5_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IA1_MACOCT5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `MAC_IA1_MACOCT6`"]
pub type MAC_IA1_MACOCT6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAC_IA1_MACOCT6`"]
pub struct MAC_IA1_MACOCT6_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IA1_MACOCT6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - MAC Address Octet 5"]
    #[inline(always)]
    pub fn mac_ia1_macoct5(&self) -> MAC_IA1_MACOCT5_R {
        MAC_IA1_MACOCT5_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MAC Address Octet 6"]
    #[inline(always)]
    pub fn mac_ia1_macoct6(&self) -> MAC_IA1_MACOCT6_R {
        MAC_IA1_MACOCT6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MAC Address Octet 5"]
    #[inline(always)]
    pub fn mac_ia1_macoct5(&mut self) -> MAC_IA1_MACOCT5_W {
        MAC_IA1_MACOCT5_W { w: self }
    }
    #[doc = "Bits 8:15 - MAC Address Octet 6"]
    #[inline(always)]
    pub fn mac_ia1_macoct6(&mut self) -> MAC_IA1_MACOCT6_W {
        MAC_IA1_MACOCT6_W { w: self }
    }
}
