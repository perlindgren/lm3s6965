#[doc = "Reader of register DID0"]
pub type R = crate::R<u32, super::DID0>;
#[doc = "Writer for register DID0"]
pub type W = crate::W<u32, super::DID0>;
#[doc = "Register DID0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DID0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Minor Revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID0_MIN_A {
    #[doc = "0: Initial device, or a major revision update"]
    SYSCTL_DID0_MIN_0 = 0,
    #[doc = "1: First metal layer change"]
    SYSCTL_DID0_MIN_1 = 1,
    #[doc = "2: Second metal layer change"]
    SYSCTL_DID0_MIN_2 = 2,
}
impl From<SYSCTL_DID0_MIN_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID0_MIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DID0_MIN`"]
pub type SYSCTL_DID0_MIN_R = crate::R<u8, SYSCTL_DID0_MIN_A>;
impl SYSCTL_DID0_MIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_DID0_MIN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_0),
            1 => Val(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_1),
            2 => Val(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MIN_0`"]
    #[inline(always)]
    pub fn is_sysctl_did0_min_0(&self) -> bool {
        *self == SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_0
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MIN_1`"]
    #[inline(always)]
    pub fn is_sysctl_did0_min_1(&self) -> bool {
        *self == SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_1
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MIN_2`"]
    #[inline(always)]
    pub fn is_sysctl_did0_min_2(&self) -> bool {
        *self == SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_2
    }
}
#[doc = "Write proxy for field `SYSCTL_DID0_MIN`"]
pub struct SYSCTL_DID0_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DID0_MIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID0_MIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Initial device, or a major revision update"]
    #[inline(always)]
    pub fn sysctl_did0_min_0(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_0)
    }
    #[doc = "First metal layer change"]
    #[inline(always)]
    pub fn sysctl_did0_min_1(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_1)
    }
    #[doc = "Second metal layer change"]
    #[inline(always)]
    pub fn sysctl_did0_min_2(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Major Revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID0_MAJ_A {
    #[doc = "0: Revision A (initial device)"]
    SYSCTL_DID0_MAJ_REVA = 0,
    #[doc = "1: Revision B (first base layer revision)"]
    SYSCTL_DID0_MAJ_REVB = 1,
    #[doc = "2: Revision C (second base layer revision)"]
    SYSCTL_DID0_MAJ_REVC = 2,
}
impl From<SYSCTL_DID0_MAJ_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID0_MAJ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DID0_MAJ`"]
pub type SYSCTL_DID0_MAJ_R = crate::R<u8, SYSCTL_DID0_MAJ_A>;
impl SYSCTL_DID0_MAJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_DID0_MAJ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVA),
            1 => Val(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVB),
            2 => Val(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MAJ_REVA`"]
    #[inline(always)]
    pub fn is_sysctl_did0_maj_reva(&self) -> bool {
        *self == SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVA
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MAJ_REVB`"]
    #[inline(always)]
    pub fn is_sysctl_did0_maj_revb(&self) -> bool {
        *self == SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVB
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MAJ_REVC`"]
    #[inline(always)]
    pub fn is_sysctl_did0_maj_revc(&self) -> bool {
        *self == SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVC
    }
}
#[doc = "Write proxy for field `SYSCTL_DID0_MAJ`"]
pub struct SYSCTL_DID0_MAJ_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DID0_MAJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID0_MAJ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Revision A (initial device)"]
    #[inline(always)]
    pub fn sysctl_did0_maj_reva(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVA)
    }
    #[doc = "Revision B (first base layer revision)"]
    #[inline(always)]
    pub fn sysctl_did0_maj_revb(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVB)
    }
    #[doc = "Revision C (second base layer revision)"]
    #[inline(always)]
    pub fn sysctl_did0_maj_revc(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Device Class\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID0_CLASS_A {
    #[doc = "1: Stellaris(R) Fury-class devices"]
    SYSCTL_DID0_CLASS_FURY = 1,
}
impl From<SYSCTL_DID0_CLASS_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID0_CLASS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DID0_CLASS`"]
pub type SYSCTL_DID0_CLASS_R = crate::R<u8, SYSCTL_DID0_CLASS_A>;
impl SYSCTL_DID0_CLASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_DID0_CLASS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SYSCTL_DID0_CLASS_A::SYSCTL_DID0_CLASS_FURY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_CLASS_FURY`"]
    #[inline(always)]
    pub fn is_sysctl_did0_class_fury(&self) -> bool {
        *self == SYSCTL_DID0_CLASS_A::SYSCTL_DID0_CLASS_FURY
    }
}
#[doc = "Write proxy for field `SYSCTL_DID0_CLASS`"]
pub struct SYSCTL_DID0_CLASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DID0_CLASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID0_CLASS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Stellaris(R) Fury-class devices"]
    #[inline(always)]
    pub fn sysctl_did0_class_fury(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_CLASS_A::SYSCTL_DID0_CLASS_FURY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "DID0 Version\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID0_VER_A {
    #[doc = "1: Second version of the DID0 register format"]
    SYSCTL_DID0_VER_1 = 1,
}
impl From<SYSCTL_DID0_VER_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID0_VER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DID0_VER`"]
pub type SYSCTL_DID0_VER_R = crate::R<u8, SYSCTL_DID0_VER_A>;
impl SYSCTL_DID0_VER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_DID0_VER_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SYSCTL_DID0_VER_A::SYSCTL_DID0_VER_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_VER_1`"]
    #[inline(always)]
    pub fn is_sysctl_did0_ver_1(&self) -> bool {
        *self == SYSCTL_DID0_VER_A::SYSCTL_DID0_VER_1
    }
}
#[doc = "Write proxy for field `SYSCTL_DID0_VER`"]
pub struct SYSCTL_DID0_VER_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DID0_VER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID0_VER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Second version of the DID0 register format"]
    #[inline(always)]
    pub fn sysctl_did0_ver_1(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_VER_A::SYSCTL_DID0_VER_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Minor Revision"]
    #[inline(always)]
    pub fn sysctl_did0_min(&self) -> SYSCTL_DID0_MIN_R {
        SYSCTL_DID0_MIN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Major Revision"]
    #[inline(always)]
    pub fn sysctl_did0_maj(&self) -> SYSCTL_DID0_MAJ_R {
        SYSCTL_DID0_MAJ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Device Class"]
    #[inline(always)]
    pub fn sysctl_did0_class(&self) -> SYSCTL_DID0_CLASS_R {
        SYSCTL_DID0_CLASS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - DID0 Version"]
    #[inline(always)]
    pub fn sysctl_did0_ver(&self) -> SYSCTL_DID0_VER_R {
        SYSCTL_DID0_VER_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minor Revision"]
    #[inline(always)]
    pub fn sysctl_did0_min(&mut self) -> SYSCTL_DID0_MIN_W {
        SYSCTL_DID0_MIN_W { w: self }
    }
    #[doc = "Bits 8:15 - Major Revision"]
    #[inline(always)]
    pub fn sysctl_did0_maj(&mut self) -> SYSCTL_DID0_MAJ_W {
        SYSCTL_DID0_MAJ_W { w: self }
    }
    #[doc = "Bits 16:23 - Device Class"]
    #[inline(always)]
    pub fn sysctl_did0_class(&mut self) -> SYSCTL_DID0_CLASS_W {
        SYSCTL_DID0_CLASS_W { w: self }
    }
    #[doc = "Bits 28:30 - DID0 Version"]
    #[inline(always)]
    pub fn sysctl_did0_ver(&mut self) -> SYSCTL_DID0_VER_W {
        SYSCTL_DID0_VER_W { w: self }
    }
}
