#[doc = "Reader of register DCGC1"]
pub type R = crate::R<u32, super::DCGC1>;
#[doc = "Writer for register DCGC1"]
pub type W = crate::W<u32, super::DCGC1>;
#[doc = "Register DCGC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DCGC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCTL_DCGC1_UART0`"]
pub type SYSCTL_DCGC1_UART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_UART0`"]
pub struct SYSCTL_DCGC1_UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_UART0_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC1_UART1`"]
pub type SYSCTL_DCGC1_UART1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_UART1`"]
pub struct SYSCTL_DCGC1_UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_UART1_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC1_UART2`"]
pub type SYSCTL_DCGC1_UART2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_UART2`"]
pub struct SYSCTL_DCGC1_UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_UART2_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC1_SSI0`"]
pub type SYSCTL_DCGC1_SSI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_SSI0`"]
pub struct SYSCTL_DCGC1_SSI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_SSI0_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC1_QEI0`"]
pub type SYSCTL_DCGC1_QEI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_QEI0`"]
pub struct SYSCTL_DCGC1_QEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_QEI0_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC1_QEI1`"]
pub type SYSCTL_DCGC1_QEI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_QEI1`"]
pub struct SYSCTL_DCGC1_QEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_QEI1_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC1_I2C0`"]
pub type SYSCTL_DCGC1_I2C0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_I2C0`"]
pub struct SYSCTL_DCGC1_I2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_I2C0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DCGC1_I2C1`"]
pub type SYSCTL_DCGC1_I2C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_I2C1`"]
pub struct SYSCTL_DCGC1_I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_I2C1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DCGC1_TIMER0`"]
pub type SYSCTL_DCGC1_TIMER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_TIMER0`"]
pub struct SYSCTL_DCGC1_TIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_TIMER0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DCGC1_TIMER1`"]
pub type SYSCTL_DCGC1_TIMER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_TIMER1`"]
pub struct SYSCTL_DCGC1_TIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_TIMER1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DCGC1_TIMER2`"]
pub type SYSCTL_DCGC1_TIMER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_TIMER2`"]
pub struct SYSCTL_DCGC1_TIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_TIMER2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DCGC1_TIMER3`"]
pub type SYSCTL_DCGC1_TIMER3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_TIMER3`"]
pub struct SYSCTL_DCGC1_TIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_TIMER3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `SYSCTL_DCGC1_COMP0`"]
pub type SYSCTL_DCGC1_COMP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_COMP0`"]
pub struct SYSCTL_DCGC1_COMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_COMP0_W<'a> {
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
#[doc = "Reader of field `SYSCTL_DCGC1_COMP1`"]
pub type SYSCTL_DCGC1_COMP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCTL_DCGC1_COMP1`"]
pub struct SYSCTL_DCGC1_COMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_DCGC1_COMP1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - UART0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_uart0(&self) -> SYSCTL_DCGC1_UART0_R {
        SYSCTL_DCGC1_UART0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_uart1(&self) -> SYSCTL_DCGC1_UART1_R {
        SYSCTL_DCGC1_UART1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART2 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_uart2(&self) -> SYSCTL_DCGC1_UART2_R {
        SYSCTL_DCGC1_UART2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SSI0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_ssi0(&self) -> SYSCTL_DCGC1_SSI0_R {
        SYSCTL_DCGC1_SSI0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - QEI0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_qei0(&self) -> SYSCTL_DCGC1_QEI0_R {
        SYSCTL_DCGC1_QEI0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - QEI1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_qei1(&self) -> SYSCTL_DCGC1_QEI1_R {
        SYSCTL_DCGC1_QEI1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - I2C0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_i2c0(&self) -> SYSCTL_DCGC1_I2C0_R {
        SYSCTL_DCGC1_I2C0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - I2C1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_i2c1(&self) -> SYSCTL_DCGC1_I2C1_R {
        SYSCTL_DCGC1_I2C1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timer 0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_timer0(&self) -> SYSCTL_DCGC1_TIMER0_R {
        SYSCTL_DCGC1_TIMER0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer 1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_timer1(&self) -> SYSCTL_DCGC1_TIMER1_R {
        SYSCTL_DCGC1_TIMER1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timer 2 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_timer2(&self) -> SYSCTL_DCGC1_TIMER2_R {
        SYSCTL_DCGC1_TIMER2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timer 3 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_timer3(&self) -> SYSCTL_DCGC1_TIMER3_R {
        SYSCTL_DCGC1_TIMER3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Analog Comparator 0 Clock Gating"]
    #[inline(always)]
    pub fn sysctl_dcgc1_comp0(&self) -> SYSCTL_DCGC1_COMP0_R {
        SYSCTL_DCGC1_COMP0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Analog Comparator 1 Clock Gating"]
    #[inline(always)]
    pub fn sysctl_dcgc1_comp1(&self) -> SYSCTL_DCGC1_COMP1_R {
        SYSCTL_DCGC1_COMP1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_uart0(&mut self) -> SYSCTL_DCGC1_UART0_W {
        SYSCTL_DCGC1_UART0_W { w: self }
    }
    #[doc = "Bit 1 - UART1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_uart1(&mut self) -> SYSCTL_DCGC1_UART1_W {
        SYSCTL_DCGC1_UART1_W { w: self }
    }
    #[doc = "Bit 2 - UART2 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_uart2(&mut self) -> SYSCTL_DCGC1_UART2_W {
        SYSCTL_DCGC1_UART2_W { w: self }
    }
    #[doc = "Bit 4 - SSI0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_ssi0(&mut self) -> SYSCTL_DCGC1_SSI0_W {
        SYSCTL_DCGC1_SSI0_W { w: self }
    }
    #[doc = "Bit 8 - QEI0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_qei0(&mut self) -> SYSCTL_DCGC1_QEI0_W {
        SYSCTL_DCGC1_QEI0_W { w: self }
    }
    #[doc = "Bit 9 - QEI1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_qei1(&mut self) -> SYSCTL_DCGC1_QEI1_W {
        SYSCTL_DCGC1_QEI1_W { w: self }
    }
    #[doc = "Bit 12 - I2C0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_i2c0(&mut self) -> SYSCTL_DCGC1_I2C0_W {
        SYSCTL_DCGC1_I2C0_W { w: self }
    }
    #[doc = "Bit 14 - I2C1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_i2c1(&mut self) -> SYSCTL_DCGC1_I2C1_W {
        SYSCTL_DCGC1_I2C1_W { w: self }
    }
    #[doc = "Bit 16 - Timer 0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_timer0(&mut self) -> SYSCTL_DCGC1_TIMER0_W {
        SYSCTL_DCGC1_TIMER0_W { w: self }
    }
    #[doc = "Bit 17 - Timer 1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_timer1(&mut self) -> SYSCTL_DCGC1_TIMER1_W {
        SYSCTL_DCGC1_TIMER1_W { w: self }
    }
    #[doc = "Bit 18 - Timer 2 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_timer2(&mut self) -> SYSCTL_DCGC1_TIMER2_W {
        SYSCTL_DCGC1_TIMER2_W { w: self }
    }
    #[doc = "Bit 19 - Timer 3 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc1_timer3(&mut self) -> SYSCTL_DCGC1_TIMER3_W {
        SYSCTL_DCGC1_TIMER3_W { w: self }
    }
    #[doc = "Bit 24 - Analog Comparator 0 Clock Gating"]
    #[inline(always)]
    pub fn sysctl_dcgc1_comp0(&mut self) -> SYSCTL_DCGC1_COMP0_W {
        SYSCTL_DCGC1_COMP0_W { w: self }
    }
    #[doc = "Bit 25 - Analog Comparator 1 Clock Gating"]
    #[inline(always)]
    pub fn sysctl_dcgc1_comp1(&mut self) -> SYSCTL_DCGC1_COMP1_W {
        SYSCTL_DCGC1_COMP1_W { w: self }
    }
}
