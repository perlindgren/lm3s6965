#[doc = "Reader of register DID1"]
pub type R = crate::R<u32, super::DID1>;
#[doc = "Writer for register DID1"]
pub type W = crate::W<u32, super::DID1>;
#[doc = "Register DID1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DID1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Qualification Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID1_QUAL_A {
    #[doc = "0: Engineering Sample (unqualified)"]
    SYSCTL_DID1_QUAL_ES = 0,
    #[doc = "1: Pilot Production (unqualified)"]
    SYSCTL_DID1_QUAL_PP = 1,
    #[doc = "2: Fully Qualified"]
    SYSCTL_DID1_QUAL_FQ = 2,
}
impl From<SYSCTL_DID1_QUAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_QUAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DID1_QUAL`"]
pub type SYSCTL_DID1_QUAL_R = crate::R<u8, SYSCTL_DID1_QUAL_A>;
impl SYSCTL_DID1_QUAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_DID1_QUAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_ES),
            1 => Val(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_PP),
            2 => Val(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_FQ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_QUAL_ES`"]
    #[inline(always)]
    pub fn is_sysctl_did1_qual_es(&self) -> bool {
        *self == SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_ES
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_QUAL_PP`"]
    #[inline(always)]
    pub fn is_sysctl_did1_qual_pp(&self) -> bool {
        *self == SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_PP
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_QUAL_FQ`"]
    #[inline(always)]
    pub fn is_sysctl_did1_qual_fq(&self) -> bool {
        *self == SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_FQ
    }
}
#[doc = "Write proxy for field `SYSCTL_DID1_QUAL`"]
pub struct SYSCTL_DID1_QUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DID1_QUAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID1_QUAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Engineering Sample (unqualified)"]
    #[inline(always)]
    pub fn sysctl_did1_qual_es(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_ES)
    }
    #[doc = "Pilot Production (unqualified)"]
    #[inline(always)]
    pub fn sysctl_did1_qual_pp(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_PP)
    }
    #[doc = "Fully Qualified"]
    #[inline(always)]
    pub fn sysctl_did1_qual_fq(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_FQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DID1_ROHS`"]
pub type SYSCTL_DID1_ROHS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DID1_ROHS`"]
pub struct SYSCTL_DID1_ROHS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DID1_ROHS_W<'a> {
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
#[doc = "Package Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID1_PKG_A {
    #[doc = "0: SOIC package"]
    SYSCTL_DID1_PKG_SOIC = 0,
    #[doc = "1: LQFP package"]
    SYSCTL_DID1_PKG_QFP = 1,
    #[doc = "2: BGA package"]
    SYSCTL_DID1_PKG_BGA = 2,
}
impl From<SYSCTL_DID1_PKG_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_PKG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DID1_PKG`"]
pub type SYSCTL_DID1_PKG_R = crate::R<u8, SYSCTL_DID1_PKG_A>;
impl SYSCTL_DID1_PKG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_DID1_PKG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_SOIC),
            1 => Val(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_QFP),
            2 => Val(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_BGA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_PKG_SOIC`"]
    #[inline(always)]
    pub fn is_sysctl_did1_pkg_soic(&self) -> bool {
        *self == SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_SOIC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_PKG_QFP`"]
    #[inline(always)]
    pub fn is_sysctl_did1_pkg_qfp(&self) -> bool {
        *self == SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_QFP
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_PKG_BGA`"]
    #[inline(always)]
    pub fn is_sysctl_did1_pkg_bga(&self) -> bool {
        *self == SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_BGA
    }
}
#[doc = "Write proxy for field `SYSCTL_DID1_PKG`"]
pub struct SYSCTL_DID1_PKG_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DID1_PKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID1_PKG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SOIC package"]
    #[inline(always)]
    pub fn sysctl_did1_pkg_soic(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_SOIC)
    }
    #[doc = "LQFP package"]
    #[inline(always)]
    pub fn sysctl_did1_pkg_qfp(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_QFP)
    }
    #[doc = "BGA package"]
    #[inline(always)]
    pub fn sysctl_did1_pkg_bga(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_BGA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Temperature Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID1_TEMP_A {
    #[doc = "0: Commercial temperature range (0C to 70C)"]
    SYSCTL_DID1_TEMP_C = 0,
    #[doc = "1: Industrial temperature range (-40C to 85C)"]
    SYSCTL_DID1_TEMP_I = 1,
    #[doc = "2: Extended temperature range (-40C to 105C)"]
    SYSCTL_DID1_TEMP_E = 2,
}
impl From<SYSCTL_DID1_TEMP_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_TEMP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DID1_TEMP`"]
pub type SYSCTL_DID1_TEMP_R = crate::R<u8, SYSCTL_DID1_TEMP_A>;
impl SYSCTL_DID1_TEMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_DID1_TEMP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_C),
            1 => Val(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_I),
            2 => Val(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_E),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_TEMP_C`"]
    #[inline(always)]
    pub fn is_sysctl_did1_temp_c(&self) -> bool {
        *self == SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_C
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_TEMP_I`"]
    #[inline(always)]
    pub fn is_sysctl_did1_temp_i(&self) -> bool {
        *self == SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_I
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_TEMP_E`"]
    #[inline(always)]
    pub fn is_sysctl_did1_temp_e(&self) -> bool {
        *self == SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_E
    }
}
#[doc = "Write proxy for field `SYSCTL_DID1_TEMP`"]
pub struct SYSCTL_DID1_TEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DID1_TEMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID1_TEMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Commercial temperature range (0C to 70C)"]
    #[inline(always)]
    pub fn sysctl_did1_temp_c(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_C)
    }
    #[doc = "Industrial temperature range (-40C to 85C)"]
    #[inline(always)]
    pub fn sysctl_did1_temp_i(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_I)
    }
    #[doc = "Extended temperature range (-40C to 105C)"]
    #[inline(always)]
    pub fn sysctl_did1_temp_e(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_E)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Package Pin Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID1_PINCNT_A {
    #[doc = "2: 100-pin package"]
    SYSCTL_DID1_PINCNT_100 = 2,
}
impl From<SYSCTL_DID1_PINCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_PINCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DID1_PINCNT`"]
pub type SYSCTL_DID1_PINCNT_R = crate::R<u8, SYSCTL_DID1_PINCNT_A>;
impl SYSCTL_DID1_PINCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_DID1_PINCNT_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_100),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_PINCNT_100`"]
    #[inline(always)]
    pub fn is_sysctl_did1_pincnt_100(&self) -> bool {
        *self == SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_100
    }
}
#[doc = "Write proxy for field `SYSCTL_DID1_PINCNT`"]
pub struct SYSCTL_DID1_PINCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DID1_PINCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID1_PINCNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "100-pin package"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt_100(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_100)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Part Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID1_PRTNO_A {
    #[doc = "115: LM3S6965"]
    SYSCTL_DID1_PRTNO_6965 = 115,
}
impl From<SYSCTL_DID1_PRTNO_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_PRTNO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DID1_PRTNO`"]
pub type SYSCTL_DID1_PRTNO_R = crate::R<u8, SYSCTL_DID1_PRTNO_A>;
impl SYSCTL_DID1_PRTNO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_DID1_PRTNO_A> {
        use crate::Variant::*;
        match self.bits {
            115 => Val(SYSCTL_DID1_PRTNO_A::SYSCTL_DID1_PRTNO_6965),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_PRTNO_6965`"]
    #[inline(always)]
    pub fn is_sysctl_did1_prtno_6965(&self) -> bool {
        *self == SYSCTL_DID1_PRTNO_A::SYSCTL_DID1_PRTNO_6965
    }
}
#[doc = "Write proxy for field `SYSCTL_DID1_PRTNO`"]
pub struct SYSCTL_DID1_PRTNO_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DID1_PRTNO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID1_PRTNO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LM3S6965"]
    #[inline(always)]
    pub fn sysctl_did1_prtno_6965(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_PRTNO_A::SYSCTL_DID1_PRTNO_6965)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Family\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID1_FAM_A {
    #[doc = "0: Stellaris family of microcontollers, that is, all devices with external part numbers starting with LM3S"]
    SYSCTL_DID1_FAM_STELLARIS = 0,
}
impl From<SYSCTL_DID1_FAM_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_FAM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DID1_FAM`"]
pub type SYSCTL_DID1_FAM_R = crate::R<u8, SYSCTL_DID1_FAM_A>;
impl SYSCTL_DID1_FAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_DID1_FAM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSCTL_DID1_FAM_A::SYSCTL_DID1_FAM_STELLARIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_FAM_STELLARIS`"]
    #[inline(always)]
    pub fn is_sysctl_did1_fam_stellaris(&self) -> bool {
        *self == SYSCTL_DID1_FAM_A::SYSCTL_DID1_FAM_STELLARIS
    }
}
#[doc = "Write proxy for field `SYSCTL_DID1_FAM`"]
pub struct SYSCTL_DID1_FAM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DID1_FAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID1_FAM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Stellaris family of microcontollers, that is, all devices with external part numbers starting with LM3S"]
    #[inline(always)]
    pub fn sysctl_did1_fam_stellaris(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_FAM_A::SYSCTL_DID1_FAM_STELLARIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "DID1 Version\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID1_VER_A {
    #[doc = "1: Second version of the DID1 register format"]
    SYSCTL_DID1_VER_1 = 1,
}
impl From<SYSCTL_DID1_VER_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_VER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DID1_VER`"]
pub type SYSCTL_DID1_VER_R = crate::R<u8, SYSCTL_DID1_VER_A>;
impl SYSCTL_DID1_VER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_DID1_VER_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SYSCTL_DID1_VER_A::SYSCTL_DID1_VER_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_VER_1`"]
    #[inline(always)]
    pub fn is_sysctl_did1_ver_1(&self) -> bool {
        *self == SYSCTL_DID1_VER_A::SYSCTL_DID1_VER_1
    }
}
#[doc = "Write proxy for field `SYSCTL_DID1_VER`"]
pub struct SYSCTL_DID1_VER_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DID1_VER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID1_VER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Second version of the DID1 register format"]
    #[inline(always)]
    pub fn sysctl_did1_ver_1(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_VER_A::SYSCTL_DID1_VER_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Qualification Status"]
    #[inline(always)]
    pub fn sysctl_did1_qual(&self) -> SYSCTL_DID1_QUAL_R {
        SYSCTL_DID1_QUAL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - RoHS-Compliance"]
    #[inline(always)]
    pub fn sysctl_did1_rohs(&self) -> SYSCTL_DID1_ROHS_R {
        SYSCTL_DID1_ROHS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Package Type"]
    #[inline(always)]
    pub fn sysctl_did1_pkg(&self) -> SYSCTL_DID1_PKG_R {
        SYSCTL_DID1_PKG_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - Temperature Range"]
    #[inline(always)]
    pub fn sysctl_did1_temp(&self) -> SYSCTL_DID1_TEMP_R {
        SYSCTL_DID1_TEMP_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 13:15 - Package Pin Count"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt(&self) -> SYSCTL_DID1_PINCNT_R {
        SYSCTL_DID1_PINCNT_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - Part Number"]
    #[inline(always)]
    pub fn sysctl_did1_prtno(&self) -> SYSCTL_DID1_PRTNO_R {
        SYSCTL_DID1_PRTNO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Family"]
    #[inline(always)]
    pub fn sysctl_did1_fam(&self) -> SYSCTL_DID1_FAM_R {
        SYSCTL_DID1_FAM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DID1 Version"]
    #[inline(always)]
    pub fn sysctl_did1_ver(&self) -> SYSCTL_DID1_VER_R {
        SYSCTL_DID1_VER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Qualification Status"]
    #[inline(always)]
    pub fn sysctl_did1_qual(&mut self) -> SYSCTL_DID1_QUAL_W {
        SYSCTL_DID1_QUAL_W { w: self }
    }
    #[doc = "Bit 2 - RoHS-Compliance"]
    #[inline(always)]
    pub fn sysctl_did1_rohs(&mut self) -> SYSCTL_DID1_ROHS_W {
        SYSCTL_DID1_ROHS_W { w: self }
    }
    #[doc = "Bits 3:4 - Package Type"]
    #[inline(always)]
    pub fn sysctl_did1_pkg(&mut self) -> SYSCTL_DID1_PKG_W {
        SYSCTL_DID1_PKG_W { w: self }
    }
    #[doc = "Bits 5:7 - Temperature Range"]
    #[inline(always)]
    pub fn sysctl_did1_temp(&mut self) -> SYSCTL_DID1_TEMP_W {
        SYSCTL_DID1_TEMP_W { w: self }
    }
    #[doc = "Bits 13:15 - Package Pin Count"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt(&mut self) -> SYSCTL_DID1_PINCNT_W {
        SYSCTL_DID1_PINCNT_W { w: self }
    }
    #[doc = "Bits 16:23 - Part Number"]
    #[inline(always)]
    pub fn sysctl_did1_prtno(&mut self) -> SYSCTL_DID1_PRTNO_W {
        SYSCTL_DID1_PRTNO_W { w: self }
    }
    #[doc = "Bits 24:27 - Family"]
    #[inline(always)]
    pub fn sysctl_did1_fam(&mut self) -> SYSCTL_DID1_FAM_W {
        SYSCTL_DID1_FAM_W { w: self }
    }
    #[doc = "Bits 28:31 - DID1 Version"]
    #[inline(always)]
    pub fn sysctl_did1_ver(&mut self) -> SYSCTL_DID1_VER_W {
        SYSCTL_DID1_VER_W { w: self }
    }
}
