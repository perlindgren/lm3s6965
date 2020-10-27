#[doc = "Reader of register DC4"]
pub type R = crate::R<u32, super::DC4>;
#[doc = "Writer for register DC4"]
pub type W = crate::W<u32, super::DC4>;
#[doc = "Register DC4 `reset()`'s with value 0"]
impl crate::ResetValue for super::DC4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCTL_DC4_GPIOA`"]
pub type SYSCTL_DC4_GPIOA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC4_GPIOA`"]
pub struct SYSCTL_DC4_GPIOA_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC4_GPIOA_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC4_GPIOB`"]
pub type SYSCTL_DC4_GPIOB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC4_GPIOB`"]
pub struct SYSCTL_DC4_GPIOB_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC4_GPIOB_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC4_GPIOC`"]
pub type SYSCTL_DC4_GPIOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC4_GPIOC`"]
pub struct SYSCTL_DC4_GPIOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC4_GPIOC_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC4_GPIOD`"]
pub type SYSCTL_DC4_GPIOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC4_GPIOD`"]
pub struct SYSCTL_DC4_GPIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC4_GPIOD_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC4_GPIOE`"]
pub type SYSCTL_DC4_GPIOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC4_GPIOE`"]
pub struct SYSCTL_DC4_GPIOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC4_GPIOE_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC4_GPIOF`"]
pub type SYSCTL_DC4_GPIOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC4_GPIOF`"]
pub struct SYSCTL_DC4_GPIOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC4_GPIOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DC4_GPIOG`"]
pub type SYSCTL_DC4_GPIOG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC4_GPIOG`"]
pub struct SYSCTL_DC4_GPIOG_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC4_GPIOG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DC4_EMAC0`"]
pub type SYSCTL_DC4_EMAC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC4_EMAC0`"]
pub struct SYSCTL_DC4_EMAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC4_EMAC0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DC4_EPHY0`"]
pub type SYSCTL_DC4_EPHY0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC4_EPHY0`"]
pub struct SYSCTL_DC4_EPHY0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC4_EPHY0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GPIO Port A Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpioa(&self) -> SYSCTL_DC4_GPIOA_R {
        SYSCTL_DC4_GPIOA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpiob(&self) -> SYSCTL_DC4_GPIOB_R {
        SYSCTL_DC4_GPIOB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpioc(&self) -> SYSCTL_DC4_GPIOC_R {
        SYSCTL_DC4_GPIOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpiod(&self) -> SYSCTL_DC4_GPIOD_R {
        SYSCTL_DC4_GPIOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpioe(&self) -> SYSCTL_DC4_GPIOE_R {
        SYSCTL_DC4_GPIOE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpiof(&self) -> SYSCTL_DC4_GPIOF_R {
        SYSCTL_DC4_GPIOF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO Port G Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpiog(&self) -> SYSCTL_DC4_GPIOG_R {
        SYSCTL_DC4_GPIOG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Ethernet MAC Layer 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc4_emac0(&self) -> SYSCTL_DC4_EMAC0_R {
        SYSCTL_DC4_EMAC0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Ethernet PHY Layer 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc4_ephy0(&self) -> SYSCTL_DC4_EPHY0_R {
        SYSCTL_DC4_EPHY0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpioa(&mut self) -> SYSCTL_DC4_GPIOA_W {
        SYSCTL_DC4_GPIOA_W { w: self }
    }
    #[doc = "Bit 1 - GPIO Port B Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpiob(&mut self) -> SYSCTL_DC4_GPIOB_W {
        SYSCTL_DC4_GPIOB_W { w: self }
    }
    #[doc = "Bit 2 - GPIO Port C Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpioc(&mut self) -> SYSCTL_DC4_GPIOC_W {
        SYSCTL_DC4_GPIOC_W { w: self }
    }
    #[doc = "Bit 3 - GPIO Port D Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpiod(&mut self) -> SYSCTL_DC4_GPIOD_W {
        SYSCTL_DC4_GPIOD_W { w: self }
    }
    #[doc = "Bit 4 - GPIO Port E Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpioe(&mut self) -> SYSCTL_DC4_GPIOE_W {
        SYSCTL_DC4_GPIOE_W { w: self }
    }
    #[doc = "Bit 5 - GPIO Port F Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpiof(&mut self) -> SYSCTL_DC4_GPIOF_W {
        SYSCTL_DC4_GPIOF_W { w: self }
    }
    #[doc = "Bit 6 - GPIO Port G Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpiog(&mut self) -> SYSCTL_DC4_GPIOG_W {
        SYSCTL_DC4_GPIOG_W { w: self }
    }
    #[doc = "Bit 28 - Ethernet MAC Layer 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc4_emac0(&mut self) -> SYSCTL_DC4_EMAC0_W {
        SYSCTL_DC4_EMAC0_W { w: self }
    }
    #[doc = "Bit 30 - Ethernet PHY Layer 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc4_ephy0(&mut self) -> SYSCTL_DC4_EPHY0_W {
        SYSCTL_DC4_EPHY0_W { w: self }
    }
}
