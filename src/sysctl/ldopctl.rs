#[doc = "Reader of register LDOPCTL"]
pub type R = crate::R<u32, super::LDOPCTL>;
#[doc = "Writer for register LDOPCTL"]
pub type W = crate::W<u32, super::LDOPCTL>;
#[doc = "Register LDOPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LDOPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LDO Output Voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_LDOPCTL_A {
    #[doc = "0: 2.50"]
    SYSCTL_LDOPCTL_2_50V = 0,
    #[doc = "1: 2.45"]
    SYSCTL_LDOPCTL_2_45V = 1,
    #[doc = "2: 2.40"]
    SYSCTL_LDOPCTL_2_40V = 2,
    #[doc = "3: 2.35"]
    SYSCTL_LDOPCTL_2_35V = 3,
    #[doc = "4: 2.30"]
    SYSCTL_LDOPCTL_2_30V = 4,
    #[doc = "5: 2.25"]
    SYSCTL_LDOPCTL_2_25V = 5,
    #[doc = "27: 2.75"]
    SYSCTL_LDOPCTL_2_75V = 27,
    #[doc = "28: 2.70"]
    SYSCTL_LDOPCTL_2_70V = 28,
    #[doc = "29: 2.65"]
    SYSCTL_LDOPCTL_2_65V = 29,
    #[doc = "30: 2.60"]
    SYSCTL_LDOPCTL_2_60V = 30,
    #[doc = "31: 2.55"]
    SYSCTL_LDOPCTL_2_55V = 31,
}
impl From<SYSCTL_LDOPCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_LDOPCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_LDOPCTL`"]
pub type SYSCTL_LDOPCTL_R = crate::R<u8, SYSCTL_LDOPCTL_A>;
impl SYSCTL_LDOPCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_LDOPCTL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_50V),
            1 => Val(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_45V),
            2 => Val(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_40V),
            3 => Val(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_35V),
            4 => Val(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_30V),
            5 => Val(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_25V),
            27 => Val(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_75V),
            28 => Val(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_70V),
            29 => Val(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_65V),
            30 => Val(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_60V),
            31 => Val(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_55V),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDOPCTL_2_50V`"]
    #[inline(always)]
    pub fn is_sysctl_ldopctl_2_50v(&self) -> bool {
        *self == SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_50V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDOPCTL_2_45V`"]
    #[inline(always)]
    pub fn is_sysctl_ldopctl_2_45v(&self) -> bool {
        *self == SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_45V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDOPCTL_2_40V`"]
    #[inline(always)]
    pub fn is_sysctl_ldopctl_2_40v(&self) -> bool {
        *self == SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_40V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDOPCTL_2_35V`"]
    #[inline(always)]
    pub fn is_sysctl_ldopctl_2_35v(&self) -> bool {
        *self == SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_35V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDOPCTL_2_30V`"]
    #[inline(always)]
    pub fn is_sysctl_ldopctl_2_30v(&self) -> bool {
        *self == SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_30V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDOPCTL_2_25V`"]
    #[inline(always)]
    pub fn is_sysctl_ldopctl_2_25v(&self) -> bool {
        *self == SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_25V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDOPCTL_2_75V`"]
    #[inline(always)]
    pub fn is_sysctl_ldopctl_2_75v(&self) -> bool {
        *self == SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_75V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDOPCTL_2_70V`"]
    #[inline(always)]
    pub fn is_sysctl_ldopctl_2_70v(&self) -> bool {
        *self == SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_70V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDOPCTL_2_65V`"]
    #[inline(always)]
    pub fn is_sysctl_ldopctl_2_65v(&self) -> bool {
        *self == SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_65V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDOPCTL_2_60V`"]
    #[inline(always)]
    pub fn is_sysctl_ldopctl_2_60v(&self) -> bool {
        *self == SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_60V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDOPCTL_2_55V`"]
    #[inline(always)]
    pub fn is_sysctl_ldopctl_2_55v(&self) -> bool {
        *self == SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_55V
    }
}
#[doc = "Write proxy for field `SYSCTL_LDOPCTL`"]
pub struct SYSCTL_LDOPCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_LDOPCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_LDOPCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "2.50"]
    #[inline(always)]
    pub fn sysctl_ldopctl_2_50v(self) -> &'a mut W {
        self.variant(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_50V)
    }
    #[doc = "2.45"]
    #[inline(always)]
    pub fn sysctl_ldopctl_2_45v(self) -> &'a mut W {
        self.variant(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_45V)
    }
    #[doc = "2.40"]
    #[inline(always)]
    pub fn sysctl_ldopctl_2_40v(self) -> &'a mut W {
        self.variant(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_40V)
    }
    #[doc = "2.35"]
    #[inline(always)]
    pub fn sysctl_ldopctl_2_35v(self) -> &'a mut W {
        self.variant(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_35V)
    }
    #[doc = "2.30"]
    #[inline(always)]
    pub fn sysctl_ldopctl_2_30v(self) -> &'a mut W {
        self.variant(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_30V)
    }
    #[doc = "2.25"]
    #[inline(always)]
    pub fn sysctl_ldopctl_2_25v(self) -> &'a mut W {
        self.variant(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_25V)
    }
    #[doc = "2.75"]
    #[inline(always)]
    pub fn sysctl_ldopctl_2_75v(self) -> &'a mut W {
        self.variant(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_75V)
    }
    #[doc = "2.70"]
    #[inline(always)]
    pub fn sysctl_ldopctl_2_70v(self) -> &'a mut W {
        self.variant(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_70V)
    }
    #[doc = "2.65"]
    #[inline(always)]
    pub fn sysctl_ldopctl_2_65v(self) -> &'a mut W {
        self.variant(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_65V)
    }
    #[doc = "2.60"]
    #[inline(always)]
    pub fn sysctl_ldopctl_2_60v(self) -> &'a mut W {
        self.variant(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_60V)
    }
    #[doc = "2.55"]
    #[inline(always)]
    pub fn sysctl_ldopctl_2_55v(self) -> &'a mut W {
        self.variant(SYSCTL_LDOPCTL_A::SYSCTL_LDOPCTL_2_55V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - LDO Output Voltage"]
    #[inline(always)]
    pub fn sysctl_ldopctl(&self) -> SYSCTL_LDOPCTL_R {
        SYSCTL_LDOPCTL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - LDO Output Voltage"]
    #[inline(always)]
    pub fn sysctl_ldopctl(&mut self) -> SYSCTL_LDOPCTL_W {
        SYSCTL_LDOPCTL_W { w: self }
    }
}
