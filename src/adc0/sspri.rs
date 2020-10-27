#[doc = "Reader of register SSPRI"]
pub type R = crate::R<u32, super::SSPRI>;
#[doc = "Writer for register SSPRI"]
pub type W = crate::W<u32, super::SSPRI>;
#[doc = "Register SSPRI `reset()`'s with value 0"]
impl crate::ResetValue for super::SSPRI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SS0 Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_SSPRI_SS0_A {
    #[doc = "0: First priority"]
    ADC_SSPRI_SS0_1ST = 0,
    #[doc = "1: Second priority"]
    ADC_SSPRI_SS0_2ND = 1,
    #[doc = "2: Third priority"]
    ADC_SSPRI_SS0_3RD = 2,
    #[doc = "3: Fourth priority"]
    ADC_SSPRI_SS0_4TH = 3,
}
impl From<ADC_SSPRI_SS0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_SSPRI_SS0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_SSPRI_SS0`"]
pub type ADC_SSPRI_SS0_R = crate::R<u8, ADC_SSPRI_SS0_A>;
impl ADC_SSPRI_SS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_SSPRI_SS0_A {
        match self.bits {
            0 => ADC_SSPRI_SS0_A::ADC_SSPRI_SS0_1ST,
            1 => ADC_SSPRI_SS0_A::ADC_SSPRI_SS0_2ND,
            2 => ADC_SSPRI_SS0_A::ADC_SSPRI_SS0_3RD,
            3 => ADC_SSPRI_SS0_A::ADC_SSPRI_SS0_4TH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS0_1ST`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss0_1st(&self) -> bool {
        *self == ADC_SSPRI_SS0_A::ADC_SSPRI_SS0_1ST
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS0_2ND`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss0_2nd(&self) -> bool {
        *self == ADC_SSPRI_SS0_A::ADC_SSPRI_SS0_2ND
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS0_3RD`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss0_3rd(&self) -> bool {
        *self == ADC_SSPRI_SS0_A::ADC_SSPRI_SS0_3RD
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS0_4TH`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss0_4th(&self) -> bool {
        *self == ADC_SSPRI_SS0_A::ADC_SSPRI_SS0_4TH
    }
}
#[doc = "Write proxy for field `ADC_SSPRI_SS0`"]
pub struct ADC_SSPRI_SS0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSPRI_SS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_SSPRI_SS0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "First priority"]
    #[inline(always)]
    pub fn adc_sspri_ss0_1st(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS0_A::ADC_SSPRI_SS0_1ST)
    }
    #[doc = "Second priority"]
    #[inline(always)]
    pub fn adc_sspri_ss0_2nd(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS0_A::ADC_SSPRI_SS0_2ND)
    }
    #[doc = "Third priority"]
    #[inline(always)]
    pub fn adc_sspri_ss0_3rd(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS0_A::ADC_SSPRI_SS0_3RD)
    }
    #[doc = "Fourth priority"]
    #[inline(always)]
    pub fn adc_sspri_ss0_4th(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS0_A::ADC_SSPRI_SS0_4TH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "SS1 Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_SSPRI_SS1_A {
    #[doc = "0: First priority"]
    ADC_SSPRI_SS1_1ST = 0,
    #[doc = "1: Second priority"]
    ADC_SSPRI_SS1_2ND = 1,
    #[doc = "2: Third priority"]
    ADC_SSPRI_SS1_3RD = 2,
    #[doc = "3: Fourth priority"]
    ADC_SSPRI_SS1_4TH = 3,
}
impl From<ADC_SSPRI_SS1_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_SSPRI_SS1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_SSPRI_SS1`"]
pub type ADC_SSPRI_SS1_R = crate::R<u8, ADC_SSPRI_SS1_A>;
impl ADC_SSPRI_SS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_SSPRI_SS1_A {
        match self.bits {
            0 => ADC_SSPRI_SS1_A::ADC_SSPRI_SS1_1ST,
            1 => ADC_SSPRI_SS1_A::ADC_SSPRI_SS1_2ND,
            2 => ADC_SSPRI_SS1_A::ADC_SSPRI_SS1_3RD,
            3 => ADC_SSPRI_SS1_A::ADC_SSPRI_SS1_4TH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS1_1ST`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss1_1st(&self) -> bool {
        *self == ADC_SSPRI_SS1_A::ADC_SSPRI_SS1_1ST
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS1_2ND`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss1_2nd(&self) -> bool {
        *self == ADC_SSPRI_SS1_A::ADC_SSPRI_SS1_2ND
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS1_3RD`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss1_3rd(&self) -> bool {
        *self == ADC_SSPRI_SS1_A::ADC_SSPRI_SS1_3RD
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS1_4TH`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss1_4th(&self) -> bool {
        *self == ADC_SSPRI_SS1_A::ADC_SSPRI_SS1_4TH
    }
}
#[doc = "Write proxy for field `ADC_SSPRI_SS1`"]
pub struct ADC_SSPRI_SS1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSPRI_SS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_SSPRI_SS1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "First priority"]
    #[inline(always)]
    pub fn adc_sspri_ss1_1st(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS1_A::ADC_SSPRI_SS1_1ST)
    }
    #[doc = "Second priority"]
    #[inline(always)]
    pub fn adc_sspri_ss1_2nd(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS1_A::ADC_SSPRI_SS1_2ND)
    }
    #[doc = "Third priority"]
    #[inline(always)]
    pub fn adc_sspri_ss1_3rd(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS1_A::ADC_SSPRI_SS1_3RD)
    }
    #[doc = "Fourth priority"]
    #[inline(always)]
    pub fn adc_sspri_ss1_4th(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS1_A::ADC_SSPRI_SS1_4TH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "SS2 Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_SSPRI_SS2_A {
    #[doc = "0: First priority"]
    ADC_SSPRI_SS2_1ST = 0,
    #[doc = "1: Second priority"]
    ADC_SSPRI_SS2_2ND = 1,
    #[doc = "2: Third priority"]
    ADC_SSPRI_SS2_3RD = 2,
    #[doc = "3: Fourth priority"]
    ADC_SSPRI_SS2_4TH = 3,
}
impl From<ADC_SSPRI_SS2_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_SSPRI_SS2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_SSPRI_SS2`"]
pub type ADC_SSPRI_SS2_R = crate::R<u8, ADC_SSPRI_SS2_A>;
impl ADC_SSPRI_SS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_SSPRI_SS2_A {
        match self.bits {
            0 => ADC_SSPRI_SS2_A::ADC_SSPRI_SS2_1ST,
            1 => ADC_SSPRI_SS2_A::ADC_SSPRI_SS2_2ND,
            2 => ADC_SSPRI_SS2_A::ADC_SSPRI_SS2_3RD,
            3 => ADC_SSPRI_SS2_A::ADC_SSPRI_SS2_4TH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS2_1ST`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss2_1st(&self) -> bool {
        *self == ADC_SSPRI_SS2_A::ADC_SSPRI_SS2_1ST
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS2_2ND`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss2_2nd(&self) -> bool {
        *self == ADC_SSPRI_SS2_A::ADC_SSPRI_SS2_2ND
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS2_3RD`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss2_3rd(&self) -> bool {
        *self == ADC_SSPRI_SS2_A::ADC_SSPRI_SS2_3RD
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS2_4TH`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss2_4th(&self) -> bool {
        *self == ADC_SSPRI_SS2_A::ADC_SSPRI_SS2_4TH
    }
}
#[doc = "Write proxy for field `ADC_SSPRI_SS2`"]
pub struct ADC_SSPRI_SS2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSPRI_SS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_SSPRI_SS2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "First priority"]
    #[inline(always)]
    pub fn adc_sspri_ss2_1st(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS2_A::ADC_SSPRI_SS2_1ST)
    }
    #[doc = "Second priority"]
    #[inline(always)]
    pub fn adc_sspri_ss2_2nd(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS2_A::ADC_SSPRI_SS2_2ND)
    }
    #[doc = "Third priority"]
    #[inline(always)]
    pub fn adc_sspri_ss2_3rd(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS2_A::ADC_SSPRI_SS2_3RD)
    }
    #[doc = "Fourth priority"]
    #[inline(always)]
    pub fn adc_sspri_ss2_4th(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS2_A::ADC_SSPRI_SS2_4TH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "SS3 Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_SSPRI_SS3_A {
    #[doc = "0: First priority"]
    ADC_SSPRI_SS3_1ST = 0,
    #[doc = "1: Second priority"]
    ADC_SSPRI_SS3_2ND = 1,
    #[doc = "2: Third priority"]
    ADC_SSPRI_SS3_3RD = 2,
    #[doc = "3: Fourth priority"]
    ADC_SSPRI_SS3_4TH = 3,
}
impl From<ADC_SSPRI_SS3_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_SSPRI_SS3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_SSPRI_SS3`"]
pub type ADC_SSPRI_SS3_R = crate::R<u8, ADC_SSPRI_SS3_A>;
impl ADC_SSPRI_SS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_SSPRI_SS3_A {
        match self.bits {
            0 => ADC_SSPRI_SS3_A::ADC_SSPRI_SS3_1ST,
            1 => ADC_SSPRI_SS3_A::ADC_SSPRI_SS3_2ND,
            2 => ADC_SSPRI_SS3_A::ADC_SSPRI_SS3_3RD,
            3 => ADC_SSPRI_SS3_A::ADC_SSPRI_SS3_4TH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS3_1ST`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss3_1st(&self) -> bool {
        *self == ADC_SSPRI_SS3_A::ADC_SSPRI_SS3_1ST
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS3_2ND`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss3_2nd(&self) -> bool {
        *self == ADC_SSPRI_SS3_A::ADC_SSPRI_SS3_2ND
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS3_3RD`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss3_3rd(&self) -> bool {
        *self == ADC_SSPRI_SS3_A::ADC_SSPRI_SS3_3RD
    }
    #[doc = "Checks if the value of the field is `ADC_SSPRI_SS3_4TH`"]
    #[inline(always)]
    pub fn is_adc_sspri_ss3_4th(&self) -> bool {
        *self == ADC_SSPRI_SS3_A::ADC_SSPRI_SS3_4TH
    }
}
#[doc = "Write proxy for field `ADC_SSPRI_SS3`"]
pub struct ADC_SSPRI_SS3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSPRI_SS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_SSPRI_SS3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "First priority"]
    #[inline(always)]
    pub fn adc_sspri_ss3_1st(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS3_A::ADC_SSPRI_SS3_1ST)
    }
    #[doc = "Second priority"]
    #[inline(always)]
    pub fn adc_sspri_ss3_2nd(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS3_A::ADC_SSPRI_SS3_2ND)
    }
    #[doc = "Third priority"]
    #[inline(always)]
    pub fn adc_sspri_ss3_3rd(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS3_A::ADC_SSPRI_SS3_3RD)
    }
    #[doc = "Fourth priority"]
    #[inline(always)]
    pub fn adc_sspri_ss3_4th(self) -> &'a mut W {
        self.variant(ADC_SSPRI_SS3_A::ADC_SSPRI_SS3_4TH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SS0 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss0(&self) -> ADC_SSPRI_SS0_R {
        ADC_SSPRI_SS0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - SS1 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss1(&self) -> ADC_SSPRI_SS1_R {
        ADC_SSPRI_SS1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - SS2 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss2(&self) -> ADC_SSPRI_SS2_R {
        ADC_SSPRI_SS2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - SS3 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss3(&self) -> ADC_SSPRI_SS3_R {
        ADC_SSPRI_SS3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SS0 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss0(&mut self) -> ADC_SSPRI_SS0_W {
        ADC_SSPRI_SS0_W { w: self }
    }
    #[doc = "Bits 4:5 - SS1 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss1(&mut self) -> ADC_SSPRI_SS1_W {
        ADC_SSPRI_SS1_W { w: self }
    }
    #[doc = "Bits 8:9 - SS2 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss2(&mut self) -> ADC_SSPRI_SS2_W {
        ADC_SSPRI_SS2_W { w: self }
    }
    #[doc = "Bits 12:13 - SS3 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss3(&mut self) -> ADC_SSPRI_SS3_W {
        ADC_SSPRI_SS3_W { w: self }
    }
}
