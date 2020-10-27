#[doc = "Reader of register DCGC2"]
pub type R = crate::R<u32, super::DCGC2>;
#[doc = "Writer for register DCGC2"]
pub type W = crate::W<u32, super::DCGC2>;
#[doc = "Register DCGC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DCGC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCTL_DCGC2_GPIOA`"]
pub type SYSCTL_DCGC2_GPIOA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC2_GPIOA`"]
pub struct SYSCTL_DCGC2_GPIOA_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC2_GPIOA_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC2_GPIOB`"]
pub type SYSCTL_DCGC2_GPIOB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC2_GPIOB`"]
pub struct SYSCTL_DCGC2_GPIOB_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC2_GPIOB_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC2_GPIOC`"]
pub type SYSCTL_DCGC2_GPIOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC2_GPIOC`"]
pub struct SYSCTL_DCGC2_GPIOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC2_GPIOC_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC2_GPIOD`"]
pub type SYSCTL_DCGC2_GPIOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC2_GPIOD`"]
pub struct SYSCTL_DCGC2_GPIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC2_GPIOD_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC2_GPIOE`"]
pub type SYSCTL_DCGC2_GPIOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC2_GPIOE`"]
pub struct SYSCTL_DCGC2_GPIOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC2_GPIOE_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC2_GPIOF`"]
pub type SYSCTL_DCGC2_GPIOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC2_GPIOF`"]
pub struct SYSCTL_DCGC2_GPIOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC2_GPIOF_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC2_GPIOG`"]
pub type SYSCTL_DCGC2_GPIOG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC2_GPIOG`"]
pub struct SYSCTL_DCGC2_GPIOG_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC2_GPIOG_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC2_EMAC0`"]
pub type SYSCTL_DCGC2_EMAC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC2_EMAC0`"]
pub struct SYSCTL_DCGC2_EMAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC2_EMAC0_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC2_EPHY0`"]
pub type SYSCTL_DCGC2_EPHY0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC2_EPHY0`"]
pub struct SYSCTL_DCGC2_EPHY0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC2_EPHY0_W<'a> {
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
    #[doc = "Bit 0 - Port A Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpioa(&self) -> SYSCTL_DCGC2_GPIOA_R {
        SYSCTL_DCGC2_GPIOA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port B Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpiob(&self) -> SYSCTL_DCGC2_GPIOB_R {
        SYSCTL_DCGC2_GPIOB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port C Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpioc(&self) -> SYSCTL_DCGC2_GPIOC_R {
        SYSCTL_DCGC2_GPIOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port D Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpiod(&self) -> SYSCTL_DCGC2_GPIOD_R {
        SYSCTL_DCGC2_GPIOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port E Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpioe(&self) -> SYSCTL_DCGC2_GPIOE_R {
        SYSCTL_DCGC2_GPIOE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port F Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpiof(&self) -> SYSCTL_DCGC2_GPIOF_R {
        SYSCTL_DCGC2_GPIOF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port G Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpiog(&self) -> SYSCTL_DCGC2_GPIOG_R {
        SYSCTL_DCGC2_GPIOG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 28 - MAC0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_emac0(&self) -> SYSCTL_DCGC2_EMAC0_R {
        SYSCTL_DCGC2_EMAC0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - PHY0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_ephy0(&self) -> SYSCTL_DCGC2_EPHY0_R {
        SYSCTL_DCGC2_EPHY0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpioa(&mut self) -> SYSCTL_DCGC2_GPIOA_W {
        SYSCTL_DCGC2_GPIOA_W { w: self }
    }
    #[doc = "Bit 1 - Port B Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpiob(&mut self) -> SYSCTL_DCGC2_GPIOB_W {
        SYSCTL_DCGC2_GPIOB_W { w: self }
    }
    #[doc = "Bit 2 - Port C Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpioc(&mut self) -> SYSCTL_DCGC2_GPIOC_W {
        SYSCTL_DCGC2_GPIOC_W { w: self }
    }
    #[doc = "Bit 3 - Port D Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpiod(&mut self) -> SYSCTL_DCGC2_GPIOD_W {
        SYSCTL_DCGC2_GPIOD_W { w: self }
    }
    #[doc = "Bit 4 - Port E Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpioe(&mut self) -> SYSCTL_DCGC2_GPIOE_W {
        SYSCTL_DCGC2_GPIOE_W { w: self }
    }
    #[doc = "Bit 5 - Port F Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpiof(&mut self) -> SYSCTL_DCGC2_GPIOF_W {
        SYSCTL_DCGC2_GPIOF_W { w: self }
    }
    #[doc = "Bit 6 - Port G Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_gpiog(&mut self) -> SYSCTL_DCGC2_GPIOG_W {
        SYSCTL_DCGC2_GPIOG_W { w: self }
    }
    #[doc = "Bit 28 - MAC0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_emac0(&mut self) -> SYSCTL_DCGC2_EMAC0_W {
        SYSCTL_DCGC2_EMAC0_W { w: self }
    }
    #[doc = "Bit 30 - PHY0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc2_ephy0(&mut self) -> SYSCTL_DCGC2_EPHY0_W {
        SYSCTL_DCGC2_EPHY0_W { w: self }
    }
}
