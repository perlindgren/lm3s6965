#[doc = "Reader of register EMUX"]
pub type R = crate::R<u32, super::EMUX>;
#[doc = "Writer for register EMUX"]
pub type W = crate::W<u32, super::EMUX>;
#[doc = "Register EMUX `reset()`'s with value 0"]
impl crate::ResetValue for super::EMUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SS0 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_EMUX_EM0_A {
    #[doc = "0: Processor (default)"]
    ADC_EMUX_EM0_PROCESSOR = 0,
    #[doc = "1: Analog Comparator 0"]
    ADC_EMUX_EM0_COMP0 = 1,
    #[doc = "2: Analog Comparator 1"]
    ADC_EMUX_EM0_COMP1 = 2,
    #[doc = "4: External (GPIO PB4)"]
    ADC_EMUX_EM0_EXTERNAL = 4,
    #[doc = "5: Timer"]
    ADC_EMUX_EM0_TIMER = 5,
    #[doc = "6: PWM0"]
    ADC_EMUX_EM0_PWM0 = 6,
    #[doc = "7: PWM1"]
    ADC_EMUX_EM0_PWM1 = 7,
    #[doc = "8: PWM2"]
    ADC_EMUX_EM0_PWM2 = 8,
    #[doc = "15: Always (continuously sample)"]
    ADC_EMUX_EM0_ALWAYS = 15,
}
impl From<ADC_EMUX_EM0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_EMUX_EM0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_EMUX_EM0`"]
pub type ADC_EMUX_EM0_R = crate::R<u8, ADC_EMUX_EM0_A>;
impl ADC_EMUX_EM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADC_EMUX_EM0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PROCESSOR),
            1 => Val(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP0),
            2 => Val(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP1),
            4 => Val(ADC_EMUX_EM0_A::ADC_EMUX_EM0_EXTERNAL),
            5 => Val(ADC_EMUX_EM0_A::ADC_EMUX_EM0_TIMER),
            6 => Val(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM0),
            7 => Val(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM1),
            8 => Val(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM2),
            15 => Val(ADC_EMUX_EM0_A::ADC_EMUX_EM0_ALWAYS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PROCESSOR`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_processor(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PROCESSOR
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_COMP0`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_comp0(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_COMP1`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_comp1(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_EXTERNAL`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_external(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_TIMER`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_timer(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_TIMER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PWM0`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PWM1`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PWM2`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_always(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_ALWAYS
    }
}
#[doc = "Write proxy for field `ADC_EMUX_EM0`"]
pub struct ADC_EMUX_EM0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_EMUX_EM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_EMUX_EM0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em0_processor(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em0_comp0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em0_comp1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP1)
    }
    #[doc = "External (GPIO PB4)"]
    #[inline(always)]
    pub fn adc_emux_em0_external(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em0_timer(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_TIMER)
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM0)
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM1)
    }
    #[doc = "PWM2"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM2)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em0_always(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "SS1 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_EMUX_EM1_A {
    #[doc = "0: Processor (default)"]
    ADC_EMUX_EM1_PROCESSOR = 0,
    #[doc = "1: Analog Comparator 0"]
    ADC_EMUX_EM1_COMP0 = 1,
    #[doc = "2: Analog Comparator 1"]
    ADC_EMUX_EM1_COMP1 = 2,
    #[doc = "4: External (GPIO PB4)"]
    ADC_EMUX_EM1_EXTERNAL = 4,
    #[doc = "5: Timer"]
    ADC_EMUX_EM1_TIMER = 5,
    #[doc = "6: PWM0"]
    ADC_EMUX_EM1_PWM0 = 6,
    #[doc = "7: PWM1"]
    ADC_EMUX_EM1_PWM1 = 7,
    #[doc = "8: PWM2"]
    ADC_EMUX_EM1_PWM2 = 8,
    #[doc = "15: Always (continuously sample)"]
    ADC_EMUX_EM1_ALWAYS = 15,
}
impl From<ADC_EMUX_EM1_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_EMUX_EM1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_EMUX_EM1`"]
pub type ADC_EMUX_EM1_R = crate::R<u8, ADC_EMUX_EM1_A>;
impl ADC_EMUX_EM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADC_EMUX_EM1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PROCESSOR),
            1 => Val(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP0),
            2 => Val(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP1),
            4 => Val(ADC_EMUX_EM1_A::ADC_EMUX_EM1_EXTERNAL),
            5 => Val(ADC_EMUX_EM1_A::ADC_EMUX_EM1_TIMER),
            6 => Val(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM0),
            7 => Val(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM1),
            8 => Val(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM2),
            15 => Val(ADC_EMUX_EM1_A::ADC_EMUX_EM1_ALWAYS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PROCESSOR`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_processor(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PROCESSOR
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_COMP0`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_comp0(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_COMP1`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_comp1(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_EXTERNAL`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_external(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_TIMER`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_timer(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_TIMER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PWM0`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PWM1`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PWM2`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_always(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_ALWAYS
    }
}
#[doc = "Write proxy for field `ADC_EMUX_EM1`"]
pub struct ADC_EMUX_EM1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_EMUX_EM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_EMUX_EM1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em1_processor(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em1_comp0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em1_comp1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP1)
    }
    #[doc = "External (GPIO PB4)"]
    #[inline(always)]
    pub fn adc_emux_em1_external(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em1_timer(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_TIMER)
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM0)
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM1)
    }
    #[doc = "PWM2"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM2)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em1_always(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "SS2 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_EMUX_EM2_A {
    #[doc = "0: Processor (default)"]
    ADC_EMUX_EM2_PROCESSOR = 0,
    #[doc = "1: Analog Comparator 0"]
    ADC_EMUX_EM2_COMP0 = 1,
    #[doc = "2: Analog Comparator 1"]
    ADC_EMUX_EM2_COMP1 = 2,
    #[doc = "4: External (GPIO PB4)"]
    ADC_EMUX_EM2_EXTERNAL = 4,
    #[doc = "5: Timer"]
    ADC_EMUX_EM2_TIMER = 5,
    #[doc = "6: PWM0"]
    ADC_EMUX_EM2_PWM0 = 6,
    #[doc = "7: PWM1"]
    ADC_EMUX_EM2_PWM1 = 7,
    #[doc = "8: PWM2"]
    ADC_EMUX_EM2_PWM2 = 8,
    #[doc = "15: Always (continuously sample)"]
    ADC_EMUX_EM2_ALWAYS = 15,
}
impl From<ADC_EMUX_EM2_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_EMUX_EM2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_EMUX_EM2`"]
pub type ADC_EMUX_EM2_R = crate::R<u8, ADC_EMUX_EM2_A>;
impl ADC_EMUX_EM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADC_EMUX_EM2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PROCESSOR),
            1 => Val(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP0),
            2 => Val(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP1),
            4 => Val(ADC_EMUX_EM2_A::ADC_EMUX_EM2_EXTERNAL),
            5 => Val(ADC_EMUX_EM2_A::ADC_EMUX_EM2_TIMER),
            6 => Val(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM0),
            7 => Val(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM1),
            8 => Val(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM2),
            15 => Val(ADC_EMUX_EM2_A::ADC_EMUX_EM2_ALWAYS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PROCESSOR`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_processor(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PROCESSOR
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_COMP0`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_comp0(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_COMP1`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_comp1(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_EXTERNAL`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_external(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_TIMER`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_timer(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_TIMER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PWM0`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PWM1`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PWM2`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_always(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_ALWAYS
    }
}
#[doc = "Write proxy for field `ADC_EMUX_EM2`"]
pub struct ADC_EMUX_EM2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_EMUX_EM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_EMUX_EM2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em2_processor(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em2_comp0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em2_comp1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP1)
    }
    #[doc = "External (GPIO PB4)"]
    #[inline(always)]
    pub fn adc_emux_em2_external(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em2_timer(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_TIMER)
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM0)
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM1)
    }
    #[doc = "PWM2"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM2)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em2_always(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "SS3 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_EMUX_EM3_A {
    #[doc = "0: Processor (default)"]
    ADC_EMUX_EM3_PROCESSOR = 0,
    #[doc = "1: Analog Comparator 0"]
    ADC_EMUX_EM3_COMP0 = 1,
    #[doc = "2: Analog Comparator 1"]
    ADC_EMUX_EM3_COMP1 = 2,
    #[doc = "4: External (GPIO PB4)"]
    ADC_EMUX_EM3_EXTERNAL = 4,
    #[doc = "5: Timer"]
    ADC_EMUX_EM3_TIMER = 5,
    #[doc = "6: PWM0"]
    ADC_EMUX_EM3_PWM0 = 6,
    #[doc = "7: PWM1"]
    ADC_EMUX_EM3_PWM1 = 7,
    #[doc = "8: PWM2"]
    ADC_EMUX_EM3_PWM2 = 8,
    #[doc = "15: Always (continuously sample)"]
    ADC_EMUX_EM3_ALWAYS = 15,
}
impl From<ADC_EMUX_EM3_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_EMUX_EM3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_EMUX_EM3`"]
pub type ADC_EMUX_EM3_R = crate::R<u8, ADC_EMUX_EM3_A>;
impl ADC_EMUX_EM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADC_EMUX_EM3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PROCESSOR),
            1 => Val(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP0),
            2 => Val(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP1),
            4 => Val(ADC_EMUX_EM3_A::ADC_EMUX_EM3_EXTERNAL),
            5 => Val(ADC_EMUX_EM3_A::ADC_EMUX_EM3_TIMER),
            6 => Val(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM0),
            7 => Val(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM1),
            8 => Val(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM2),
            15 => Val(ADC_EMUX_EM3_A::ADC_EMUX_EM3_ALWAYS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PROCESSOR`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_processor(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PROCESSOR
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_COMP0`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_comp0(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_COMP1`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_comp1(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_EXTERNAL`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_external(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_TIMER`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_timer(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_TIMER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PWM0`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PWM1`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PWM2`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_always(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_ALWAYS
    }
}
#[doc = "Write proxy for field `ADC_EMUX_EM3`"]
pub struct ADC_EMUX_EM3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_EMUX_EM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_EMUX_EM3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em3_processor(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em3_comp0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em3_comp1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP1)
    }
    #[doc = "External (GPIO PB4)"]
    #[inline(always)]
    pub fn adc_emux_em3_external(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em3_timer(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_TIMER)
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM0)
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM1)
    }
    #[doc = "PWM2"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM2)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em3_always(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - SS0 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em0(&self) -> ADC_EMUX_EM0_R {
        ADC_EMUX_EM0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SS1 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em1(&self) -> ADC_EMUX_EM1_R {
        ADC_EMUX_EM1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SS2 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em2(&self) -> ADC_EMUX_EM2_R {
        ADC_EMUX_EM2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SS3 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em3(&self) -> ADC_EMUX_EM3_R {
        ADC_EMUX_EM3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SS0 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em0(&mut self) -> ADC_EMUX_EM0_W {
        ADC_EMUX_EM0_W { w: self }
    }
    #[doc = "Bits 4:7 - SS1 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em1(&mut self) -> ADC_EMUX_EM1_W {
        ADC_EMUX_EM1_W { w: self }
    }
    #[doc = "Bits 8:11 - SS2 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em2(&mut self) -> ADC_EMUX_EM2_W {
        ADC_EMUX_EM2_W { w: self }
    }
    #[doc = "Bits 12:15 - SS3 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em3(&mut self) -> ADC_EMUX_EM3_W {
        ADC_EMUX_EM3_W { w: self }
    }
}
