#[doc = "Reader of register SCGC0"]
pub type R = crate::R<u32, super::SCGC0>;
#[doc = "Writer for register SCGC0"]
pub type W = crate::W<u32, super::SCGC0>;
#[doc = "Register SCGC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCGC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCTL_SCGC0_HIB`"]
pub type SYSCTL_SCGC0_HIB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_SCGC0_HIB`"]
pub struct SYSCTL_SCGC0_HIB_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_SCGC0_HIB_W<'a> {
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
#[doc = "ADC Sample Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_SCGC0_ADCSPD_A {
    #[doc = "0: 125K samples/second"]
    SYSCTL_SCGC0_ADCSPD125K = 0,
    #[doc = "1: 250K samples/second"]
    SYSCTL_SCGC0_ADCSPD250K = 1,
    #[doc = "2: 500K samples/second"]
    SYSCTL_SCGC0_ADCSPD500K = 2,
    #[doc = "3: 1M samples/second"]
    SYSCTL_SCGC0_ADCSPD1M = 3,
}
impl From<SYSCTL_SCGC0_ADCSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_SCGC0_ADCSPD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_SCGC0_ADCSPD`"]
pub type SYSCTL_SCGC0_ADCSPD_R = crate::R<u8, SYSCTL_SCGC0_ADCSPD_A>;
impl SYSCTL_SCGC0_ADCSPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTL_SCGC0_ADCSPD_A {
        match self.bits {
            0 => SYSCTL_SCGC0_ADCSPD_A::SYSCTL_SCGC0_ADCSPD125K,
            1 => SYSCTL_SCGC0_ADCSPD_A::SYSCTL_SCGC0_ADCSPD250K,
            2 => SYSCTL_SCGC0_ADCSPD_A::SYSCTL_SCGC0_ADCSPD500K,
            3 => SYSCTL_SCGC0_ADCSPD_A::SYSCTL_SCGC0_ADCSPD1M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SCGC0_ADCSPD125K`"]
    #[inline(always)]
    pub fn is_sysctl_scgc0_adcspd125k(&self) -> bool {
        *self == SYSCTL_SCGC0_ADCSPD_A::SYSCTL_SCGC0_ADCSPD125K
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SCGC0_ADCSPD250K`"]
    #[inline(always)]
    pub fn is_sysctl_scgc0_adcspd250k(&self) -> bool {
        *self == SYSCTL_SCGC0_ADCSPD_A::SYSCTL_SCGC0_ADCSPD250K
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SCGC0_ADCSPD500K`"]
    #[inline(always)]
    pub fn is_sysctl_scgc0_adcspd500k(&self) -> bool {
        *self == SYSCTL_SCGC0_ADCSPD_A::SYSCTL_SCGC0_ADCSPD500K
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SCGC0_ADCSPD1M`"]
    #[inline(always)]
    pub fn is_sysctl_scgc0_adcspd1m(&self) -> bool {
        *self == SYSCTL_SCGC0_ADCSPD_A::SYSCTL_SCGC0_ADCSPD1M
    }
}
#[doc = "Write proxy for field `SYSCTL_SCGC0_ADCSPD`"]
pub struct SYSCTL_SCGC0_ADCSPD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_SCGC0_ADCSPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_SCGC0_ADCSPD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "125K samples/second"]
    #[inline(always)]
    pub fn sysctl_scgc0_adcspd125k(self) -> &'a mut W {
        self.variant(SYSCTL_SCGC0_ADCSPD_A::SYSCTL_SCGC0_ADCSPD125K)
    }
    #[doc = "250K samples/second"]
    #[inline(always)]
    pub fn sysctl_scgc0_adcspd250k(self) -> &'a mut W {
        self.variant(SYSCTL_SCGC0_ADCSPD_A::SYSCTL_SCGC0_ADCSPD250K)
    }
    #[doc = "500K samples/second"]
    #[inline(always)]
    pub fn sysctl_scgc0_adcspd500k(self) -> &'a mut W {
        self.variant(SYSCTL_SCGC0_ADCSPD_A::SYSCTL_SCGC0_ADCSPD500K)
    }
    #[doc = "1M samples/second"]
    #[inline(always)]
    pub fn sysctl_scgc0_adcspd1m(self) -> &'a mut W {
        self.variant(SYSCTL_SCGC0_ADCSPD_A::SYSCTL_SCGC0_ADCSPD1M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - HIB Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgc0_hib(&self) -> SYSCTL_SCGC0_HIB_R {
        SYSCTL_SCGC0_HIB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - ADC Sample Speed"]
    #[inline(always)]
    pub fn sysctl_scgc0_adcspd(&self) -> SYSCTL_SCGC0_ADCSPD_R {
        SYSCTL_SCGC0_ADCSPD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - HIB Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgc0_hib(&mut self) -> SYSCTL_SCGC0_HIB_W {
        SYSCTL_SCGC0_HIB_W { w: self }
    }
    #[doc = "Bits 8:9 - ADC Sample Speed"]
    #[inline(always)]
    pub fn sysctl_scgc0_adcspd(&mut self) -> SYSCTL_SCGC0_ADCSPD_W {
        SYSCTL_SCGC0_ADCSPD_W { w: self }
    }
}
