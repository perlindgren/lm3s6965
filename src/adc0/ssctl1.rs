#[doc = "Reader of register SSCTL1"]
pub type R = crate::R<u32, super::SSCTL1>;
#[doc = "Writer for register SSCTL1"]
pub type W = crate::W<u32, super::SSCTL1>;
#[doc = "Register SSCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSCTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_SSCTL1_D0`"]
pub type ADC_SSCTL1_D0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_D0`"]
pub struct ADC_SSCTL1_D0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_D0_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_END0`"]
pub type ADC_SSCTL1_END0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_END0`"]
pub struct ADC_SSCTL1_END0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_END0_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_IE0`"]
pub type ADC_SSCTL1_IE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_IE0`"]
pub struct ADC_SSCTL1_IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_IE0_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_TS0`"]
pub type ADC_SSCTL1_TS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_TS0`"]
pub struct ADC_SSCTL1_TS0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_TS0_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_D1`"]
pub type ADC_SSCTL1_D1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_D1`"]
pub struct ADC_SSCTL1_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_D1_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_END1`"]
pub type ADC_SSCTL1_END1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_END1`"]
pub struct ADC_SSCTL1_END1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_END1_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_IE1`"]
pub type ADC_SSCTL1_IE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_IE1`"]
pub struct ADC_SSCTL1_IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_IE1_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_TS1`"]
pub type ADC_SSCTL1_TS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_TS1`"]
pub struct ADC_SSCTL1_TS1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_TS1_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_D2`"]
pub type ADC_SSCTL1_D2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_D2`"]
pub struct ADC_SSCTL1_D2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_D2_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_END2`"]
pub type ADC_SSCTL1_END2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_END2`"]
pub struct ADC_SSCTL1_END2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_END2_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_IE2`"]
pub type ADC_SSCTL1_IE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_IE2`"]
pub struct ADC_SSCTL1_IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_IE2_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_TS2`"]
pub type ADC_SSCTL1_TS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_TS2`"]
pub struct ADC_SSCTL1_TS2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_TS2_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_D3`"]
pub type ADC_SSCTL1_D3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_D3`"]
pub struct ADC_SSCTL1_D3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_D3_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_END3`"]
pub type ADC_SSCTL1_END3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_END3`"]
pub struct ADC_SSCTL1_END3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_END3_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_IE3`"]
pub type ADC_SSCTL1_IE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_IE3`"]
pub struct ADC_SSCTL1_IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_IE3_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL1_TS3`"]
pub type ADC_SSCTL1_TS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL1_TS3`"]
pub struct ADC_SSCTL1_TS3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL1_TS3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - 1st Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl1_d0(&self) -> ADC_SSCTL1_D0_R {
        ADC_SSCTL1_D0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl1_end0(&self) -> ADC_SSCTL1_END0_R {
        ADC_SSCTL1_END0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl1_ie0(&self) -> ADC_SSCTL1_IE0_R {
        ADC_SSCTL1_IE0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl1_ts0(&self) -> ADC_SSCTL1_TS0_R {
        ADC_SSCTL1_TS0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 2nd Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl1_d1(&self) -> ADC_SSCTL1_D1_R {
        ADC_SSCTL1_D1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl1_end1(&self) -> ADC_SSCTL1_END1_R {
        ADC_SSCTL1_END1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl1_ie1(&self) -> ADC_SSCTL1_IE1_R {
        ADC_SSCTL1_IE1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl1_ts1(&self) -> ADC_SSCTL1_TS1_R {
        ADC_SSCTL1_TS1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 3rd Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl1_d2(&self) -> ADC_SSCTL1_D2_R {
        ADC_SSCTL1_D2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl1_end2(&self) -> ADC_SSCTL1_END2_R {
        ADC_SSCTL1_END2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl1_ie2(&self) -> ADC_SSCTL1_IE2_R {
        ADC_SSCTL1_IE2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl1_ts2(&self) -> ADC_SSCTL1_TS2_R {
        ADC_SSCTL1_TS2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 4th Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl1_d3(&self) -> ADC_SSCTL1_D3_R {
        ADC_SSCTL1_D3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl1_end3(&self) -> ADC_SSCTL1_END3_R {
        ADC_SSCTL1_END3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl1_ie3(&self) -> ADC_SSCTL1_IE3_R {
        ADC_SSCTL1_IE3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl1_ts3(&self) -> ADC_SSCTL1_TS3_R {
        ADC_SSCTL1_TS3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl1_d0(&mut self) -> ADC_SSCTL1_D0_W {
        ADC_SSCTL1_D0_W { w: self }
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl1_end0(&mut self) -> ADC_SSCTL1_END0_W {
        ADC_SSCTL1_END0_W { w: self }
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl1_ie0(&mut self) -> ADC_SSCTL1_IE0_W {
        ADC_SSCTL1_IE0_W { w: self }
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl1_ts0(&mut self) -> ADC_SSCTL1_TS0_W {
        ADC_SSCTL1_TS0_W { w: self }
    }
    #[doc = "Bit 4 - 2nd Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl1_d1(&mut self) -> ADC_SSCTL1_D1_W {
        ADC_SSCTL1_D1_W { w: self }
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl1_end1(&mut self) -> ADC_SSCTL1_END1_W {
        ADC_SSCTL1_END1_W { w: self }
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl1_ie1(&mut self) -> ADC_SSCTL1_IE1_W {
        ADC_SSCTL1_IE1_W { w: self }
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl1_ts1(&mut self) -> ADC_SSCTL1_TS1_W {
        ADC_SSCTL1_TS1_W { w: self }
    }
    #[doc = "Bit 8 - 3rd Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl1_d2(&mut self) -> ADC_SSCTL1_D2_W {
        ADC_SSCTL1_D2_W { w: self }
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl1_end2(&mut self) -> ADC_SSCTL1_END2_W {
        ADC_SSCTL1_END2_W { w: self }
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl1_ie2(&mut self) -> ADC_SSCTL1_IE2_W {
        ADC_SSCTL1_IE2_W { w: self }
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl1_ts2(&mut self) -> ADC_SSCTL1_TS2_W {
        ADC_SSCTL1_TS2_W { w: self }
    }
    #[doc = "Bit 12 - 4th Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl1_d3(&mut self) -> ADC_SSCTL1_D3_W {
        ADC_SSCTL1_D3_W { w: self }
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl1_end3(&mut self) -> ADC_SSCTL1_END3_W {
        ADC_SSCTL1_END3_W { w: self }
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl1_ie3(&mut self) -> ADC_SSCTL1_IE3_W {
        ADC_SSCTL1_IE3_W { w: self }
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl1_ts3(&mut self) -> ADC_SSCTL1_TS3_W {
        ADC_SSCTL1_TS3_W { w: self }
    }
}
