#[doc = "Reader of register CR0"]
pub type R = crate::R<u32, super::CR0>;
#[doc = "Writer for register CR0"]
pub type W = crate::W<u32, super::CR0>;
#[doc = "Register CR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SSI Data Size Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSI_CR0_DSS_A {
    #[doc = "3: 4-bit data"]
    SSI_CR0_DSS_4 = 3,
    #[doc = "4: 5-bit data"]
    SSI_CR0_DSS_5 = 4,
    #[doc = "5: 6-bit data"]
    SSI_CR0_DSS_6 = 5,
    #[doc = "6: 7-bit data"]
    SSI_CR0_DSS_7 = 6,
    #[doc = "7: 8-bit data"]
    SSI_CR0_DSS_8 = 7,
    #[doc = "8: 9-bit data"]
    SSI_CR0_DSS_9 = 8,
    #[doc = "9: 10-bit data"]
    SSI_CR0_DSS_10 = 9,
    #[doc = "10: 11-bit data"]
    SSI_CR0_DSS_11 = 10,
    #[doc = "11: 12-bit data"]
    SSI_CR0_DSS_12 = 11,
    #[doc = "12: 13-bit data"]
    SSI_CR0_DSS_13 = 12,
    #[doc = "13: 14-bit data"]
    SSI_CR0_DSS_14 = 13,
    #[doc = "14: 15-bit data"]
    SSI_CR0_DSS_15 = 14,
    #[doc = "15: 16-bit data"]
    SSI_CR0_DSS_16 = 15,
}
impl From<SSI_CR0_DSS_A> for u8 {
    #[inline(always)]
    fn from(variant: SSI_CR0_DSS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSI_CR0_DSS`"]
pub type SSI_CR0_DSS_R = crate::R<u8, SSI_CR0_DSS_A>;
impl SSI_CR0_DSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSI_CR0_DSS_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(SSI_CR0_DSS_A::SSI_CR0_DSS_4),
            4 => Val(SSI_CR0_DSS_A::SSI_CR0_DSS_5),
            5 => Val(SSI_CR0_DSS_A::SSI_CR0_DSS_6),
            6 => Val(SSI_CR0_DSS_A::SSI_CR0_DSS_7),
            7 => Val(SSI_CR0_DSS_A::SSI_CR0_DSS_8),
            8 => Val(SSI_CR0_DSS_A::SSI_CR0_DSS_9),
            9 => Val(SSI_CR0_DSS_A::SSI_CR0_DSS_10),
            10 => Val(SSI_CR0_DSS_A::SSI_CR0_DSS_11),
            11 => Val(SSI_CR0_DSS_A::SSI_CR0_DSS_12),
            12 => Val(SSI_CR0_DSS_A::SSI_CR0_DSS_13),
            13 => Val(SSI_CR0_DSS_A::SSI_CR0_DSS_14),
            14 => Val(SSI_CR0_DSS_A::SSI_CR0_DSS_15),
            15 => Val(SSI_CR0_DSS_A::SSI_CR0_DSS_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_4`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_4(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_4
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_5`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_5(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_5
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_6`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_6(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_6
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_7`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_7(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_7
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_8`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_8(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_8
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_9`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_9(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_9
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_10`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_10(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_10
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_11`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_11(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_11
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_12`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_12(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_12
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_13`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_13(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_13
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_14`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_14(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_14
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_15`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_15(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_15
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_16`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_16(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_16
    }
}
#[doc = "Write proxy for field `SSI_CR0_DSS`"]
pub struct SSI_CR0_DSS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_CR0_DSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSI_CR0_DSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_4(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_4)
    }
    #[doc = "5-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_5(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_5)
    }
    #[doc = "6-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_6(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_6)
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_7(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_7)
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_8(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_8)
    }
    #[doc = "9-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_9(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_9)
    }
    #[doc = "10-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_10(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_10)
    }
    #[doc = "11-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_11(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_11)
    }
    #[doc = "12-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_12(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_12)
    }
    #[doc = "13-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_13(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_13)
    }
    #[doc = "14-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_14(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_14)
    }
    #[doc = "15-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_15(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_15)
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_16(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "SSI Frame Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSI_CR0_FRF_A {
    #[doc = "0: Freescale SPI Frame Format"]
    SSI_CR0_FRF_MOTO = 0,
    #[doc = "1: Texas Instruments Synchronous Serial Frame Format"]
    SSI_CR0_FRF_TI = 1,
    #[doc = "2: MICROWIRE Frame Format"]
    SSI_CR0_FRF_NMW = 2,
}
impl From<SSI_CR0_FRF_A> for u8 {
    #[inline(always)]
    fn from(variant: SSI_CR0_FRF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSI_CR0_FRF`"]
pub type SSI_CR0_FRF_R = crate::R<u8, SSI_CR0_FRF_A>;
impl SSI_CR0_FRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSI_CR0_FRF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SSI_CR0_FRF_A::SSI_CR0_FRF_MOTO),
            1 => Val(SSI_CR0_FRF_A::SSI_CR0_FRF_TI),
            2 => Val(SSI_CR0_FRF_A::SSI_CR0_FRF_NMW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_FRF_MOTO`"]
    #[inline(always)]
    pub fn is_ssi_cr0_frf_moto(&self) -> bool {
        *self == SSI_CR0_FRF_A::SSI_CR0_FRF_MOTO
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_FRF_TI`"]
    #[inline(always)]
    pub fn is_ssi_cr0_frf_ti(&self) -> bool {
        *self == SSI_CR0_FRF_A::SSI_CR0_FRF_TI
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_FRF_NMW`"]
    #[inline(always)]
    pub fn is_ssi_cr0_frf_nmw(&self) -> bool {
        *self == SSI_CR0_FRF_A::SSI_CR0_FRF_NMW
    }
}
#[doc = "Write proxy for field `SSI_CR0_FRF`"]
pub struct SSI_CR0_FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_CR0_FRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSI_CR0_FRF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Freescale SPI Frame Format"]
    #[inline(always)]
    pub fn ssi_cr0_frf_moto(self) -> &'a mut W {
        self.variant(SSI_CR0_FRF_A::SSI_CR0_FRF_MOTO)
    }
    #[doc = "Texas Instruments Synchronous Serial Frame Format"]
    #[inline(always)]
    pub fn ssi_cr0_frf_ti(self) -> &'a mut W {
        self.variant(SSI_CR0_FRF_A::SSI_CR0_FRF_TI)
    }
    #[doc = "MICROWIRE Frame Format"]
    #[inline(always)]
    pub fn ssi_cr0_frf_nmw(self) -> &'a mut W {
        self.variant(SSI_CR0_FRF_A::SSI_CR0_FRF_NMW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SSI_CR0_SPO`"]
pub type SSI_CR0_SPO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_CR0_SPO`"]
pub struct SSI_CR0_SPO_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_CR0_SPO_W<'a> {
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
#[doc = "Reader of field `SSI_CR0_SPH`"]
pub type SSI_CR0_SPH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_CR0_SPH`"]
pub struct SSI_CR0_SPH_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_CR0_SPH_W<'a> {
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
#[doc = "Reader of field `SSI_CR0_SCR`"]
pub type SSI_CR0_SCR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSI_CR0_SCR`"]
pub struct SSI_CR0_SCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_CR0_SCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - SSI Data Size Select"]
    #[inline(always)]
    pub fn ssi_cr0_dss(&self) -> SSI_CR0_DSS_R {
        SSI_CR0_DSS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - SSI Frame Format Select"]
    #[inline(always)]
    pub fn ssi_cr0_frf(&self) -> SSI_CR0_FRF_R {
        SSI_CR0_FRF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - SSI Serial Clock Polarity"]
    #[inline(always)]
    pub fn ssi_cr0_spo(&self) -> SSI_CR0_SPO_R {
        SSI_CR0_SPO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SSI Serial Clock Phase"]
    #[inline(always)]
    pub fn ssi_cr0_sph(&self) -> SSI_CR0_SPH_R {
        SSI_CR0_SPH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - SSI Serial Clock Rate"]
    #[inline(always)]
    pub fn ssi_cr0_scr(&self) -> SSI_CR0_SCR_R {
        SSI_CR0_SCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SSI Data Size Select"]
    #[inline(always)]
    pub fn ssi_cr0_dss(&mut self) -> SSI_CR0_DSS_W {
        SSI_CR0_DSS_W { w: self }
    }
    #[doc = "Bits 4:5 - SSI Frame Format Select"]
    #[inline(always)]
    pub fn ssi_cr0_frf(&mut self) -> SSI_CR0_FRF_W {
        SSI_CR0_FRF_W { w: self }
    }
    #[doc = "Bit 6 - SSI Serial Clock Polarity"]
    #[inline(always)]
    pub fn ssi_cr0_spo(&mut self) -> SSI_CR0_SPO_W {
        SSI_CR0_SPO_W { w: self }
    }
    #[doc = "Bit 7 - SSI Serial Clock Phase"]
    #[inline(always)]
    pub fn ssi_cr0_sph(&mut self) -> SSI_CR0_SPH_W {
        SSI_CR0_SPH_W { w: self }
    }
    #[doc = "Bits 8:15 - SSI Serial Clock Rate"]
    #[inline(always)]
    pub fn ssi_cr0_scr(&mut self) -> SSI_CR0_SCR_W {
        SSI_CR0_SCR_W { w: self }
    }
}
