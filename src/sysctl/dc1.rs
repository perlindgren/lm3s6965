#[doc = "Reader of register DC1"]
pub type R = crate::R<u32, super::DC1>;
#[doc = "Writer for register DC1"]
pub type W = crate::W<u32, super::DC1>;
#[doc = "Register DC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCTL_DC1_JTAG`"]
pub type SYSCTL_DC1_JTAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC1_JTAG`"]
pub struct SYSCTL_DC1_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC1_JTAG_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC1_SWD`"]
pub type SYSCTL_DC1_SWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC1_SWD`"]
pub struct SYSCTL_DC1_SWD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC1_SWD_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC1_SWO`"]
pub type SYSCTL_DC1_SWO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC1_SWO`"]
pub struct SYSCTL_DC1_SWO_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC1_SWO_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC1_WDT0`"]
pub type SYSCTL_DC1_WDT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC1_WDT0`"]
pub struct SYSCTL_DC1_WDT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC1_WDT0_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC1_PLL`"]
pub type SYSCTL_DC1_PLL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC1_PLL`"]
pub struct SYSCTL_DC1_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC1_PLL_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC1_TEMP`"]
pub type SYSCTL_DC1_TEMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC1_TEMP`"]
pub struct SYSCTL_DC1_TEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC1_TEMP_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC1_HIB`"]
pub type SYSCTL_DC1_HIB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC1_HIB`"]
pub struct SYSCTL_DC1_HIB_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC1_HIB_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC1_MPU`"]
pub type SYSCTL_DC1_MPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC1_MPU`"]
pub struct SYSCTL_DC1_MPU_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC1_MPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "System Clock Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DC1_MINSYSDIV_A {
    #[doc = "3: Specifies a 50-MHz CPU clock with a PLL divider of 4"]
    SYSCTL_DC1_MINSYSDIV_50 = 3,
}
impl From<SYSCTL_DC1_MINSYSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DC1_MINSYSDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DC1_MINSYSDIV`"]
pub type SYSCTL_DC1_MINSYSDIV_R = crate::R<u8, SYSCTL_DC1_MINSYSDIV_A>;
impl SYSCTL_DC1_MINSYSDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_DC1_MINSYSDIV_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_50),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DC1_MINSYSDIV_50`"]
    #[inline(always)]
    pub fn is_sysctl_dc1_minsysdiv_50(&self) -> bool {
        *self == SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_50
    }
}
#[doc = "Write proxy for field `SYSCTL_DC1_MINSYSDIV`"]
pub struct SYSCTL_DC1_MINSYSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC1_MINSYSDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DC1_MINSYSDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Specifies a 50-MHz CPU clock with a PLL divider of 4"]
    #[inline(always)]
    pub fn sysctl_dc1_minsysdiv_50(self) -> &'a mut W {
        self.variant(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - JTAG Present"]
    #[inline(always)]
    pub fn sysctl_dc1_jtag(&self) -> SYSCTL_DC1_JTAG_R {
        SYSCTL_DC1_JTAG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SWD Present"]
    #[inline(always)]
    pub fn sysctl_dc1_swd(&self) -> SYSCTL_DC1_SWD_R {
        SYSCTL_DC1_SWD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SWO Trace Port Present"]
    #[inline(always)]
    pub fn sysctl_dc1_swo(&self) -> SYSCTL_DC1_SWO_R {
        SYSCTL_DC1_SWO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Watchdog Timer 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc1_wdt0(&self) -> SYSCTL_DC1_WDT0_R {
        SYSCTL_DC1_WDT0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PLL Present"]
    #[inline(always)]
    pub fn sysctl_dc1_pll(&self) -> SYSCTL_DC1_PLL_R {
        SYSCTL_DC1_PLL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Temp Sensor Present"]
    #[inline(always)]
    pub fn sysctl_dc1_temp(&self) -> SYSCTL_DC1_TEMP_R {
        SYSCTL_DC1_TEMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Hibernation Module Present"]
    #[inline(always)]
    pub fn sysctl_dc1_hib(&self) -> SYSCTL_DC1_HIB_R {
        SYSCTL_DC1_HIB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPU Present"]
    #[inline(always)]
    pub fn sysctl_dc1_mpu(&self) -> SYSCTL_DC1_MPU_R {
        SYSCTL_DC1_MPU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - System Clock Divider"]
    #[inline(always)]
    pub fn sysctl_dc1_minsysdiv(&self) -> SYSCTL_DC1_MINSYSDIV_R {
        SYSCTL_DC1_MINSYSDIV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - JTAG Present"]
    #[inline(always)]
    pub fn sysctl_dc1_jtag(&mut self) -> SYSCTL_DC1_JTAG_W {
        SYSCTL_DC1_JTAG_W { w: self }
    }
    #[doc = "Bit 1 - SWD Present"]
    #[inline(always)]
    pub fn sysctl_dc1_swd(&mut self) -> SYSCTL_DC1_SWD_W {
        SYSCTL_DC1_SWD_W { w: self }
    }
    #[doc = "Bit 2 - SWO Trace Port Present"]
    #[inline(always)]
    pub fn sysctl_dc1_swo(&mut self) -> SYSCTL_DC1_SWO_W {
        SYSCTL_DC1_SWO_W { w: self }
    }
    #[doc = "Bit 3 - Watchdog Timer 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc1_wdt0(&mut self) -> SYSCTL_DC1_WDT0_W {
        SYSCTL_DC1_WDT0_W { w: self }
    }
    #[doc = "Bit 4 - PLL Present"]
    #[inline(always)]
    pub fn sysctl_dc1_pll(&mut self) -> SYSCTL_DC1_PLL_W {
        SYSCTL_DC1_PLL_W { w: self }
    }
    #[doc = "Bit 5 - Temp Sensor Present"]
    #[inline(always)]
    pub fn sysctl_dc1_temp(&mut self) -> SYSCTL_DC1_TEMP_W {
        SYSCTL_DC1_TEMP_W { w: self }
    }
    #[doc = "Bit 6 - Hibernation Module Present"]
    #[inline(always)]
    pub fn sysctl_dc1_hib(&mut self) -> SYSCTL_DC1_HIB_W {
        SYSCTL_DC1_HIB_W { w: self }
    }
    #[doc = "Bit 7 - MPU Present"]
    #[inline(always)]
    pub fn sysctl_dc1_mpu(&mut self) -> SYSCTL_DC1_MPU_W {
        SYSCTL_DC1_MPU_W { w: self }
    }
    #[doc = "Bits 12:15 - System Clock Divider"]
    #[inline(always)]
    pub fn sysctl_dc1_minsysdiv(&mut self) -> SYSCTL_DC1_MINSYSDIV_W {
        SYSCTL_DC1_MINSYSDIV_W { w: self }
    }
}
