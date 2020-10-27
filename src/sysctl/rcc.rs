#[doc = "Reader of register RCC"]
pub type R = crate::R<u32, super::RCC>;
#[doc = "Writer for register RCC"]
pub type W = crate::W<u32, super::RCC>;
#[doc = "Register RCC `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCTL_RCC_MOSCDIS`"]
pub type SYSCTL_RCC_MOSCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RCC_MOSCDIS`"]
pub struct SYSCTL_RCC_MOSCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC_MOSCDIS_W<'a> {
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
#[doc = "Reader of field `SYSCTL_RCC_IOSCDIS`"]
pub type SYSCTL_RCC_IOSCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RCC_IOSCDIS`"]
pub struct SYSCTL_RCC_IOSCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC_IOSCDIS_W<'a> {
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
#[doc = "Oscillator Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_RCC_OSCSRC_A {
    #[doc = "0: MOSC"]
    SYSCTL_RCC_OSCSRC_MAIN = 0,
    #[doc = "1: IOSC"]
    SYSCTL_RCC_OSCSRC_INT = 1,
    #[doc = "2: IOSC/4"]
    SYSCTL_RCC_OSCSRC_INT4 = 2,
    #[doc = "3: 30 kHz"]
    SYSCTL_RCC_OSCSRC_30 = 3,
}
impl From<SYSCTL_RCC_OSCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RCC_OSCSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_RCC_OSCSRC`"]
pub type SYSCTL_RCC_OSCSRC_R = crate::R<u8, SYSCTL_RCC_OSCSRC_A>;
impl SYSCTL_RCC_OSCSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTL_RCC_OSCSRC_A {
        match self.bits {
            0 => SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_MAIN,
            1 => SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_INT,
            2 => SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_INT4,
            3 => SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_30,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_OSCSRC_MAIN`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_oscsrc_main(&self) -> bool {
        *self == SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_MAIN
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_OSCSRC_INT`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_oscsrc_int(&self) -> bool {
        *self == SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_INT
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_OSCSRC_INT4`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_oscsrc_int4(&self) -> bool {
        *self == SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_INT4
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_OSCSRC_30`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_oscsrc_30(&self) -> bool {
        *self == SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_30
    }
}
#[doc = "Write proxy for field `SYSCTL_RCC_OSCSRC`"]
pub struct SYSCTL_RCC_OSCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC_OSCSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RCC_OSCSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn sysctl_rcc_oscsrc_main(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_MAIN)
    }
    #[doc = "IOSC"]
    #[inline(always)]
    pub fn sysctl_rcc_oscsrc_int(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_INT)
    }
    #[doc = "IOSC/4"]
    #[inline(always)]
    pub fn sysctl_rcc_oscsrc_int4(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_INT4)
    }
    #[doc = "30 kHz"]
    #[inline(always)]
    pub fn sysctl_rcc_oscsrc_30(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_30)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Crystal Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_RCC_XTAL_A {
    #[doc = "0: 1 MHz"]
    SYSCTL_RCC_XTAL_1MHZ = 0,
    #[doc = "1: 1.8432 MHz"]
    SYSCTL_RCC_XTAL_1_84MHZ = 1,
    #[doc = "2: 2 MHz"]
    SYSCTL_RCC_XTAL_2MHZ = 2,
    #[doc = "3: 2.4576 MHz"]
    SYSCTL_RCC_XTAL_2_45MHZ = 3,
    #[doc = "4: 3.579545 MHz"]
    SYSCTL_RCC_XTAL_3_57MHZ = 4,
    #[doc = "5: 3.6864 MHz"]
    SYSCTL_RCC_XTAL_3_68MHZ = 5,
    #[doc = "6: 4 MHz"]
    SYSCTL_RCC_XTAL_4MHZ = 6,
    #[doc = "7: 4.096 MHz"]
    SYSCTL_RCC_XTAL_4_09MHZ = 7,
    #[doc = "8: 4.9152 MHz"]
    SYSCTL_RCC_XTAL_4_91MHZ = 8,
    #[doc = "9: 5 MHz"]
    SYSCTL_RCC_XTAL_5MHZ = 9,
    #[doc = "10: 5.12 MHz"]
    SYSCTL_RCC_XTAL_5_12MHZ = 10,
    #[doc = "11: 6 MHz"]
    SYSCTL_RCC_XTAL_6MHZ = 11,
    #[doc = "12: 6.144 MHz"]
    SYSCTL_RCC_XTAL_6_14MHZ = 12,
    #[doc = "13: 7.3728 MHz"]
    SYSCTL_RCC_XTAL_7_37MHZ = 13,
    #[doc = "14: 8 MHz"]
    SYSCTL_RCC_XTAL_8MHZ = 14,
    #[doc = "15: 8.192 MHz"]
    SYSCTL_RCC_XTAL_8_19MHZ = 15,
}
impl From<SYSCTL_RCC_XTAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RCC_XTAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_RCC_XTAL`"]
pub type SYSCTL_RCC_XTAL_R = crate::R<u8, SYSCTL_RCC_XTAL_A>;
impl SYSCTL_RCC_XTAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTL_RCC_XTAL_A {
        match self.bits {
            0 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_1MHZ,
            1 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_1_84MHZ,
            2 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_2MHZ,
            3 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_2_45MHZ,
            4 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_3_57MHZ,
            5 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_3_68MHZ,
            6 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4MHZ,
            7 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4_09MHZ,
            8 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4_91MHZ,
            9 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_5MHZ,
            10 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_5_12MHZ,
            11 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_6MHZ,
            12 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_6_14MHZ,
            13 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_7_37MHZ,
            14 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_8MHZ,
            15 => SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_8_19MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_1MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_1mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_1MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_1_84MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_1_84mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_1_84MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_2MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_2mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_2MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_2_45MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_2_45mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_2_45MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_3_57MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_3_57mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_3_57MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_3_68MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_3_68mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_3_68MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_4MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_4mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_4_09MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_4_09mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4_09MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_4_91MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_4_91mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4_91MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_5MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_5mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_5MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_5_12MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_5_12mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_5_12MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_6MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_6mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_6MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_6_14MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_6_14mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_6_14MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_7_37MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_7_37mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_7_37MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_8MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_8mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_8MHZ
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_XTAL_8_19MHZ`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_8_19mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_8_19MHZ
    }
}
#[doc = "Write proxy for field `SYSCTL_RCC_XTAL`"]
pub struct SYSCTL_RCC_XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC_XTAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RCC_XTAL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_1mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_1MHZ)
    }
    #[doc = "1.8432 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_1_84mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_1_84MHZ)
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_2mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_2MHZ)
    }
    #[doc = "2.4576 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_2_45mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_2_45MHZ)
    }
    #[doc = "3.579545 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_3_57mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_3_57MHZ)
    }
    #[doc = "3.6864 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_3_68mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_3_68MHZ)
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_4mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4MHZ)
    }
    #[doc = "4.096 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_4_09mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4_09MHZ)
    }
    #[doc = "4.9152 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_4_91mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4_91MHZ)
    }
    #[doc = "5 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_5mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_5MHZ)
    }
    #[doc = "5.12 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_5_12mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_5_12MHZ)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_6mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_6MHZ)
    }
    #[doc = "6.144 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_6_14mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_6_14MHZ)
    }
    #[doc = "7.3728 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_7_37mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_7_37MHZ)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_8mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_8MHZ)
    }
    #[doc = "8.192 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_8_19mhz(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_8_19MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_RCC_BYPASS`"]
pub type SYSCTL_RCC_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RCC_BYPASS`"]
pub struct SYSCTL_RCC_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC_BYPASS_W<'a> {
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
#[doc = "Reader of field `SYSCTL_RCC_PWRDN`"]
pub type SYSCTL_RCC_PWRDN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RCC_PWRDN`"]
pub struct SYSCTL_RCC_PWRDN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC_PWRDN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "PWM Unit Clock Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_RCC_PWMDIV_A {
    #[doc = "0: PWM clock /2"]
    SYSCTL_RCC_PWMDIV_2 = 0,
    #[doc = "1: PWM clock /4"]
    SYSCTL_RCC_PWMDIV_4 = 1,
    #[doc = "2: PWM clock /8"]
    SYSCTL_RCC_PWMDIV_8 = 2,
    #[doc = "3: PWM clock /16"]
    SYSCTL_RCC_PWMDIV_16 = 3,
    #[doc = "4: PWM clock /32"]
    SYSCTL_RCC_PWMDIV_32 = 4,
    #[doc = "5: PWM clock /64"]
    SYSCTL_RCC_PWMDIV_64 = 5,
}
impl From<SYSCTL_RCC_PWMDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RCC_PWMDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_RCC_PWMDIV`"]
pub type SYSCTL_RCC_PWMDIV_R = crate::R<u8, SYSCTL_RCC_PWMDIV_A>;
impl SYSCTL_RCC_PWMDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_RCC_PWMDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_2),
            1 => Val(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_4),
            2 => Val(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_8),
            3 => Val(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_16),
            4 => Val(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_32),
            5 => Val(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_PWMDIV_2`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_pwmdiv_2(&self) -> bool {
        *self == SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_2
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_PWMDIV_4`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_pwmdiv_4(&self) -> bool {
        *self == SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_4
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_PWMDIV_8`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_pwmdiv_8(&self) -> bool {
        *self == SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_8
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_PWMDIV_16`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_pwmdiv_16(&self) -> bool {
        *self == SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_16
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_PWMDIV_32`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_pwmdiv_32(&self) -> bool {
        *self == SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_32
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_PWMDIV_64`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_pwmdiv_64(&self) -> bool {
        *self == SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_64
    }
}
#[doc = "Write proxy for field `SYSCTL_RCC_PWMDIV`"]
pub struct SYSCTL_RCC_PWMDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC_PWMDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RCC_PWMDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM clock /2"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv_2(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_2)
    }
    #[doc = "PWM clock /4"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv_4(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_4)
    }
    #[doc = "PWM clock /8"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv_8(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_8)
    }
    #[doc = "PWM clock /16"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv_16(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_16)
    }
    #[doc = "PWM clock /32"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv_32(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_32)
    }
    #[doc = "PWM clock /64"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv_64(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_RCC_USEPWMDIV`"]
pub type SYSCTL_RCC_USEPWMDIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RCC_USEPWMDIV`"]
pub struct SYSCTL_RCC_USEPWMDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC_USEPWMDIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_RCC_USESYSDIV`"]
pub type SYSCTL_RCC_USESYSDIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RCC_USESYSDIV`"]
pub struct SYSCTL_RCC_USESYSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC_USESYSDIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "System Clock Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_RCC_SYSDIV_A {
    #[doc = "1: System clock /2"]
    SYSCTL_RCC_SYSDIV_2 = 1,
    #[doc = "2: System clock /3"]
    SYSCTL_RCC_SYSDIV_3 = 2,
    #[doc = "3: System clock /4"]
    SYSCTL_RCC_SYSDIV_4 = 3,
    #[doc = "4: System clock /5"]
    SYSCTL_RCC_SYSDIV_5 = 4,
    #[doc = "5: System clock /6"]
    SYSCTL_RCC_SYSDIV_6 = 5,
    #[doc = "6: System clock /7"]
    SYSCTL_RCC_SYSDIV_7 = 6,
    #[doc = "7: System clock /8"]
    SYSCTL_RCC_SYSDIV_8 = 7,
    #[doc = "8: System clock /9"]
    SYSCTL_RCC_SYSDIV_9 = 8,
    #[doc = "9: System clock /10"]
    SYSCTL_RCC_SYSDIV_10 = 9,
    #[doc = "10: System clock /11"]
    SYSCTL_RCC_SYSDIV_11 = 10,
    #[doc = "11: System clock /12"]
    SYSCTL_RCC_SYSDIV_12 = 11,
    #[doc = "12: System clock /13"]
    SYSCTL_RCC_SYSDIV_13 = 12,
    #[doc = "13: System clock /14"]
    SYSCTL_RCC_SYSDIV_14 = 13,
    #[doc = "14: System clock /15"]
    SYSCTL_RCC_SYSDIV_15 = 14,
    #[doc = "15: System clock /16"]
    SYSCTL_RCC_SYSDIV_16 = 15,
}
impl From<SYSCTL_RCC_SYSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RCC_SYSDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_RCC_SYSDIV`"]
pub type SYSCTL_RCC_SYSDIV_R = crate::R<u8, SYSCTL_RCC_SYSDIV_A>;
impl SYSCTL_RCC_SYSDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_RCC_SYSDIV_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_2),
            2 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_3),
            3 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_4),
            4 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_5),
            5 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_6),
            6 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_7),
            7 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_8),
            8 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_9),
            9 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_10),
            10 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_11),
            11 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_12),
            12 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_13),
            13 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_14),
            14 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_15),
            15 => Val(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_2`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_2(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_2
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_3`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_3(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_3
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_4`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_4(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_4
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_5`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_5(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_6`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_6(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_6
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_7`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_7(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_7
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_8`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_8(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_8
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_9`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_9(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_9
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_10`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_10(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_10
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_11`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_11(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_11
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_12`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_12(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_12
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_13`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_13(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_13
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_14`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_14(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_14
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_15`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_15(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_15
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC_SYSDIV_16`"]
    #[inline(always)]
    pub fn is_sysctl_rcc_sysdiv_16(&self) -> bool {
        *self == SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_16
    }
}
#[doc = "Write proxy for field `SYSCTL_RCC_SYSDIV`"]
pub struct SYSCTL_RCC_SYSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC_SYSDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RCC_SYSDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "System clock /2"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_2(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_2)
    }
    #[doc = "System clock /3"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_3(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_3)
    }
    #[doc = "System clock /4"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_4(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_4)
    }
    #[doc = "System clock /5"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_5(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_5)
    }
    #[doc = "System clock /6"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_6(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_6)
    }
    #[doc = "System clock /7"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_7(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_7)
    }
    #[doc = "System clock /8"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_8(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_8)
    }
    #[doc = "System clock /9"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_9(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_9)
    }
    #[doc = "System clock /10"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_10(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_10)
    }
    #[doc = "System clock /11"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_11(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_11)
    }
    #[doc = "System clock /12"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_12(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_12)
    }
    #[doc = "System clock /13"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_13(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_13)
    }
    #[doc = "System clock /14"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_14(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_14)
    }
    #[doc = "System clock /15"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_15(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_15)
    }
    #[doc = "System clock /16"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv_16(self) -> &'a mut W {
        self.variant(SYSCTL_RCC_SYSDIV_A::SYSCTL_RCC_SYSDIV_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | (((value as u32) & 0x0f) << 23);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_RCC_ACG`"]
pub type SYSCTL_RCC_ACG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RCC_ACG`"]
pub struct SYSCTL_RCC_ACG_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC_ACG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Main Oscillator Disable"]
    #[inline(always)]
    pub fn sysctl_rcc_moscdis(&self) -> SYSCTL_RCC_MOSCDIS_R {
        SYSCTL_RCC_MOSCDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Oscillator Disable"]
    #[inline(always)]
    pub fn sysctl_rcc_ioscdis(&self) -> SYSCTL_RCC_IOSCDIS_R {
        SYSCTL_RCC_IOSCDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Oscillator Source"]
    #[inline(always)]
    pub fn sysctl_rcc_oscsrc(&self) -> SYSCTL_RCC_OSCSRC_R {
        SYSCTL_RCC_OSCSRC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:9 - Crystal Value"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal(&self) -> SYSCTL_RCC_XTAL_R {
        SYSCTL_RCC_XTAL_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - PLL Bypass"]
    #[inline(always)]
    pub fn sysctl_rcc_bypass(&self) -> SYSCTL_RCC_BYPASS_R {
        SYSCTL_RCC_BYPASS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PLL Power Down"]
    #[inline(always)]
    pub fn sysctl_rcc_pwrdn(&self) -> SYSCTL_RCC_PWRDN_R {
        SYSCTL_RCC_PWRDN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - PWM Unit Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv(&self) -> SYSCTL_RCC_PWMDIV_R {
        SYSCTL_RCC_PWMDIV_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Enable PWM Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rcc_usepwmdiv(&self) -> SYSCTL_RCC_USEPWMDIV_R {
        SYSCTL_RCC_USEPWMDIV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable System Clock Divider"]
    #[inline(always)]
    pub fn sysctl_rcc_usesysdiv(&self) -> SYSCTL_RCC_USESYSDIV_R {
        SYSCTL_RCC_USESYSDIV_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - System Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv(&self) -> SYSCTL_RCC_SYSDIV_R {
        SYSCTL_RCC_SYSDIV_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - Auto Clock Gating"]
    #[inline(always)]
    pub fn sysctl_rcc_acg(&self) -> SYSCTL_RCC_ACG_R {
        SYSCTL_RCC_ACG_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main Oscillator Disable"]
    #[inline(always)]
    pub fn sysctl_rcc_moscdis(&mut self) -> SYSCTL_RCC_MOSCDIS_W {
        SYSCTL_RCC_MOSCDIS_W { w: self }
    }
    #[doc = "Bit 1 - Internal Oscillator Disable"]
    #[inline(always)]
    pub fn sysctl_rcc_ioscdis(&mut self) -> SYSCTL_RCC_IOSCDIS_W {
        SYSCTL_RCC_IOSCDIS_W { w: self }
    }
    #[doc = "Bits 4:5 - Oscillator Source"]
    #[inline(always)]
    pub fn sysctl_rcc_oscsrc(&mut self) -> SYSCTL_RCC_OSCSRC_W {
        SYSCTL_RCC_OSCSRC_W { w: self }
    }
    #[doc = "Bits 6:9 - Crystal Value"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal(&mut self) -> SYSCTL_RCC_XTAL_W {
        SYSCTL_RCC_XTAL_W { w: self }
    }
    #[doc = "Bit 11 - PLL Bypass"]
    #[inline(always)]
    pub fn sysctl_rcc_bypass(&mut self) -> SYSCTL_RCC_BYPASS_W {
        SYSCTL_RCC_BYPASS_W { w: self }
    }
    #[doc = "Bit 13 - PLL Power Down"]
    #[inline(always)]
    pub fn sysctl_rcc_pwrdn(&mut self) -> SYSCTL_RCC_PWRDN_W {
        SYSCTL_RCC_PWRDN_W { w: self }
    }
    #[doc = "Bits 17:19 - PWM Unit Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv(&mut self) -> SYSCTL_RCC_PWMDIV_W {
        SYSCTL_RCC_PWMDIV_W { w: self }
    }
    #[doc = "Bit 20 - Enable PWM Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rcc_usepwmdiv(&mut self) -> SYSCTL_RCC_USEPWMDIV_W {
        SYSCTL_RCC_USEPWMDIV_W { w: self }
    }
    #[doc = "Bit 22 - Enable System Clock Divider"]
    #[inline(always)]
    pub fn sysctl_rcc_usesysdiv(&mut self) -> SYSCTL_RCC_USESYSDIV_W {
        SYSCTL_RCC_USESYSDIV_W { w: self }
    }
    #[doc = "Bits 23:26 - System Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv(&mut self) -> SYSCTL_RCC_SYSDIV_W {
        SYSCTL_RCC_SYSDIV_W { w: self }
    }
    #[doc = "Bit 27 - Auto Clock Gating"]
    #[inline(always)]
    pub fn sysctl_rcc_acg(&mut self) -> SYSCTL_RCC_ACG_W {
        SYSCTL_RCC_ACG_W { w: self }
    }
}
