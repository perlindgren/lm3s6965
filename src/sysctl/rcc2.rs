#[doc = "Reader of register RCC2"]
pub type R = crate::R<u32, super::RCC2>;
#[doc = "Writer for register RCC2"]
pub type W = crate::W<u32, super::RCC2>;
#[doc = "Register RCC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Oscillator Source 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_RCC2_OSCSRC2_A {
    #[doc = "0: MOSC"]
    SYSCTL_RCC2_OSCSRC2_MO = 0,
    #[doc = "1: PIOSC"]
    SYSCTL_RCC2_OSCSRC2_IO = 1,
    #[doc = "2: PIOSC/4"]
    SYSCTL_RCC2_OSCSRC2_IO4 = 2,
    #[doc = "3: 30 kHz"]
    SYSCTL_RCC2_OSCSRC2_30 = 3,
    #[doc = "7: 32.768 kHz"]
    SYSCTL_RCC2_OSCSRC2_32 = 7,
}
impl From<SYSCTL_RCC2_OSCSRC2_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RCC2_OSCSRC2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_RCC2_OSCSRC2`"]
pub type SYSCTL_RCC2_OSCSRC2_R = crate::R<u8, SYSCTL_RCC2_OSCSRC2_A>;
impl SYSCTL_RCC2_OSCSRC2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_RCC2_OSCSRC2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_MO),
            1 => Val(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_IO),
            2 => Val(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_IO4),
            3 => Val(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_30),
            7 => Val(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_OSCSRC2_MO`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_oscsrc2_mo(&self) -> bool {
        *self == SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_MO
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_OSCSRC2_IO`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_oscsrc2_io(&self) -> bool {
        *self == SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_IO
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_OSCSRC2_IO4`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_oscsrc2_io4(&self) -> bool {
        *self == SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_IO4
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_OSCSRC2_30`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_oscsrc2_30(&self) -> bool {
        *self == SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_30
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_OSCSRC2_32`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_oscsrc2_32(&self) -> bool {
        *self == SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_32
    }
}
#[doc = "Write proxy for field `SYSCTL_RCC2_OSCSRC2`"]
pub struct SYSCTL_RCC2_OSCSRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC2_OSCSRC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RCC2_OSCSRC2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn sysctl_rcc2_oscsrc2_mo(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_MO)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn sysctl_rcc2_oscsrc2_io(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_IO)
    }
    #[doc = "PIOSC/4"]
    #[inline(always)]
    pub fn sysctl_rcc2_oscsrc2_io4(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_IO4)
    }
    #[doc = "30 kHz"]
    #[inline(always)]
    pub fn sysctl_rcc2_oscsrc2_30(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_30)
    }
    #[doc = "32.768 kHz"]
    #[inline(always)]
    pub fn sysctl_rcc2_oscsrc2_32(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_RCC2_BYPASS2`"]
pub type SYSCTL_RCC2_BYPASS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RCC2_BYPASS2`"]
pub struct SYSCTL_RCC2_BYPASS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC2_BYPASS2_W<'a> {
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
#[doc = "Reader of field `SYSCTL_RCC2_PWRDN2`"]
pub type SYSCTL_RCC2_PWRDN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RCC2_PWRDN2`"]
pub struct SYSCTL_RCC2_PWRDN2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC2_PWRDN2_W<'a> {
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
#[doc = "System Clock Divisor 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_RCC2_SYSDIV2_A {
    #[doc = "1: System clock /2"]
    SYSCTL_RCC2_SYSDIV2_2 = 1,
    #[doc = "2: System clock /3"]
    SYSCTL_RCC2_SYSDIV2_3 = 2,
    #[doc = "3: System clock /4"]
    SYSCTL_RCC2_SYSDIV2_4 = 3,
    #[doc = "4: System clock /5"]
    SYSCTL_RCC2_SYSDIV2_5 = 4,
    #[doc = "5: System clock /6"]
    SYSCTL_RCC2_SYSDIV2_6 = 5,
    #[doc = "6: System clock /7"]
    SYSCTL_RCC2_SYSDIV2_7 = 6,
    #[doc = "7: System clock /8"]
    SYSCTL_RCC2_SYSDIV2_8 = 7,
    #[doc = "8: System clock /9"]
    SYSCTL_RCC2_SYSDIV2_9 = 8,
    #[doc = "9: System clock /10"]
    SYSCTL_RCC2_SYSDIV2_10 = 9,
    #[doc = "10: System clock /11"]
    SYSCTL_RCC2_SYSDIV2_11 = 10,
    #[doc = "11: System clock /12"]
    SYSCTL_RCC2_SYSDIV2_12 = 11,
    #[doc = "12: System clock /13"]
    SYSCTL_RCC2_SYSDIV2_13 = 12,
    #[doc = "13: System clock /14"]
    SYSCTL_RCC2_SYSDIV2_14 = 13,
    #[doc = "14: System clock /15"]
    SYSCTL_RCC2_SYSDIV2_15 = 14,
    #[doc = "15: System clock /16"]
    SYSCTL_RCC2_SYSDIV2_16 = 15,
    #[doc = "16: System clock /17"]
    SYSCTL_RCC2_SYSDIV2_17 = 16,
    #[doc = "17: System clock /18"]
    SYSCTL_RCC2_SYSDIV2_18 = 17,
    #[doc = "18: System clock /19"]
    SYSCTL_RCC2_SYSDIV2_19 = 18,
    #[doc = "19: System clock /20"]
    SYSCTL_RCC2_SYSDIV2_20 = 19,
    #[doc = "20: System clock /21"]
    SYSCTL_RCC2_SYSDIV2_21 = 20,
    #[doc = "21: System clock /22"]
    SYSCTL_RCC2_SYSDIV2_22 = 21,
    #[doc = "22: System clock /23"]
    SYSCTL_RCC2_SYSDIV2_23 = 22,
    #[doc = "23: System clock /24"]
    SYSCTL_RCC2_SYSDIV2_24 = 23,
    #[doc = "24: System clock /25"]
    SYSCTL_RCC2_SYSDIV2_25 = 24,
    #[doc = "25: System clock /26"]
    SYSCTL_RCC2_SYSDIV2_26 = 25,
    #[doc = "26: System clock /27"]
    SYSCTL_RCC2_SYSDIV2_27 = 26,
    #[doc = "27: System clock /28"]
    SYSCTL_RCC2_SYSDIV2_28 = 27,
    #[doc = "28: System clock /29"]
    SYSCTL_RCC2_SYSDIV2_29 = 28,
    #[doc = "29: System clock /30"]
    SYSCTL_RCC2_SYSDIV2_30 = 29,
    #[doc = "30: System clock /31"]
    SYSCTL_RCC2_SYSDIV2_31 = 30,
    #[doc = "31: System clock /32"]
    SYSCTL_RCC2_SYSDIV2_32 = 31,
    #[doc = "32: System clock /33"]
    SYSCTL_RCC2_SYSDIV2_33 = 32,
    #[doc = "33: System clock /34"]
    SYSCTL_RCC2_SYSDIV2_34 = 33,
    #[doc = "34: System clock /35"]
    SYSCTL_RCC2_SYSDIV2_35 = 34,
    #[doc = "35: System clock /36"]
    SYSCTL_RCC2_SYSDIV2_36 = 35,
    #[doc = "36: System clock /37"]
    SYSCTL_RCC2_SYSDIV2_37 = 36,
    #[doc = "37: System clock /38"]
    SYSCTL_RCC2_SYSDIV2_38 = 37,
    #[doc = "38: System clock /39"]
    SYSCTL_RCC2_SYSDIV2_39 = 38,
    #[doc = "39: System clock /40"]
    SYSCTL_RCC2_SYSDIV2_40 = 39,
    #[doc = "40: System clock /41"]
    SYSCTL_RCC2_SYSDIV2_41 = 40,
    #[doc = "41: System clock /42"]
    SYSCTL_RCC2_SYSDIV2_42 = 41,
    #[doc = "42: System clock /43"]
    SYSCTL_RCC2_SYSDIV2_43 = 42,
    #[doc = "43: System clock /44"]
    SYSCTL_RCC2_SYSDIV2_44 = 43,
    #[doc = "44: System clock /45"]
    SYSCTL_RCC2_SYSDIV2_45 = 44,
    #[doc = "45: System clock /46"]
    SYSCTL_RCC2_SYSDIV2_46 = 45,
    #[doc = "46: System clock /47"]
    SYSCTL_RCC2_SYSDIV2_47 = 46,
    #[doc = "47: System clock /48"]
    SYSCTL_RCC2_SYSDIV2_48 = 47,
    #[doc = "48: System clock /49"]
    SYSCTL_RCC2_SYSDIV2_49 = 48,
    #[doc = "49: System clock /50"]
    SYSCTL_RCC2_SYSDIV2_50 = 49,
    #[doc = "50: System clock /51"]
    SYSCTL_RCC2_SYSDIV2_51 = 50,
    #[doc = "51: System clock /52"]
    SYSCTL_RCC2_SYSDIV2_52 = 51,
    #[doc = "52: System clock /53"]
    SYSCTL_RCC2_SYSDIV2_53 = 52,
    #[doc = "53: System clock /54"]
    SYSCTL_RCC2_SYSDIV2_54 = 53,
    #[doc = "54: System clock /55"]
    SYSCTL_RCC2_SYSDIV2_55 = 54,
    #[doc = "55: System clock /56"]
    SYSCTL_RCC2_SYSDIV2_56 = 55,
    #[doc = "56: System clock /57"]
    SYSCTL_RCC2_SYSDIV2_57 = 56,
    #[doc = "57: System clock /58"]
    SYSCTL_RCC2_SYSDIV2_58 = 57,
    #[doc = "58: System clock /59"]
    SYSCTL_RCC2_SYSDIV2_59 = 58,
    #[doc = "59: System clock /60"]
    SYSCTL_RCC2_SYSDIV2_60 = 59,
    #[doc = "60: System clock /61"]
    SYSCTL_RCC2_SYSDIV2_61 = 60,
    #[doc = "61: System clock /62"]
    SYSCTL_RCC2_SYSDIV2_62 = 61,
    #[doc = "62: System clock /63"]
    SYSCTL_RCC2_SYSDIV2_63 = 62,
    #[doc = "63: System clock /64"]
    SYSCTL_RCC2_SYSDIV2_64 = 63,
}
impl From<SYSCTL_RCC2_SYSDIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RCC2_SYSDIV2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_RCC2_SYSDIV2`"]
pub type SYSCTL_RCC2_SYSDIV2_R = crate::R<u8, SYSCTL_RCC2_SYSDIV2_A>;
impl SYSCTL_RCC2_SYSDIV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_RCC2_SYSDIV2_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_2),
            2 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_3),
            3 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_4),
            4 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_5),
            5 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_6),
            6 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_7),
            7 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_8),
            8 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_9),
            9 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_10),
            10 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_11),
            11 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_12),
            12 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_13),
            13 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_14),
            14 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_15),
            15 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_16),
            16 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_17),
            17 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_18),
            18 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_19),
            19 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_20),
            20 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_21),
            21 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_22),
            22 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_23),
            23 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_24),
            24 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_25),
            25 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_26),
            26 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_27),
            27 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_28),
            28 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_29),
            29 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_30),
            30 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_31),
            31 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_32),
            32 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_33),
            33 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_34),
            34 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_35),
            35 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_36),
            36 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_37),
            37 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_38),
            38 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_39),
            39 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_40),
            40 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_41),
            41 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_42),
            42 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_43),
            43 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_44),
            44 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_45),
            45 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_46),
            46 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_47),
            47 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_48),
            48 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_49),
            49 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_50),
            50 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_51),
            51 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_52),
            52 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_53),
            53 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_54),
            54 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_55),
            55 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_56),
            56 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_57),
            57 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_58),
            58 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_59),
            59 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_60),
            60 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_61),
            61 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_62),
            62 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_63),
            63 => Val(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_2`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_2(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_2
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_3`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_3(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_3
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_4`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_4(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_4
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_5`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_5(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_6`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_6(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_6
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_7`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_7(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_7
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_8`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_8(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_8
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_9`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_9(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_9
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_10`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_10(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_10
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_11`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_11(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_11
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_12`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_12(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_12
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_13`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_13(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_13
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_14`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_14(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_14
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_15`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_15(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_15
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_16`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_16(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_16
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_17`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_17(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_17
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_18`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_18(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_18
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_19`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_19(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_19
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_20`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_20(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_20
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_21`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_21(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_21
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_22`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_22(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_22
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_23`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_23(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_23
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_24`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_24(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_24
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_25`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_25(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_25
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_26`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_26(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_26
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_27`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_27(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_27
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_28`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_28(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_28
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_29`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_29(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_29
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_30`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_30(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_30
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_31`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_31(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_31
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_32`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_32(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_32
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_33`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_33(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_33
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_34`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_34(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_34
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_35`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_35(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_35
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_36`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_36(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_36
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_37`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_37(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_37
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_38`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_38(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_38
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_39`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_39(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_39
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_40`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_40(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_40
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_41`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_41(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_41
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_42`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_42(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_42
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_43`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_43(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_43
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_44`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_44(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_44
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_45`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_45(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_45
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_46`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_46(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_46
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_47`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_47(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_47
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_48`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_48(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_48
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_49`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_49(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_49
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_50`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_50(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_50
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_51`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_51(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_51
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_52`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_52(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_52
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_53`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_53(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_53
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_54`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_54(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_54
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_55`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_55(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_55
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_56`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_56(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_56
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_57`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_57(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_57
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_58`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_58(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_58
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_59`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_59(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_59
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_60`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_60(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_60
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_61`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_61(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_61
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_62`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_62(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_62
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_63`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_63(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_63
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RCC2_SYSDIV2_64`"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_sysdiv2_64(&self) -> bool {
        *self == SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_64
    }
}
#[doc = "Write proxy for field `SYSCTL_RCC2_SYSDIV2`"]
pub struct SYSCTL_RCC2_SYSDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC2_SYSDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RCC2_SYSDIV2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "System clock /2"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_2(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_2)
    }
    #[doc = "System clock /3"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_3(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_3)
    }
    #[doc = "System clock /4"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_4(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_4)
    }
    #[doc = "System clock /5"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_5(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_5)
    }
    #[doc = "System clock /6"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_6(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_6)
    }
    #[doc = "System clock /7"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_7(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_7)
    }
    #[doc = "System clock /8"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_8(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_8)
    }
    #[doc = "System clock /9"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_9(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_9)
    }
    #[doc = "System clock /10"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_10(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_10)
    }
    #[doc = "System clock /11"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_11(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_11)
    }
    #[doc = "System clock /12"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_12(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_12)
    }
    #[doc = "System clock /13"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_13(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_13)
    }
    #[doc = "System clock /14"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_14(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_14)
    }
    #[doc = "System clock /15"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_15(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_15)
    }
    #[doc = "System clock /16"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_16(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_16)
    }
    #[doc = "System clock /17"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_17(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_17)
    }
    #[doc = "System clock /18"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_18(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_18)
    }
    #[doc = "System clock /19"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_19(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_19)
    }
    #[doc = "System clock /20"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_20(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_20)
    }
    #[doc = "System clock /21"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_21(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_21)
    }
    #[doc = "System clock /22"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_22(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_22)
    }
    #[doc = "System clock /23"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_23(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_23)
    }
    #[doc = "System clock /24"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_24(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_24)
    }
    #[doc = "System clock /25"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_25(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_25)
    }
    #[doc = "System clock /26"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_26(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_26)
    }
    #[doc = "System clock /27"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_27(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_27)
    }
    #[doc = "System clock /28"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_28(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_28)
    }
    #[doc = "System clock /29"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_29(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_29)
    }
    #[doc = "System clock /30"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_30(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_30)
    }
    #[doc = "System clock /31"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_31(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_31)
    }
    #[doc = "System clock /32"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_32(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_32)
    }
    #[doc = "System clock /33"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_33(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_33)
    }
    #[doc = "System clock /34"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_34(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_34)
    }
    #[doc = "System clock /35"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_35(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_35)
    }
    #[doc = "System clock /36"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_36(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_36)
    }
    #[doc = "System clock /37"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_37(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_37)
    }
    #[doc = "System clock /38"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_38(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_38)
    }
    #[doc = "System clock /39"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_39(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_39)
    }
    #[doc = "System clock /40"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_40(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_40)
    }
    #[doc = "System clock /41"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_41(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_41)
    }
    #[doc = "System clock /42"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_42(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_42)
    }
    #[doc = "System clock /43"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_43(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_43)
    }
    #[doc = "System clock /44"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_44(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_44)
    }
    #[doc = "System clock /45"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_45(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_45)
    }
    #[doc = "System clock /46"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_46(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_46)
    }
    #[doc = "System clock /47"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_47(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_47)
    }
    #[doc = "System clock /48"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_48(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_48)
    }
    #[doc = "System clock /49"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_49(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_49)
    }
    #[doc = "System clock /50"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_50(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_50)
    }
    #[doc = "System clock /51"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_51(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_51)
    }
    #[doc = "System clock /52"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_52(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_52)
    }
    #[doc = "System clock /53"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_53(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_53)
    }
    #[doc = "System clock /54"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_54(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_54)
    }
    #[doc = "System clock /55"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_55(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_55)
    }
    #[doc = "System clock /56"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_56(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_56)
    }
    #[doc = "System clock /57"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_57(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_57)
    }
    #[doc = "System clock /58"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_58(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_58)
    }
    #[doc = "System clock /59"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_59(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_59)
    }
    #[doc = "System clock /60"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_60(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_60)
    }
    #[doc = "System clock /61"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_61(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_61)
    }
    #[doc = "System clock /62"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_62(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_62)
    }
    #[doc = "System clock /63"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_63(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_63)
    }
    #[doc = "System clock /64"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2_64(self) -> &'a mut W {
        self.variant(SYSCTL_RCC2_SYSDIV2_A::SYSCTL_RCC2_SYSDIV2_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 23)) | (((value as u32) & 0x3f) << 23);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_RCC2_USERCC2`"]
pub type SYSCTL_RCC2_USERCC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_RCC2_USERCC2`"]
pub struct SYSCTL_RCC2_USERCC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RCC2_USERCC2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - Oscillator Source 2"]
    #[inline(always)]
    pub fn sysctl_rcc2_oscsrc2(&self) -> SYSCTL_RCC2_OSCSRC2_R {
        SYSCTL_RCC2_OSCSRC2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 11 - PLL Bypass 2"]
    #[inline(always)]
    pub fn sysctl_rcc2_bypass2(&self) -> SYSCTL_RCC2_BYPASS2_R {
        SYSCTL_RCC2_BYPASS2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Power-Down PLL 2"]
    #[inline(always)]
    pub fn sysctl_rcc2_pwrdn2(&self) -> SYSCTL_RCC2_PWRDN2_R {
        SYSCTL_RCC2_PWRDN2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 23:28 - System Clock Divisor 2"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2(&self) -> SYSCTL_RCC2_SYSDIV2_R {
        SYSCTL_RCC2_SYSDIV2_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Use RCC2"]
    #[inline(always)]
    pub fn sysctl_rcc2_usercc2(&self) -> SYSCTL_RCC2_USERCC2_R {
        SYSCTL_RCC2_USERCC2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - Oscillator Source 2"]
    #[inline(always)]
    pub fn sysctl_rcc2_oscsrc2(&mut self) -> SYSCTL_RCC2_OSCSRC2_W {
        SYSCTL_RCC2_OSCSRC2_W { w: self }
    }
    #[doc = "Bit 11 - PLL Bypass 2"]
    #[inline(always)]
    pub fn sysctl_rcc2_bypass2(&mut self) -> SYSCTL_RCC2_BYPASS2_W {
        SYSCTL_RCC2_BYPASS2_W { w: self }
    }
    #[doc = "Bit 13 - Power-Down PLL 2"]
    #[inline(always)]
    pub fn sysctl_rcc2_pwrdn2(&mut self) -> SYSCTL_RCC2_PWRDN2_W {
        SYSCTL_RCC2_PWRDN2_W { w: self }
    }
    #[doc = "Bits 23:28 - System Clock Divisor 2"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2(&mut self) -> SYSCTL_RCC2_SYSDIV2_W {
        SYSCTL_RCC2_SYSDIV2_W { w: self }
    }
    #[doc = "Bit 31 - Use RCC2"]
    #[inline(always)]
    pub fn sysctl_rcc2_usercc2(&mut self) -> SYSCTL_RCC2_USERCC2_W {
        SYSCTL_RCC2_USERCC2_W { w: self }
    }
}
