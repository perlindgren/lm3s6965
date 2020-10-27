#[doc = "Reader of register DC3"]
pub type R = crate::R<u32, super::DC3>;
#[doc = "Writer for register DC3"]
pub type W = crate::W<u32, super::DC3>;
#[doc = "Register DC3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DC3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCTL_DC3_PWM0`"]
pub type SYSCTL_DC3_PWM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_PWM0`"]
pub struct SYSCTL_DC3_PWM0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_PWM0_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC3_PWM1`"]
pub type SYSCTL_DC3_PWM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_PWM1`"]
pub struct SYSCTL_DC3_PWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_PWM1_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC3_PWM2`"]
pub type SYSCTL_DC3_PWM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_PWM2`"]
pub struct SYSCTL_DC3_PWM2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_PWM2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DC3_PWM3`"]
pub type SYSCTL_DC3_PWM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_PWM3`"]
pub struct SYSCTL_DC3_PWM3_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_PWM3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DC3_PWM4`"]
pub type SYSCTL_DC3_PWM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_PWM4`"]
pub struct SYSCTL_DC3_PWM4_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_PWM4_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC3_PWM5`"]
pub type SYSCTL_DC3_PWM5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_PWM5`"]
pub struct SYSCTL_DC3_PWM5_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_PWM5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DC3_C0MINUS`"]
pub type SYSCTL_DC3_C0MINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_C0MINUS`"]
pub struct SYSCTL_DC3_C0MINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_C0MINUS_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC3_C0PLUS`"]
pub type SYSCTL_DC3_C0PLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_C0PLUS`"]
pub struct SYSCTL_DC3_C0PLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_C0PLUS_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC3_C0O`"]
pub type SYSCTL_DC3_C0O_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_C0O`"]
pub struct SYSCTL_DC3_C0O_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_C0O_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DC3_C1MINUS`"]
pub type SYSCTL_DC3_C1MINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_C1MINUS`"]
pub struct SYSCTL_DC3_C1MINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_C1MINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DC3_C1PLUS`"]
pub type SYSCTL_DC3_C1PLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_C1PLUS`"]
pub struct SYSCTL_DC3_C1PLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_C1PLUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DC3_PWMFAULT`"]
pub type SYSCTL_DC3_PWMFAULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_PWMFAULT`"]
pub struct SYSCTL_DC3_PWMFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_PWMFAULT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DC3_CCP0`"]
pub type SYSCTL_DC3_CCP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_CCP0`"]
pub struct SYSCTL_DC3_CCP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_CCP0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DC3_CCP1`"]
pub type SYSCTL_DC3_CCP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_CCP1`"]
pub struct SYSCTL_DC3_CCP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_CCP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DC3_CCP2`"]
pub type SYSCTL_DC3_CCP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_CCP2`"]
pub struct SYSCTL_DC3_CCP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_CCP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DC3_CCP3`"]
pub type SYSCTL_DC3_CCP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_CCP3`"]
pub struct SYSCTL_DC3_CCP3_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_CCP3_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DC3_32KHZ`"]
pub type SYSCTL_DC3_32KHZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DC3_32KHZ`"]
pub struct SYSCTL_DC3_32KHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DC3_32KHZ_W<'a> {
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
    #[doc = "Bit 0 - PWM0 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwm0(&self) -> SYSCTL_DC3_PWM0_R {
        SYSCTL_DC3_PWM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM1 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwm1(&self) -> SYSCTL_DC3_PWM1_R {
        SYSCTL_DC3_PWM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM2 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwm2(&self) -> SYSCTL_DC3_PWM2_R {
        SYSCTL_DC3_PWM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM3 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwm3(&self) -> SYSCTL_DC3_PWM3_R {
        SYSCTL_DC3_PWM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM4 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwm4(&self) -> SYSCTL_DC3_PWM4_R {
        SYSCTL_DC3_PWM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM5 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwm5(&self) -> SYSCTL_DC3_PWM5_R {
        SYSCTL_DC3_PWM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - C0- Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_c0minus(&self) -> SYSCTL_DC3_C0MINUS_R {
        SYSCTL_DC3_C0MINUS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - C0+ Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_c0plus(&self) -> SYSCTL_DC3_C0PLUS_R {
        SYSCTL_DC3_C0PLUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - C0o Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_c0o(&self) -> SYSCTL_DC3_C0O_R {
        SYSCTL_DC3_C0O_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - C1- Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_c1minus(&self) -> SYSCTL_DC3_C1MINUS_R {
        SYSCTL_DC3_C1MINUS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - C1+ Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_c1plus(&self) -> SYSCTL_DC3_C1PLUS_R {
        SYSCTL_DC3_C1PLUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PWM Fault Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwmfault(&self) -> SYSCTL_DC3_PWMFAULT_R {
        SYSCTL_DC3_PWMFAULT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CCP0 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_ccp0(&self) -> SYSCTL_DC3_CCP0_R {
        SYSCTL_DC3_CCP0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CCP1 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_ccp1(&self) -> SYSCTL_DC3_CCP1_R {
        SYSCTL_DC3_CCP1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CCP2 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_ccp2(&self) -> SYSCTL_DC3_CCP2_R {
        SYSCTL_DC3_CCP2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - CCP3 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_ccp3(&self) -> SYSCTL_DC3_CCP3_R {
        SYSCTL_DC3_CCP3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 32KHz Input Clock Available"]
    #[inline(always)]
    pub fn sysctl_dc3_32khz(&self) -> SYSCTL_DC3_32KHZ_R {
        SYSCTL_DC3_32KHZ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwm0(&mut self) -> SYSCTL_DC3_PWM0_W {
        SYSCTL_DC3_PWM0_W { w: self }
    }
    #[doc = "Bit 1 - PWM1 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwm1(&mut self) -> SYSCTL_DC3_PWM1_W {
        SYSCTL_DC3_PWM1_W { w: self }
    }
    #[doc = "Bit 2 - PWM2 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwm2(&mut self) -> SYSCTL_DC3_PWM2_W {
        SYSCTL_DC3_PWM2_W { w: self }
    }
    #[doc = "Bit 3 - PWM3 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwm3(&mut self) -> SYSCTL_DC3_PWM3_W {
        SYSCTL_DC3_PWM3_W { w: self }
    }
    #[doc = "Bit 4 - PWM4 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwm4(&mut self) -> SYSCTL_DC3_PWM4_W {
        SYSCTL_DC3_PWM4_W { w: self }
    }
    #[doc = "Bit 5 - PWM5 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwm5(&mut self) -> SYSCTL_DC3_PWM5_W {
        SYSCTL_DC3_PWM5_W { w: self }
    }
    #[doc = "Bit 6 - C0- Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_c0minus(&mut self) -> SYSCTL_DC3_C0MINUS_W {
        SYSCTL_DC3_C0MINUS_W { w: self }
    }
    #[doc = "Bit 7 - C0+ Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_c0plus(&mut self) -> SYSCTL_DC3_C0PLUS_W {
        SYSCTL_DC3_C0PLUS_W { w: self }
    }
    #[doc = "Bit 8 - C0o Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_c0o(&mut self) -> SYSCTL_DC3_C0O_W {
        SYSCTL_DC3_C0O_W { w: self }
    }
    #[doc = "Bit 9 - C1- Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_c1minus(&mut self) -> SYSCTL_DC3_C1MINUS_W {
        SYSCTL_DC3_C1MINUS_W { w: self }
    }
    #[doc = "Bit 10 - C1+ Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_c1plus(&mut self) -> SYSCTL_DC3_C1PLUS_W {
        SYSCTL_DC3_C1PLUS_W { w: self }
    }
    #[doc = "Bit 15 - PWM Fault Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_pwmfault(&mut self) -> SYSCTL_DC3_PWMFAULT_W {
        SYSCTL_DC3_PWMFAULT_W { w: self }
    }
    #[doc = "Bit 24 - CCP0 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_ccp0(&mut self) -> SYSCTL_DC3_CCP0_W {
        SYSCTL_DC3_CCP0_W { w: self }
    }
    #[doc = "Bit 25 - CCP1 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_ccp1(&mut self) -> SYSCTL_DC3_CCP1_W {
        SYSCTL_DC3_CCP1_W { w: self }
    }
    #[doc = "Bit 26 - CCP2 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_ccp2(&mut self) -> SYSCTL_DC3_CCP2_W {
        SYSCTL_DC3_CCP2_W { w: self }
    }
    #[doc = "Bit 27 - CCP3 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc3_ccp3(&mut self) -> SYSCTL_DC3_CCP3_W {
        SYSCTL_DC3_CCP3_W { w: self }
    }
    #[doc = "Bit 31 - 32KHz Input Clock Available"]
    #[inline(always)]
    pub fn sysctl_dc3_32khz(&mut self) -> SYSCTL_DC3_32KHZ_W {
        SYSCTL_DC3_32KHZ_W { w: self }
    }
}
