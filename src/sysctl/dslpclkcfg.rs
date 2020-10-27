#[doc = "Reader of register DSLPCLKCFG"]
pub type R = crate::R<u32, super::DSLPCLKCFG>;
#[doc = "Writer for register DSLPCLKCFG"]
pub type W = crate::W<u32, super::DSLPCLKCFG>;
#[doc = "Register DSLPCLKCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DSLPCLKCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DSLPCLKCFG_O_A {
    #[doc = "0: MOSC"]
    SYSCTL_DSLPCLKCFG_O_IGN = 0,
    #[doc = "1: PIOSC"]
    SYSCTL_DSLPCLKCFG_O_IO = 1,
    #[doc = "3: 30 kHz"]
    SYSCTL_DSLPCLKCFG_O_30 = 3,
    #[doc = "7: 32.768 kHz"]
    SYSCTL_DSLPCLKCFG_O_32 = 7,
}
impl From<SYSCTL_DSLPCLKCFG_O_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DSLPCLKCFG_O_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DSLPCLKCFG_O`"]
pub type SYSCTL_DSLPCLKCFG_O_R = crate::R<u8, SYSCTL_DSLPCLKCFG_O_A>;
impl SYSCTL_DSLPCLKCFG_O_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCTL_DSLPCLKCFG_O_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_IGN),
            1 => Val(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_IO),
            3 => Val(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_30),
            7 => Val(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_O_IGN`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_o_ign(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_IGN
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_O_IO`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_o_io(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_IO
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_O_30`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_o_30(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_30
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_O_32`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_o_32(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_32
    }
}
#[doc = "Write proxy for field `SYSCTL_DSLPCLKCFG_O`"]
pub struct SYSCTL_DSLPCLKCFG_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DSLPCLKCFG_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DSLPCLKCFG_O_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_o_ign(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_IGN)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_o_io(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_IO)
    }
    #[doc = "30 kHz"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_o_30(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_30)
    }
    #[doc = "32.768 kHz"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_o_32(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Divider Field Override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DSLPCLKCFG_D_A {
    #[doc = "0: System clock /1"]
    SYSCTL_DSLPCLKCFG_D_1 = 0,
    #[doc = "1: System clock /2"]
    SYSCTL_DSLPCLKCFG_D_2 = 1,
    #[doc = "2: System clock /3"]
    SYSCTL_DSLPCLKCFG_D_3 = 2,
    #[doc = "3: System clock /4"]
    SYSCTL_DSLPCLKCFG_D_4 = 3,
    #[doc = "4: System clock /5"]
    SYSCTL_DSLPCLKCFG_D_5 = 4,
    #[doc = "5: System clock /6"]
    SYSCTL_DSLPCLKCFG_D_6 = 5,
    #[doc = "6: System clock /7"]
    SYSCTL_DSLPCLKCFG_D_7 = 6,
    #[doc = "7: System clock /8"]
    SYSCTL_DSLPCLKCFG_D_8 = 7,
    #[doc = "8: System clock /9"]
    SYSCTL_DSLPCLKCFG_D_9 = 8,
    #[doc = "9: System clock /10"]
    SYSCTL_DSLPCLKCFG_D_10 = 9,
    #[doc = "10: System clock /11"]
    SYSCTL_DSLPCLKCFG_D_11 = 10,
    #[doc = "11: System clock /12"]
    SYSCTL_DSLPCLKCFG_D_12 = 11,
    #[doc = "12: System clock /13"]
    SYSCTL_DSLPCLKCFG_D_13 = 12,
    #[doc = "13: System clock /14"]
    SYSCTL_DSLPCLKCFG_D_14 = 13,
    #[doc = "14: System clock /15"]
    SYSCTL_DSLPCLKCFG_D_15 = 14,
    #[doc = "15: System clock /16"]
    SYSCTL_DSLPCLKCFG_D_16 = 15,
    #[doc = "16: System clock /17"]
    SYSCTL_DSLPCLKCFG_D_17 = 16,
    #[doc = "17: System clock /18"]
    SYSCTL_DSLPCLKCFG_D_18 = 17,
    #[doc = "18: System clock /19"]
    SYSCTL_DSLPCLKCFG_D_19 = 18,
    #[doc = "19: System clock /20"]
    SYSCTL_DSLPCLKCFG_D_20 = 19,
    #[doc = "20: System clock /21"]
    SYSCTL_DSLPCLKCFG_D_21 = 20,
    #[doc = "21: System clock /22"]
    SYSCTL_DSLPCLKCFG_D_22 = 21,
    #[doc = "22: System clock /23"]
    SYSCTL_DSLPCLKCFG_D_23 = 22,
    #[doc = "23: System clock /24"]
    SYSCTL_DSLPCLKCFG_D_24 = 23,
    #[doc = "24: System clock /25"]
    SYSCTL_DSLPCLKCFG_D_25 = 24,
    #[doc = "25: System clock /26"]
    SYSCTL_DSLPCLKCFG_D_26 = 25,
    #[doc = "26: System clock /27"]
    SYSCTL_DSLPCLKCFG_D_27 = 26,
    #[doc = "27: System clock /28"]
    SYSCTL_DSLPCLKCFG_D_28 = 27,
    #[doc = "28: System clock /29"]
    SYSCTL_DSLPCLKCFG_D_29 = 28,
    #[doc = "29: System clock /30"]
    SYSCTL_DSLPCLKCFG_D_30 = 29,
    #[doc = "30: System clock /31"]
    SYSCTL_DSLPCLKCFG_D_31 = 30,
    #[doc = "31: System clock /32"]
    SYSCTL_DSLPCLKCFG_D_32 = 31,
    #[doc = "32: System clock /33"]
    SYSCTL_DSLPCLKCFG_D_33 = 32,
    #[doc = "33: System clock /34"]
    SYSCTL_DSLPCLKCFG_D_34 = 33,
    #[doc = "34: System clock /35"]
    SYSCTL_DSLPCLKCFG_D_35 = 34,
    #[doc = "35: System clock /36"]
    SYSCTL_DSLPCLKCFG_D_36 = 35,
    #[doc = "36: System clock /37"]
    SYSCTL_DSLPCLKCFG_D_37 = 36,
    #[doc = "37: System clock /38"]
    SYSCTL_DSLPCLKCFG_D_38 = 37,
    #[doc = "38: System clock /39"]
    SYSCTL_DSLPCLKCFG_D_39 = 38,
    #[doc = "39: System clock /40"]
    SYSCTL_DSLPCLKCFG_D_40 = 39,
    #[doc = "40: System clock /41"]
    SYSCTL_DSLPCLKCFG_D_41 = 40,
    #[doc = "41: System clock /42"]
    SYSCTL_DSLPCLKCFG_D_42 = 41,
    #[doc = "42: System clock /43"]
    SYSCTL_DSLPCLKCFG_D_43 = 42,
    #[doc = "43: System clock /44"]
    SYSCTL_DSLPCLKCFG_D_44 = 43,
    #[doc = "44: System clock /45"]
    SYSCTL_DSLPCLKCFG_D_45 = 44,
    #[doc = "45: System clock /46"]
    SYSCTL_DSLPCLKCFG_D_46 = 45,
    #[doc = "46: System clock /47"]
    SYSCTL_DSLPCLKCFG_D_47 = 46,
    #[doc = "47: System clock /48"]
    SYSCTL_DSLPCLKCFG_D_48 = 47,
    #[doc = "48: System clock /49"]
    SYSCTL_DSLPCLKCFG_D_49 = 48,
    #[doc = "49: System clock /50"]
    SYSCTL_DSLPCLKCFG_D_50 = 49,
    #[doc = "50: System clock /51"]
    SYSCTL_DSLPCLKCFG_D_51 = 50,
    #[doc = "51: System clock /52"]
    SYSCTL_DSLPCLKCFG_D_52 = 51,
    #[doc = "52: System clock /53"]
    SYSCTL_DSLPCLKCFG_D_53 = 52,
    #[doc = "53: System clock /54"]
    SYSCTL_DSLPCLKCFG_D_54 = 53,
    #[doc = "54: System clock /55"]
    SYSCTL_DSLPCLKCFG_D_55 = 54,
    #[doc = "55: System clock /56"]
    SYSCTL_DSLPCLKCFG_D_56 = 55,
    #[doc = "56: System clock /57"]
    SYSCTL_DSLPCLKCFG_D_57 = 56,
    #[doc = "57: System clock /58"]
    SYSCTL_DSLPCLKCFG_D_58 = 57,
    #[doc = "58: System clock /59"]
    SYSCTL_DSLPCLKCFG_D_59 = 58,
    #[doc = "59: System clock /60"]
    SYSCTL_DSLPCLKCFG_D_60 = 59,
    #[doc = "60: System clock /61"]
    SYSCTL_DSLPCLKCFG_D_61 = 60,
    #[doc = "61: System clock /62"]
    SYSCTL_DSLPCLKCFG_D_62 = 61,
    #[doc = "62: System clock /63"]
    SYSCTL_DSLPCLKCFG_D_63 = 62,
    #[doc = "63: System clock /64"]
    SYSCTL_DSLPCLKCFG_D_64 = 63,
}
impl From<SYSCTL_DSLPCLKCFG_D_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DSLPCLKCFG_D_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTL_DSLPCLKCFG_D`"]
pub type SYSCTL_DSLPCLKCFG_D_R = crate::R<u8, SYSCTL_DSLPCLKCFG_D_A>;
impl SYSCTL_DSLPCLKCFG_D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTL_DSLPCLKCFG_D_A {
        match self.bits {
            0 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_1,
            1 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_2,
            2 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_3,
            3 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_4,
            4 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_5,
            5 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_6,
            6 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_7,
            7 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_8,
            8 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_9,
            9 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_10,
            10 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_11,
            11 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_12,
            12 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_13,
            13 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_14,
            14 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_15,
            15 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_16,
            16 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_17,
            17 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_18,
            18 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_19,
            19 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_20,
            20 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_21,
            21 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_22,
            22 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_23,
            23 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_24,
            24 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_25,
            25 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_26,
            26 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_27,
            27 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_28,
            28 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_29,
            29 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_30,
            30 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_31,
            31 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_32,
            32 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_33,
            33 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_34,
            34 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_35,
            35 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_36,
            36 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_37,
            37 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_38,
            38 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_39,
            39 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_40,
            40 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_41,
            41 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_42,
            42 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_43,
            43 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_44,
            44 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_45,
            45 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_46,
            46 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_47,
            47 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_48,
            48 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_49,
            49 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_50,
            50 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_51,
            51 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_52,
            52 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_53,
            53 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_54,
            54 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_55,
            55 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_56,
            56 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_57,
            57 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_58,
            58 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_59,
            59 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_60,
            60 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_61,
            61 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_62,
            62 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_63,
            63 => SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_1`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_1(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_1
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_2`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_2(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_2
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_3`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_3(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_3
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_4`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_4(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_4
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_5`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_5(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_6`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_6(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_6
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_7`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_7(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_7
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_8`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_8(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_8
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_9`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_9(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_9
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_10`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_10(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_10
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_11`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_11(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_11
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_12`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_12(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_12
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_13`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_13(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_13
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_14`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_14(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_14
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_15`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_15(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_15
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_16`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_16(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_16
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_17`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_17(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_17
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_18`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_18(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_18
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_19`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_19(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_19
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_20`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_20(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_20
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_21`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_21(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_21
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_22`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_22(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_22
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_23`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_23(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_23
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_24`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_24(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_24
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_25`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_25(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_25
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_26`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_26(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_26
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_27`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_27(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_27
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_28`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_28(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_28
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_29`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_29(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_29
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_30`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_30(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_30
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_31`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_31(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_31
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_32`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_32(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_32
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_33`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_33(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_33
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_34`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_34(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_34
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_35`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_35(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_35
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_36`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_36(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_36
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_37`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_37(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_37
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_38`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_38(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_38
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_39`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_39(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_39
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_40`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_40(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_40
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_41`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_41(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_41
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_42`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_42(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_42
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_43`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_43(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_43
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_44`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_44(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_44
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_45`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_45(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_45
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_46`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_46(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_46
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_47`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_47(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_47
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_48`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_48(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_48
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_49`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_49(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_49
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_50`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_50(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_50
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_51`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_51(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_51
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_52`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_52(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_52
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_53`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_53(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_53
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_54`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_54(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_54
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_55`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_55(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_55
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_56`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_56(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_56
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_57`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_57(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_57
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_58`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_58(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_58
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_59`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_59(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_59
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_60`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_60(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_60
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_61`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_61(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_61
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_62`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_62(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_62
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_63`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_63(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_63
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPCLKCFG_D_64`"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_d_64(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_64
    }
}
#[doc = "Write proxy for field `SYSCTL_DSLPCLKCFG_D`"]
pub struct SYSCTL_DSLPCLKCFG_D_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DSLPCLKCFG_D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DSLPCLKCFG_D_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "System clock /1"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_1(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_1)
    }
    #[doc = "System clock /2"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_2(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_2)
    }
    #[doc = "System clock /3"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_3(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_3)
    }
    #[doc = "System clock /4"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_4(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_4)
    }
    #[doc = "System clock /5"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_5(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_5)
    }
    #[doc = "System clock /6"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_6(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_6)
    }
    #[doc = "System clock /7"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_7(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_7)
    }
    #[doc = "System clock /8"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_8(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_8)
    }
    #[doc = "System clock /9"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_9(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_9)
    }
    #[doc = "System clock /10"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_10(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_10)
    }
    #[doc = "System clock /11"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_11(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_11)
    }
    #[doc = "System clock /12"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_12(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_12)
    }
    #[doc = "System clock /13"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_13(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_13)
    }
    #[doc = "System clock /14"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_14(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_14)
    }
    #[doc = "System clock /15"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_15(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_15)
    }
    #[doc = "System clock /16"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_16(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_16)
    }
    #[doc = "System clock /17"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_17(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_17)
    }
    #[doc = "System clock /18"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_18(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_18)
    }
    #[doc = "System clock /19"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_19(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_19)
    }
    #[doc = "System clock /20"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_20(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_20)
    }
    #[doc = "System clock /21"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_21(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_21)
    }
    #[doc = "System clock /22"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_22(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_22)
    }
    #[doc = "System clock /23"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_23(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_23)
    }
    #[doc = "System clock /24"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_24(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_24)
    }
    #[doc = "System clock /25"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_25(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_25)
    }
    #[doc = "System clock /26"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_26(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_26)
    }
    #[doc = "System clock /27"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_27(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_27)
    }
    #[doc = "System clock /28"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_28(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_28)
    }
    #[doc = "System clock /29"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_29(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_29)
    }
    #[doc = "System clock /30"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_30(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_30)
    }
    #[doc = "System clock /31"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_31(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_31)
    }
    #[doc = "System clock /32"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_32(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_32)
    }
    #[doc = "System clock /33"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_33(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_33)
    }
    #[doc = "System clock /34"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_34(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_34)
    }
    #[doc = "System clock /35"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_35(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_35)
    }
    #[doc = "System clock /36"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_36(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_36)
    }
    #[doc = "System clock /37"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_37(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_37)
    }
    #[doc = "System clock /38"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_38(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_38)
    }
    #[doc = "System clock /39"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_39(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_39)
    }
    #[doc = "System clock /40"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_40(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_40)
    }
    #[doc = "System clock /41"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_41(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_41)
    }
    #[doc = "System clock /42"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_42(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_42)
    }
    #[doc = "System clock /43"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_43(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_43)
    }
    #[doc = "System clock /44"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_44(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_44)
    }
    #[doc = "System clock /45"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_45(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_45)
    }
    #[doc = "System clock /46"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_46(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_46)
    }
    #[doc = "System clock /47"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_47(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_47)
    }
    #[doc = "System clock /48"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_48(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_48)
    }
    #[doc = "System clock /49"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_49(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_49)
    }
    #[doc = "System clock /50"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_50(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_50)
    }
    #[doc = "System clock /51"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_51(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_51)
    }
    #[doc = "System clock /52"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_52(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_52)
    }
    #[doc = "System clock /53"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_53(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_53)
    }
    #[doc = "System clock /54"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_54(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_54)
    }
    #[doc = "System clock /55"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_55(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_55)
    }
    #[doc = "System clock /56"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_56(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_56)
    }
    #[doc = "System clock /57"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_57(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_57)
    }
    #[doc = "System clock /58"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_58(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_58)
    }
    #[doc = "System clock /59"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_59(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_59)
    }
    #[doc = "System clock /60"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_60(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_60)
    }
    #[doc = "System clock /61"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_61(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_61)
    }
    #[doc = "System clock /62"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_62(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_62)
    }
    #[doc = "System clock /63"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_63(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_63)
    }
    #[doc = "System clock /64"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d_64(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPCLKCFG_D_A::SYSCTL_DSLPCLKCFG_D_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 23)) | (((value as u32) & 0x3f) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - Clock Source"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_o(&self) -> SYSCTL_DSLPCLKCFG_O_R {
        SYSCTL_DSLPCLKCFG_O_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 23:28 - Divider Field Override"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d(&self) -> SYSCTL_DSLPCLKCFG_D_R {
        SYSCTL_DSLPCLKCFG_D_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Clock Source"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_o(&mut self) -> SYSCTL_DSLPCLKCFG_O_W {
        SYSCTL_DSLPCLKCFG_O_W { w: self }
    }
    #[doc = "Bits 23:28 - Divider Field Override"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d(&mut self) -> SYSCTL_DSLPCLKCFG_D_W {
        SYSCTL_DSLPCLKCFG_D_W { w: self }
    }
}
