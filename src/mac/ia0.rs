#[doc = "Reader of register IA0"]
pub type R = crate::R<u32, super::IA0>;
#[doc = "Writer for register IA0"]
pub type W = crate::W<u32, super::IA0>;
#[doc = "Register IA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IA0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAC_IA0_MACOCT1`"]
pub type MAC_IA0_MACOCT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAC_IA0_MACOCT1`"]
pub struct MAC_IA0_MACOCT1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IA0_MACOCT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `MAC_IA0_MACOCT2`"]
pub type MAC_IA0_MACOCT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAC_IA0_MACOCT2`"]
pub struct MAC_IA0_MACOCT2_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IA0_MACOCT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `MAC_IA0_MACOCT3`"]
pub type MAC_IA0_MACOCT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAC_IA0_MACOCT3`"]
pub struct MAC_IA0_MACOCT3_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IA0_MACOCT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `MAC_IA0_MACOCT4`"]
pub type MAC_IA0_MACOCT4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAC_IA0_MACOCT4`"]
pub struct MAC_IA0_MACOCT4_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_IA0_MACOCT4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - MAC Address Octet 1"]
    #[inline(always)]
    pub fn mac_ia0_macoct1(&self) -> MAC_IA0_MACOCT1_R {
        MAC_IA0_MACOCT1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MAC Address Octet 2"]
    #[inline(always)]
    pub fn mac_ia0_macoct2(&self) -> MAC_IA0_MACOCT2_R {
        MAC_IA0_MACOCT2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MAC Address Octet 3"]
    #[inline(always)]
    pub fn mac_ia0_macoct3(&self) -> MAC_IA0_MACOCT3_R {
        MAC_IA0_MACOCT3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - MAC Address Octet 4"]
    #[inline(always)]
    pub fn mac_ia0_macoct4(&self) -> MAC_IA0_MACOCT4_R {
        MAC_IA0_MACOCT4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MAC Address Octet 1"]
    #[inline(always)]
    pub fn mac_ia0_macoct1(&mut self) -> MAC_IA0_MACOCT1_W {
        MAC_IA0_MACOCT1_W { w: self }
    }
    #[doc = "Bits 8:15 - MAC Address Octet 2"]
    #[inline(always)]
    pub fn mac_ia0_macoct2(&mut self) -> MAC_IA0_MACOCT2_W {
        MAC_IA0_MACOCT2_W { w: self }
    }
    #[doc = "Bits 16:23 - MAC Address Octet 3"]
    #[inline(always)]
    pub fn mac_ia0_macoct3(&mut self) -> MAC_IA0_MACOCT3_W {
        MAC_IA0_MACOCT3_W { w: self }
    }
    #[doc = "Bits 24:31 - MAC Address Octet 4"]
    #[inline(always)]
    pub fn mac_ia0_macoct4(&mut self) -> MAC_IA0_MACOCT4_W {
        MAC_IA0_MACOCT4_W { w: self }
    }
}
