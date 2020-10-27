#[doc = "Reader of register SSMUX0"]
pub type R = crate::R<u32, super::SSMUX0>;
#[doc = "Writer for register SSMUX0"]
pub type W = crate::W<u32, super::SSMUX0>;
#[doc = "Register SSMUX0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSMUX0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_SSMUX0_MUX0`"]
pub type ADC_SSMUX0_MUX0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_SSMUX0_MUX0`"]
pub struct ADC_SSMUX0_MUX0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSMUX0_MUX0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSMUX0_MUX1`"]
pub type ADC_SSMUX0_MUX1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_SSMUX0_MUX1`"]
pub struct ADC_SSMUX0_MUX1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSMUX0_MUX1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSMUX0_MUX2`"]
pub type ADC_SSMUX0_MUX2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_SSMUX0_MUX2`"]
pub struct ADC_SSMUX0_MUX2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSMUX0_MUX2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSMUX0_MUX3`"]
pub type ADC_SSMUX0_MUX3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_SSMUX0_MUX3`"]
pub struct ADC_SSMUX0_MUX3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSMUX0_MUX3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSMUX0_MUX4`"]
pub type ADC_SSMUX0_MUX4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_SSMUX0_MUX4`"]
pub struct ADC_SSMUX0_MUX4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSMUX0_MUX4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSMUX0_MUX5`"]
pub type ADC_SSMUX0_MUX5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_SSMUX0_MUX5`"]
pub struct ADC_SSMUX0_MUX5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSMUX0_MUX5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSMUX0_MUX6`"]
pub type ADC_SSMUX0_MUX6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_SSMUX0_MUX6`"]
pub struct ADC_SSMUX0_MUX6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSMUX0_MUX6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSMUX0_MUX7`"]
pub type ADC_SSMUX0_MUX7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_SSMUX0_MUX7`"]
pub struct ADC_SSMUX0_MUX7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSMUX0_MUX7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux0(&self) -> ADC_SSMUX0_MUX0_R {
        ADC_SSMUX0_MUX0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 2nd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux1(&self) -> ADC_SSMUX0_MUX1_R {
        ADC_SSMUX0_MUX1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - 3rd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux2(&self) -> ADC_SSMUX0_MUX2_R {
        ADC_SSMUX0_MUX2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - 4th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux3(&self) -> ADC_SSMUX0_MUX3_R {
        ADC_SSMUX0_MUX3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - 5th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux4(&self) -> ADC_SSMUX0_MUX4_R {
        ADC_SSMUX0_MUX4_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - 6th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux5(&self) -> ADC_SSMUX0_MUX5_R {
        ADC_SSMUX0_MUX5_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - 7th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux6(&self) -> ADC_SSMUX0_MUX6_R {
        ADC_SSMUX0_MUX6_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - 8th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux7(&self) -> ADC_SSMUX0_MUX7_R {
        ADC_SSMUX0_MUX7_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux0(&mut self) -> ADC_SSMUX0_MUX0_W {
        ADC_SSMUX0_MUX0_W { w: self }
    }
    #[doc = "Bits 4:5 - 2nd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux1(&mut self) -> ADC_SSMUX0_MUX1_W {
        ADC_SSMUX0_MUX1_W { w: self }
    }
    #[doc = "Bits 8:9 - 3rd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux2(&mut self) -> ADC_SSMUX0_MUX2_W {
        ADC_SSMUX0_MUX2_W { w: self }
    }
    #[doc = "Bits 12:13 - 4th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux3(&mut self) -> ADC_SSMUX0_MUX3_W {
        ADC_SSMUX0_MUX3_W { w: self }
    }
    #[doc = "Bits 16:17 - 5th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux4(&mut self) -> ADC_SSMUX0_MUX4_W {
        ADC_SSMUX0_MUX4_W { w: self }
    }
    #[doc = "Bits 20:21 - 6th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux5(&mut self) -> ADC_SSMUX0_MUX5_W {
        ADC_SSMUX0_MUX5_W { w: self }
    }
    #[doc = "Bits 24:25 - 7th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux6(&mut self) -> ADC_SSMUX0_MUX6_W {
        ADC_SSMUX0_MUX6_W { w: self }
    }
    #[doc = "Bits 28:29 - 8th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux7(&mut self) -> ADC_SSMUX0_MUX7_W {
        ADC_SSMUX0_MUX7_W { w: self }
    }
}
