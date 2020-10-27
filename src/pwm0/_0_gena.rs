#[doc = "Reader of register _0_GENA"]
pub type R = crate::R<u32, super::_0_GENA>;
#[doc = "Writer for register _0_GENA"]
pub type W = crate::W<u32, super::_0_GENA>;
#[doc = "Register _0_GENA `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_GENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Action for Counter=0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_X_GENA_ACTZERO_A {
    #[doc = "0: Do nothing"]
    PWM_X_GENA_ACTZERO_NONE = 0,
    #[doc = "1: Invert pwmA"]
    PWM_X_GENA_ACTZERO_INV = 1,
    #[doc = "2: Drive pwmA Low"]
    PWM_X_GENA_ACTZERO_ZERO = 2,
    #[doc = "3: Drive pwmA High"]
    PWM_X_GENA_ACTZERO_ONE = 3,
}
impl From<PWM_X_GENA_ACTZERO_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_X_GENA_ACTZERO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWM_X_GENA_ACTZERO`"]
pub type PWM_X_GENA_ACTZERO_R = crate::R<u8, PWM_X_GENA_ACTZERO_A>;
impl PWM_X_GENA_ACTZERO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_X_GENA_ACTZERO_A {
        match self.bits {
            0 => PWM_X_GENA_ACTZERO_A::PWM_X_GENA_ACTZERO_NONE,
            1 => PWM_X_GENA_ACTZERO_A::PWM_X_GENA_ACTZERO_INV,
            2 => PWM_X_GENA_ACTZERO_A::PWM_X_GENA_ACTZERO_ZERO,
            3 => PWM_X_GENA_ACTZERO_A::PWM_X_GENA_ACTZERO_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTZERO_NONE`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actzero_none(&self) -> bool {
        *self == PWM_X_GENA_ACTZERO_A::PWM_X_GENA_ACTZERO_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTZERO_INV`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actzero_inv(&self) -> bool {
        *self == PWM_X_GENA_ACTZERO_A::PWM_X_GENA_ACTZERO_INV
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTZERO_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actzero_zero(&self) -> bool {
        *self == PWM_X_GENA_ACTZERO_A::PWM_X_GENA_ACTZERO_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTZERO_ONE`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actzero_one(&self) -> bool {
        *self == PWM_X_GENA_ACTZERO_A::PWM_X_GENA_ACTZERO_ONE
    }
}
#[doc = "Write proxy for field `PWM_X_GENA_ACTZERO`"]
pub struct PWM_X_GENA_ACTZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_GENA_ACTZERO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_X_GENA_ACTZERO_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_x_gena_actzero_none(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTZERO_A::PWM_X_GENA_ACTZERO_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_x_gena_actzero_inv(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTZERO_A::PWM_X_GENA_ACTZERO_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_x_gena_actzero_zero(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTZERO_A::PWM_X_GENA_ACTZERO_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_x_gena_actzero_one(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTZERO_A::PWM_X_GENA_ACTZERO_ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Action for Counter=LOAD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_X_GENA_ACTLOAD_A {
    #[doc = "0: Do nothing"]
    PWM_X_GENA_ACTLOAD_NONE = 0,
    #[doc = "1: Invert pwmA"]
    PWM_X_GENA_ACTLOAD_INV = 1,
    #[doc = "2: Drive pwmA Low"]
    PWM_X_GENA_ACTLOAD_ZERO = 2,
    #[doc = "3: Drive pwmA High"]
    PWM_X_GENA_ACTLOAD_ONE = 3,
}
impl From<PWM_X_GENA_ACTLOAD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_X_GENA_ACTLOAD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWM_X_GENA_ACTLOAD`"]
pub type PWM_X_GENA_ACTLOAD_R = crate::R<u8, PWM_X_GENA_ACTLOAD_A>;
impl PWM_X_GENA_ACTLOAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_X_GENA_ACTLOAD_A {
        match self.bits {
            0 => PWM_X_GENA_ACTLOAD_A::PWM_X_GENA_ACTLOAD_NONE,
            1 => PWM_X_GENA_ACTLOAD_A::PWM_X_GENA_ACTLOAD_INV,
            2 => PWM_X_GENA_ACTLOAD_A::PWM_X_GENA_ACTLOAD_ZERO,
            3 => PWM_X_GENA_ACTLOAD_A::PWM_X_GENA_ACTLOAD_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTLOAD_NONE`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actload_none(&self) -> bool {
        *self == PWM_X_GENA_ACTLOAD_A::PWM_X_GENA_ACTLOAD_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTLOAD_INV`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actload_inv(&self) -> bool {
        *self == PWM_X_GENA_ACTLOAD_A::PWM_X_GENA_ACTLOAD_INV
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTLOAD_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actload_zero(&self) -> bool {
        *self == PWM_X_GENA_ACTLOAD_A::PWM_X_GENA_ACTLOAD_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTLOAD_ONE`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actload_one(&self) -> bool {
        *self == PWM_X_GENA_ACTLOAD_A::PWM_X_GENA_ACTLOAD_ONE
    }
}
#[doc = "Write proxy for field `PWM_X_GENA_ACTLOAD`"]
pub struct PWM_X_GENA_ACTLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_GENA_ACTLOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_X_GENA_ACTLOAD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_x_gena_actload_none(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTLOAD_A::PWM_X_GENA_ACTLOAD_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_x_gena_actload_inv(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTLOAD_A::PWM_X_GENA_ACTLOAD_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_x_gena_actload_zero(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTLOAD_A::PWM_X_GENA_ACTLOAD_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_x_gena_actload_one(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTLOAD_A::PWM_X_GENA_ACTLOAD_ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Action for Comparator A Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_X_GENA_ACTCMPAU_A {
    #[doc = "0: Do nothing"]
    PWM_X_GENA_ACTCMPAU_NONE = 0,
    #[doc = "1: Invert pwmA"]
    PWM_X_GENA_ACTCMPAU_INV = 1,
    #[doc = "2: Drive pwmA Low"]
    PWM_X_GENA_ACTCMPAU_ZERO = 2,
    #[doc = "3: Drive pwmA High"]
    PWM_X_GENA_ACTCMPAU_ONE = 3,
}
impl From<PWM_X_GENA_ACTCMPAU_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_X_GENA_ACTCMPAU_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWM_X_GENA_ACTCMPAU`"]
pub type PWM_X_GENA_ACTCMPAU_R = crate::R<u8, PWM_X_GENA_ACTCMPAU_A>;
impl PWM_X_GENA_ACTCMPAU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_X_GENA_ACTCMPAU_A {
        match self.bits {
            0 => PWM_X_GENA_ACTCMPAU_A::PWM_X_GENA_ACTCMPAU_NONE,
            1 => PWM_X_GENA_ACTCMPAU_A::PWM_X_GENA_ACTCMPAU_INV,
            2 => PWM_X_GENA_ACTCMPAU_A::PWM_X_GENA_ACTCMPAU_ZERO,
            3 => PWM_X_GENA_ACTCMPAU_A::PWM_X_GENA_ACTCMPAU_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPAU_NONE`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpau_none(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPAU_A::PWM_X_GENA_ACTCMPAU_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPAU_INV`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpau_inv(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPAU_A::PWM_X_GENA_ACTCMPAU_INV
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPAU_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpau_zero(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPAU_A::PWM_X_GENA_ACTCMPAU_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPAU_ONE`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpau_one(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPAU_A::PWM_X_GENA_ACTCMPAU_ONE
    }
}
#[doc = "Write proxy for field `PWM_X_GENA_ACTCMPAU`"]
pub struct PWM_X_GENA_ACTCMPAU_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_GENA_ACTCMPAU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_X_GENA_ACTCMPAU_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpau_none(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPAU_A::PWM_X_GENA_ACTCMPAU_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpau_inv(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPAU_A::PWM_X_GENA_ACTCMPAU_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpau_zero(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPAU_A::PWM_X_GENA_ACTCMPAU_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpau_one(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPAU_A::PWM_X_GENA_ACTCMPAU_ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Action for Comparator A Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_X_GENA_ACTCMPAD_A {
    #[doc = "0: Do nothing"]
    PWM_X_GENA_ACTCMPAD_NONE = 0,
    #[doc = "1: Invert pwmA"]
    PWM_X_GENA_ACTCMPAD_INV = 1,
    #[doc = "2: Drive pwmA Low"]
    PWM_X_GENA_ACTCMPAD_ZERO = 2,
    #[doc = "3: Drive pwmA High"]
    PWM_X_GENA_ACTCMPAD_ONE = 3,
}
impl From<PWM_X_GENA_ACTCMPAD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_X_GENA_ACTCMPAD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWM_X_GENA_ACTCMPAD`"]
pub type PWM_X_GENA_ACTCMPAD_R = crate::R<u8, PWM_X_GENA_ACTCMPAD_A>;
impl PWM_X_GENA_ACTCMPAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_X_GENA_ACTCMPAD_A {
        match self.bits {
            0 => PWM_X_GENA_ACTCMPAD_A::PWM_X_GENA_ACTCMPAD_NONE,
            1 => PWM_X_GENA_ACTCMPAD_A::PWM_X_GENA_ACTCMPAD_INV,
            2 => PWM_X_GENA_ACTCMPAD_A::PWM_X_GENA_ACTCMPAD_ZERO,
            3 => PWM_X_GENA_ACTCMPAD_A::PWM_X_GENA_ACTCMPAD_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPAD_NONE`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpad_none(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPAD_A::PWM_X_GENA_ACTCMPAD_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPAD_INV`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpad_inv(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPAD_A::PWM_X_GENA_ACTCMPAD_INV
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPAD_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpad_zero(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPAD_A::PWM_X_GENA_ACTCMPAD_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPAD_ONE`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpad_one(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPAD_A::PWM_X_GENA_ACTCMPAD_ONE
    }
}
#[doc = "Write proxy for field `PWM_X_GENA_ACTCMPAD`"]
pub struct PWM_X_GENA_ACTCMPAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_GENA_ACTCMPAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_X_GENA_ACTCMPAD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpad_none(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPAD_A::PWM_X_GENA_ACTCMPAD_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpad_inv(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPAD_A::PWM_X_GENA_ACTCMPAD_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpad_zero(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPAD_A::PWM_X_GENA_ACTCMPAD_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpad_one(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPAD_A::PWM_X_GENA_ACTCMPAD_ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Action for Comparator B Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_X_GENA_ACTCMPBU_A {
    #[doc = "0: Do nothing"]
    PWM_X_GENA_ACTCMPBU_NONE = 0,
    #[doc = "1: Invert pwmA"]
    PWM_X_GENA_ACTCMPBU_INV = 1,
    #[doc = "2: Drive pwmA Low"]
    PWM_X_GENA_ACTCMPBU_ZERO = 2,
    #[doc = "3: Drive pwmA High"]
    PWM_X_GENA_ACTCMPBU_ONE = 3,
}
impl From<PWM_X_GENA_ACTCMPBU_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_X_GENA_ACTCMPBU_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWM_X_GENA_ACTCMPBU`"]
pub type PWM_X_GENA_ACTCMPBU_R = crate::R<u8, PWM_X_GENA_ACTCMPBU_A>;
impl PWM_X_GENA_ACTCMPBU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_X_GENA_ACTCMPBU_A {
        match self.bits {
            0 => PWM_X_GENA_ACTCMPBU_A::PWM_X_GENA_ACTCMPBU_NONE,
            1 => PWM_X_GENA_ACTCMPBU_A::PWM_X_GENA_ACTCMPBU_INV,
            2 => PWM_X_GENA_ACTCMPBU_A::PWM_X_GENA_ACTCMPBU_ZERO,
            3 => PWM_X_GENA_ACTCMPBU_A::PWM_X_GENA_ACTCMPBU_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPBU_NONE`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpbu_none(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPBU_A::PWM_X_GENA_ACTCMPBU_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPBU_INV`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpbu_inv(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPBU_A::PWM_X_GENA_ACTCMPBU_INV
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPBU_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpbu_zero(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPBU_A::PWM_X_GENA_ACTCMPBU_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPBU_ONE`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpbu_one(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPBU_A::PWM_X_GENA_ACTCMPBU_ONE
    }
}
#[doc = "Write proxy for field `PWM_X_GENA_ACTCMPBU`"]
pub struct PWM_X_GENA_ACTCMPBU_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_GENA_ACTCMPBU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_X_GENA_ACTCMPBU_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpbu_none(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPBU_A::PWM_X_GENA_ACTCMPBU_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpbu_inv(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPBU_A::PWM_X_GENA_ACTCMPBU_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpbu_zero(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPBU_A::PWM_X_GENA_ACTCMPBU_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpbu_one(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPBU_A::PWM_X_GENA_ACTCMPBU_ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Action for Comparator B Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_X_GENA_ACTCMPBD_A {
    #[doc = "0: Do nothing"]
    PWM_X_GENA_ACTCMPBD_NONE = 0,
    #[doc = "1: Invert pwmA"]
    PWM_X_GENA_ACTCMPBD_INV = 1,
    #[doc = "2: Drive pwmA Low"]
    PWM_X_GENA_ACTCMPBD_ZERO = 2,
    #[doc = "3: Drive pwmA High"]
    PWM_X_GENA_ACTCMPBD_ONE = 3,
}
impl From<PWM_X_GENA_ACTCMPBD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_X_GENA_ACTCMPBD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWM_X_GENA_ACTCMPBD`"]
pub type PWM_X_GENA_ACTCMPBD_R = crate::R<u8, PWM_X_GENA_ACTCMPBD_A>;
impl PWM_X_GENA_ACTCMPBD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_X_GENA_ACTCMPBD_A {
        match self.bits {
            0 => PWM_X_GENA_ACTCMPBD_A::PWM_X_GENA_ACTCMPBD_NONE,
            1 => PWM_X_GENA_ACTCMPBD_A::PWM_X_GENA_ACTCMPBD_INV,
            2 => PWM_X_GENA_ACTCMPBD_A::PWM_X_GENA_ACTCMPBD_ZERO,
            3 => PWM_X_GENA_ACTCMPBD_A::PWM_X_GENA_ACTCMPBD_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPBD_NONE`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpbd_none(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPBD_A::PWM_X_GENA_ACTCMPBD_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPBD_INV`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpbd_inv(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPBD_A::PWM_X_GENA_ACTCMPBD_INV
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPBD_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpbd_zero(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPBD_A::PWM_X_GENA_ACTCMPBD_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_X_GENA_ACTCMPBD_ONE`"]
    #[inline(always)]
    pub fn is_pwm_x_gena_actcmpbd_one(&self) -> bool {
        *self == PWM_X_GENA_ACTCMPBD_A::PWM_X_GENA_ACTCMPBD_ONE
    }
}
#[doc = "Write proxy for field `PWM_X_GENA_ACTCMPBD`"]
pub struct PWM_X_GENA_ACTCMPBD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_X_GENA_ACTCMPBD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_X_GENA_ACTCMPBD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpbd_none(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPBD_A::PWM_X_GENA_ACTCMPBD_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpbd_inv(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPBD_A::PWM_X_GENA_ACTCMPBD_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpbd_zero(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPBD_A::PWM_X_GENA_ACTCMPBD_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpbd_one(self) -> &'a mut W {
        self.variant(PWM_X_GENA_ACTCMPBD_A::PWM_X_GENA_ACTCMPBD_ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Action for Counter=0"]
    #[inline(always)]
    pub fn pwm_x_gena_actzero(&self) -> PWM_X_GENA_ACTZERO_R {
        PWM_X_GENA_ACTZERO_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Action for Counter=LOAD"]
    #[inline(always)]
    pub fn pwm_x_gena_actload(&self) -> PWM_X_GENA_ACTLOAD_R {
        PWM_X_GENA_ACTLOAD_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Action for Comparator A Up"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpau(&self) -> PWM_X_GENA_ACTCMPAU_R {
        PWM_X_GENA_ACTCMPAU_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Action for Comparator A Down"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpad(&self) -> PWM_X_GENA_ACTCMPAD_R {
        PWM_X_GENA_ACTCMPAD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Action for Comparator B Up"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpbu(&self) -> PWM_X_GENA_ACTCMPBU_R {
        PWM_X_GENA_ACTCMPBU_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Action for Comparator B Down"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpbd(&self) -> PWM_X_GENA_ACTCMPBD_R {
        PWM_X_GENA_ACTCMPBD_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Action for Counter=0"]
    #[inline(always)]
    pub fn pwm_x_gena_actzero(&mut self) -> PWM_X_GENA_ACTZERO_W {
        PWM_X_GENA_ACTZERO_W { w: self }
    }
    #[doc = "Bits 2:3 - Action for Counter=LOAD"]
    #[inline(always)]
    pub fn pwm_x_gena_actload(&mut self) -> PWM_X_GENA_ACTLOAD_W {
        PWM_X_GENA_ACTLOAD_W { w: self }
    }
    #[doc = "Bits 4:5 - Action for Comparator A Up"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpau(&mut self) -> PWM_X_GENA_ACTCMPAU_W {
        PWM_X_GENA_ACTCMPAU_W { w: self }
    }
    #[doc = "Bits 6:7 - Action for Comparator A Down"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpad(&mut self) -> PWM_X_GENA_ACTCMPAD_W {
        PWM_X_GENA_ACTCMPAD_W { w: self }
    }
    #[doc = "Bits 8:9 - Action for Comparator B Up"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpbu(&mut self) -> PWM_X_GENA_ACTCMPBU_W {
        PWM_X_GENA_ACTCMPBU_W { w: self }
    }
    #[doc = "Bits 10:11 - Action for Comparator B Down"]
    #[inline(always)]
    pub fn pwm_x_gena_actcmpbd(&mut self) -> PWM_X_GENA_ACTCMPBD_W {
        PWM_X_GENA_ACTCMPBD_W { w: self }
    }
}
