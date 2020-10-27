#[doc = "Reader of register ACCTL1"]
pub type R = crate::R<u32, super::ACCTL1>;
#[doc = "Writer for register ACCTL1"]
pub type W = crate::W<u32, super::ACCTL1>;
#[doc = "Register ACCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ACCTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP_ACCTL1_CINV`"]
pub type COMP_ACCTL1_CINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP_ACCTL1_CINV`"]
pub struct COMP_ACCTL1_CINV_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACCTL1_CINV_W<'a> {
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
#[doc = "Interrupt Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_ACCTL1_ISEN_A {
    #[doc = "0: Level sense, see ISLVAL"]
    COMP_ACCTL1_ISEN_LEVEL = 0,
    #[doc = "1: Falling edge"]
    COMP_ACCTL1_ISEN_FALL = 1,
    #[doc = "2: Rising edge"]
    COMP_ACCTL1_ISEN_RISE = 2,
    #[doc = "3: Either edge"]
    COMP_ACCTL1_ISEN_BOTH = 3,
}
impl From<COMP_ACCTL1_ISEN_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_ACCTL1_ISEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMP_ACCTL1_ISEN`"]
pub type COMP_ACCTL1_ISEN_R = crate::R<u8, COMP_ACCTL1_ISEN_A>;
impl COMP_ACCTL1_ISEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_ACCTL1_ISEN_A {
        match self.bits {
            0 => COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_LEVEL,
            1 => COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_FALL,
            2 => COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_RISE,
            3 => COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ISEN_LEVEL`"]
    #[inline(always)]
    pub fn is_comp_acctl1_isen_level(&self) -> bool {
        *self == COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_LEVEL
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ISEN_FALL`"]
    #[inline(always)]
    pub fn is_comp_acctl1_isen_fall(&self) -> bool {
        *self == COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_FALL
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ISEN_RISE`"]
    #[inline(always)]
    pub fn is_comp_acctl1_isen_rise(&self) -> bool {
        *self == COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_RISE
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ISEN_BOTH`"]
    #[inline(always)]
    pub fn is_comp_acctl1_isen_both(&self) -> bool {
        *self == COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_BOTH
    }
}
#[doc = "Write proxy for field `COMP_ACCTL1_ISEN`"]
pub struct COMP_ACCTL1_ISEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACCTL1_ISEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_ACCTL1_ISEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Level sense, see ISLVAL"]
    #[inline(always)]
    pub fn comp_acctl1_isen_level(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_LEVEL)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn comp_acctl1_isen_fall(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn comp_acctl1_isen_rise(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_RISE)
    }
    #[doc = "Either edge"]
    #[inline(always)]
    pub fn comp_acctl1_isen_both(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `COMP_ACCTL1_ISLVAL`"]
pub type COMP_ACCTL1_ISLVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP_ACCTL1_ISLVAL`"]
pub struct COMP_ACCTL1_ISLVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACCTL1_ISLVAL_W<'a> {
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
#[doc = "Trigger Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_ACCTL1_TSEN_A {
    #[doc = "0: Level sense, see TSLVAL"]
    COMP_ACCTL1_TSEN_LEVEL = 0,
    #[doc = "1: Falling edge"]
    COMP_ACCTL1_TSEN_FALL = 1,
    #[doc = "2: Rising edge"]
    COMP_ACCTL1_TSEN_RISE = 2,
    #[doc = "3: Either edge"]
    COMP_ACCTL1_TSEN_BOTH = 3,
}
impl From<COMP_ACCTL1_TSEN_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_ACCTL1_TSEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMP_ACCTL1_TSEN`"]
pub type COMP_ACCTL1_TSEN_R = crate::R<u8, COMP_ACCTL1_TSEN_A>;
impl COMP_ACCTL1_TSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_ACCTL1_TSEN_A {
        match self.bits {
            0 => COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_LEVEL,
            1 => COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_FALL,
            2 => COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_RISE,
            3 => COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_TSEN_LEVEL`"]
    #[inline(always)]
    pub fn is_comp_acctl1_tsen_level(&self) -> bool {
        *self == COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_LEVEL
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_TSEN_FALL`"]
    #[inline(always)]
    pub fn is_comp_acctl1_tsen_fall(&self) -> bool {
        *self == COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_FALL
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_TSEN_RISE`"]
    #[inline(always)]
    pub fn is_comp_acctl1_tsen_rise(&self) -> bool {
        *self == COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_RISE
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_TSEN_BOTH`"]
    #[inline(always)]
    pub fn is_comp_acctl1_tsen_both(&self) -> bool {
        *self == COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_BOTH
    }
}
#[doc = "Write proxy for field `COMP_ACCTL1_TSEN`"]
pub struct COMP_ACCTL1_TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACCTL1_TSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_ACCTL1_TSEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Level sense, see TSLVAL"]
    #[inline(always)]
    pub fn comp_acctl1_tsen_level(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_LEVEL)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn comp_acctl1_tsen_fall(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn comp_acctl1_tsen_rise(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_RISE)
    }
    #[doc = "Either edge"]
    #[inline(always)]
    pub fn comp_acctl1_tsen_both(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `COMP_ACCTL1_TSLVAL`"]
pub type COMP_ACCTL1_TSLVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP_ACCTL1_TSLVAL`"]
pub struct COMP_ACCTL1_TSLVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACCTL1_TSLVAL_W<'a> {
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
#[doc = "Analog Source Positive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_ACCTL1_ASRCP_A {
    #[doc = "0: Pin value of Cn+"]
    COMP_ACCTL1_ASRCP_PIN = 0,
    #[doc = "1: Pin value of C0+"]
    COMP_ACCTL1_ASRCP_PIN0 = 1,
    #[doc = "2: Internal voltage reference (VIREF)"]
    COMP_ACCTL1_ASRCP_REF = 2,
}
impl From<COMP_ACCTL1_ASRCP_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_ACCTL1_ASRCP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMP_ACCTL1_ASRCP`"]
pub type COMP_ACCTL1_ASRCP_R = crate::R<u8, COMP_ACCTL1_ASRCP_A>;
impl COMP_ACCTL1_ASRCP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMP_ACCTL1_ASRCP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN),
            1 => Val(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN0),
            2 => Val(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_REF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ASRCP_PIN`"]
    #[inline(always)]
    pub fn is_comp_acctl1_asrcp_pin(&self) -> bool {
        *self == COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ASRCP_PIN0`"]
    #[inline(always)]
    pub fn is_comp_acctl1_asrcp_pin0(&self) -> bool {
        *self == COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN0
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ASRCP_REF`"]
    #[inline(always)]
    pub fn is_comp_acctl1_asrcp_ref(&self) -> bool {
        *self == COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_REF
    }
}
#[doc = "Write proxy for field `COMP_ACCTL1_ASRCP`"]
pub struct COMP_ACCTL1_ASRCP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACCTL1_ASRCP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_ACCTL1_ASRCP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pin value of Cn+"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp_pin(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN)
    }
    #[doc = "Pin value of C0+"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp_pin0(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN0)
    }
    #[doc = "Internal voltage reference (VIREF)"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp_ref(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_REF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `COMP_ACCTL1_TOEN`"]
pub type COMP_ACCTL1_TOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP_ACCTL1_TOEN`"]
pub struct COMP_ACCTL1_TOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ACCTL1_TOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Comparator Output Invert"]
    #[inline(always)]
    pub fn comp_acctl1_cinv(&self) -> COMP_ACCTL1_CINV_R {
        COMP_ACCTL1_CINV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Interrupt Sense"]
    #[inline(always)]
    pub fn comp_acctl1_isen(&self) -> COMP_ACCTL1_ISEN_R {
        COMP_ACCTL1_ISEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Interrupt Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl1_islval(&self) -> COMP_ACCTL1_ISLVAL_R {
        COMP_ACCTL1_ISLVAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Trigger Sense"]
    #[inline(always)]
    pub fn comp_acctl1_tsen(&self) -> COMP_ACCTL1_TSEN_R {
        COMP_ACCTL1_TSEN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Trigger Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl1_tslval(&self) -> COMP_ACCTL1_TSLVAL_R {
        COMP_ACCTL1_TSLVAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Analog Source Positive"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp(&self) -> COMP_ACCTL1_ASRCP_R {
        COMP_ACCTL1_ASRCP_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Trigger Output Enable"]
    #[inline(always)]
    pub fn comp_acctl1_toen(&self) -> COMP_ACCTL1_TOEN_R {
        COMP_ACCTL1_TOEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator Output Invert"]
    #[inline(always)]
    pub fn comp_acctl1_cinv(&mut self) -> COMP_ACCTL1_CINV_W {
        COMP_ACCTL1_CINV_W { w: self }
    }
    #[doc = "Bits 2:3 - Interrupt Sense"]
    #[inline(always)]
    pub fn comp_acctl1_isen(&mut self) -> COMP_ACCTL1_ISEN_W {
        COMP_ACCTL1_ISEN_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl1_islval(&mut self) -> COMP_ACCTL1_ISLVAL_W {
        COMP_ACCTL1_ISLVAL_W { w: self }
    }
    #[doc = "Bits 5:6 - Trigger Sense"]
    #[inline(always)]
    pub fn comp_acctl1_tsen(&mut self) -> COMP_ACCTL1_TSEN_W {
        COMP_ACCTL1_TSEN_W { w: self }
    }
    #[doc = "Bit 7 - Trigger Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl1_tslval(&mut self) -> COMP_ACCTL1_TSLVAL_W {
        COMP_ACCTL1_TSLVAL_W { w: self }
    }
    #[doc = "Bits 9:10 - Analog Source Positive"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp(&mut self) -> COMP_ACCTL1_ASRCP_W {
        COMP_ACCTL1_ASRCP_W { w: self }
    }
    #[doc = "Bit 11 - Trigger Output Enable"]
    #[inline(always)]
    pub fn comp_acctl1_toen(&mut self) -> COMP_ACCTL1_TOEN_W {
        COMP_ACCTL1_TOEN_W { w: self }
    }
}
