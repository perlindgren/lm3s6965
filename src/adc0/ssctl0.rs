#[doc = "Reader of register SSCTL0"]
pub type R = crate::R<u32, super::SSCTL0>;
#[doc = "Writer for register SSCTL0"]
pub type W = crate::W<u32, super::SSCTL0>;
#[doc = "Register SSCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSCTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_SSCTL0_D0`"]
pub type ADC_SSCTL0_D0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_D0`"]
pub struct ADC_SSCTL0_D0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_D0_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_END0`"]
pub type ADC_SSCTL0_END0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_END0`"]
pub struct ADC_SSCTL0_END0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_END0_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_IE0`"]
pub type ADC_SSCTL0_IE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_IE0`"]
pub struct ADC_SSCTL0_IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_IE0_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_TS0`"]
pub type ADC_SSCTL0_TS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_TS0`"]
pub struct ADC_SSCTL0_TS0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_TS0_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_D1`"]
pub type ADC_SSCTL0_D1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_D1`"]
pub struct ADC_SSCTL0_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_D1_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_END1`"]
pub type ADC_SSCTL0_END1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_END1`"]
pub struct ADC_SSCTL0_END1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_END1_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_IE1`"]
pub type ADC_SSCTL0_IE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_IE1`"]
pub struct ADC_SSCTL0_IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_IE1_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_TS1`"]
pub type ADC_SSCTL0_TS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_TS1`"]
pub struct ADC_SSCTL0_TS1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_TS1_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_D2`"]
pub type ADC_SSCTL0_D2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_D2`"]
pub struct ADC_SSCTL0_D2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_D2_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_END2`"]
pub type ADC_SSCTL0_END2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_END2`"]
pub struct ADC_SSCTL0_END2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_END2_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_IE2`"]
pub type ADC_SSCTL0_IE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_IE2`"]
pub struct ADC_SSCTL0_IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_IE2_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_TS2`"]
pub type ADC_SSCTL0_TS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_TS2`"]
pub struct ADC_SSCTL0_TS2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_TS2_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_D3`"]
pub type ADC_SSCTL0_D3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_D3`"]
pub struct ADC_SSCTL0_D3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_D3_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_END3`"]
pub type ADC_SSCTL0_END3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_END3`"]
pub struct ADC_SSCTL0_END3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_END3_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_IE3`"]
pub type ADC_SSCTL0_IE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_IE3`"]
pub struct ADC_SSCTL0_IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_IE3_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_TS3`"]
pub type ADC_SSCTL0_TS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_TS3`"]
pub struct ADC_SSCTL0_TS3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_TS3_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_D4`"]
pub type ADC_SSCTL0_D4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_D4`"]
pub struct ADC_SSCTL0_D4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_D4_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_END4`"]
pub type ADC_SSCTL0_END4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_END4`"]
pub struct ADC_SSCTL0_END4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_END4_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_IE4`"]
pub type ADC_SSCTL0_IE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_IE4`"]
pub struct ADC_SSCTL0_IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_IE4_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_TS4`"]
pub type ADC_SSCTL0_TS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_TS4`"]
pub struct ADC_SSCTL0_TS4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_TS4_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_D5`"]
pub type ADC_SSCTL0_D5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_D5`"]
pub struct ADC_SSCTL0_D5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_D5_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_END5`"]
pub type ADC_SSCTL0_END5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_END5`"]
pub struct ADC_SSCTL0_END5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_END5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSCTL0_IE5`"]
pub type ADC_SSCTL0_IE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_IE5`"]
pub struct ADC_SSCTL0_IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_IE5_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_TS5`"]
pub type ADC_SSCTL0_TS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_TS5`"]
pub struct ADC_SSCTL0_TS5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_TS5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSCTL0_D6`"]
pub type ADC_SSCTL0_D6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_D6`"]
pub struct ADC_SSCTL0_D6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_D6_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_END6`"]
pub type ADC_SSCTL0_END6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_END6`"]
pub struct ADC_SSCTL0_END6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_END6_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_IE6`"]
pub type ADC_SSCTL0_IE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_IE6`"]
pub struct ADC_SSCTL0_IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_IE6_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_TS6`"]
pub type ADC_SSCTL0_TS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_TS6`"]
pub struct ADC_SSCTL0_TS6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_TS6_W<'a> {
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
#[doc = "Reader of field `ADC_SSCTL0_D7`"]
pub type ADC_SSCTL0_D7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_D7`"]
pub struct ADC_SSCTL0_D7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_D7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSCTL0_END7`"]
pub type ADC_SSCTL0_END7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_END7`"]
pub struct ADC_SSCTL0_END7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_END7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSCTL0_IE7`"]
pub type ADC_SSCTL0_IE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_IE7`"]
pub struct ADC_SSCTL0_IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_IE7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `ADC_SSCTL0_TS7`"]
pub type ADC_SSCTL0_TS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SSCTL0_TS7`"]
pub struct ADC_SSCTL0_TS7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SSCTL0_TS7_W<'a> {
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
    #[doc = "Bit 0 - 1st Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d0(&self) -> ADC_SSCTL0_D0_R {
        ADC_SSCTL0_D0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end0(&self) -> ADC_SSCTL0_END0_R {
        ADC_SSCTL0_END0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie0(&self) -> ADC_SSCTL0_IE0_R {
        ADC_SSCTL0_IE0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts0(&self) -> ADC_SSCTL0_TS0_R {
        ADC_SSCTL0_TS0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 2nd Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d1(&self) -> ADC_SSCTL0_D1_R {
        ADC_SSCTL0_D1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end1(&self) -> ADC_SSCTL0_END1_R {
        ADC_SSCTL0_END1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie1(&self) -> ADC_SSCTL0_IE1_R {
        ADC_SSCTL0_IE1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts1(&self) -> ADC_SSCTL0_TS1_R {
        ADC_SSCTL0_TS1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 3rd Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d2(&self) -> ADC_SSCTL0_D2_R {
        ADC_SSCTL0_D2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end2(&self) -> ADC_SSCTL0_END2_R {
        ADC_SSCTL0_END2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie2(&self) -> ADC_SSCTL0_IE2_R {
        ADC_SSCTL0_IE2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts2(&self) -> ADC_SSCTL0_TS2_R {
        ADC_SSCTL0_TS2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 4th Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d3(&self) -> ADC_SSCTL0_D3_R {
        ADC_SSCTL0_D3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end3(&self) -> ADC_SSCTL0_END3_R {
        ADC_SSCTL0_END3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie3(&self) -> ADC_SSCTL0_IE3_R {
        ADC_SSCTL0_IE3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts3(&self) -> ADC_SSCTL0_TS3_R {
        ADC_SSCTL0_TS3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 5th Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d4(&self) -> ADC_SSCTL0_D4_R {
        ADC_SSCTL0_D4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 5th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end4(&self) -> ADC_SSCTL0_END4_R {
        ADC_SSCTL0_END4_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 5th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie4(&self) -> ADC_SSCTL0_IE4_R {
        ADC_SSCTL0_IE4_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 5th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts4(&self) -> ADC_SSCTL0_TS4_R {
        ADC_SSCTL0_TS4_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 6th Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d5(&self) -> ADC_SSCTL0_D5_R {
        ADC_SSCTL0_D5_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 6th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end5(&self) -> ADC_SSCTL0_END5_R {
        ADC_SSCTL0_END5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 6th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie5(&self) -> ADC_SSCTL0_IE5_R {
        ADC_SSCTL0_IE5_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 6th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts5(&self) -> ADC_SSCTL0_TS5_R {
        ADC_SSCTL0_TS5_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 7th Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d6(&self) -> ADC_SSCTL0_D6_R {
        ADC_SSCTL0_D6_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 7th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end6(&self) -> ADC_SSCTL0_END6_R {
        ADC_SSCTL0_END6_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 7th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie6(&self) -> ADC_SSCTL0_IE6_R {
        ADC_SSCTL0_IE6_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 7th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts6(&self) -> ADC_SSCTL0_TS6_R {
        ADC_SSCTL0_TS6_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 8th Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d7(&self) -> ADC_SSCTL0_D7_R {
        ADC_SSCTL0_D7_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 8th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end7(&self) -> ADC_SSCTL0_END7_R {
        ADC_SSCTL0_END7_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 8th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie7(&self) -> ADC_SSCTL0_IE7_R {
        ADC_SSCTL0_IE7_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 8th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts7(&self) -> ADC_SSCTL0_TS7_R {
        ADC_SSCTL0_TS7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d0(&mut self) -> ADC_SSCTL0_D0_W {
        ADC_SSCTL0_D0_W { w: self }
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end0(&mut self) -> ADC_SSCTL0_END0_W {
        ADC_SSCTL0_END0_W { w: self }
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie0(&mut self) -> ADC_SSCTL0_IE0_W {
        ADC_SSCTL0_IE0_W { w: self }
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts0(&mut self) -> ADC_SSCTL0_TS0_W {
        ADC_SSCTL0_TS0_W { w: self }
    }
    #[doc = "Bit 4 - 2nd Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d1(&mut self) -> ADC_SSCTL0_D1_W {
        ADC_SSCTL0_D1_W { w: self }
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end1(&mut self) -> ADC_SSCTL0_END1_W {
        ADC_SSCTL0_END1_W { w: self }
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie1(&mut self) -> ADC_SSCTL0_IE1_W {
        ADC_SSCTL0_IE1_W { w: self }
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts1(&mut self) -> ADC_SSCTL0_TS1_W {
        ADC_SSCTL0_TS1_W { w: self }
    }
    #[doc = "Bit 8 - 3rd Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d2(&mut self) -> ADC_SSCTL0_D2_W {
        ADC_SSCTL0_D2_W { w: self }
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end2(&mut self) -> ADC_SSCTL0_END2_W {
        ADC_SSCTL0_END2_W { w: self }
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie2(&mut self) -> ADC_SSCTL0_IE2_W {
        ADC_SSCTL0_IE2_W { w: self }
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts2(&mut self) -> ADC_SSCTL0_TS2_W {
        ADC_SSCTL0_TS2_W { w: self }
    }
    #[doc = "Bit 12 - 4th Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d3(&mut self) -> ADC_SSCTL0_D3_W {
        ADC_SSCTL0_D3_W { w: self }
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end3(&mut self) -> ADC_SSCTL0_END3_W {
        ADC_SSCTL0_END3_W { w: self }
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie3(&mut self) -> ADC_SSCTL0_IE3_W {
        ADC_SSCTL0_IE3_W { w: self }
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts3(&mut self) -> ADC_SSCTL0_TS3_W {
        ADC_SSCTL0_TS3_W { w: self }
    }
    #[doc = "Bit 16 - 5th Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d4(&mut self) -> ADC_SSCTL0_D4_W {
        ADC_SSCTL0_D4_W { w: self }
    }
    #[doc = "Bit 17 - 5th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end4(&mut self) -> ADC_SSCTL0_END4_W {
        ADC_SSCTL0_END4_W { w: self }
    }
    #[doc = "Bit 18 - 5th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie4(&mut self) -> ADC_SSCTL0_IE4_W {
        ADC_SSCTL0_IE4_W { w: self }
    }
    #[doc = "Bit 19 - 5th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts4(&mut self) -> ADC_SSCTL0_TS4_W {
        ADC_SSCTL0_TS4_W { w: self }
    }
    #[doc = "Bit 20 - 6th Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d5(&mut self) -> ADC_SSCTL0_D5_W {
        ADC_SSCTL0_D5_W { w: self }
    }
    #[doc = "Bit 21 - 6th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end5(&mut self) -> ADC_SSCTL0_END5_W {
        ADC_SSCTL0_END5_W { w: self }
    }
    #[doc = "Bit 22 - 6th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie5(&mut self) -> ADC_SSCTL0_IE5_W {
        ADC_SSCTL0_IE5_W { w: self }
    }
    #[doc = "Bit 23 - 6th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts5(&mut self) -> ADC_SSCTL0_TS5_W {
        ADC_SSCTL0_TS5_W { w: self }
    }
    #[doc = "Bit 24 - 7th Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d6(&mut self) -> ADC_SSCTL0_D6_W {
        ADC_SSCTL0_D6_W { w: self }
    }
    #[doc = "Bit 25 - 7th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end6(&mut self) -> ADC_SSCTL0_END6_W {
        ADC_SSCTL0_END6_W { w: self }
    }
    #[doc = "Bit 26 - 7th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie6(&mut self) -> ADC_SSCTL0_IE6_W {
        ADC_SSCTL0_IE6_W { w: self }
    }
    #[doc = "Bit 27 - 7th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts6(&mut self) -> ADC_SSCTL0_TS6_W {
        ADC_SSCTL0_TS6_W { w: self }
    }
    #[doc = "Bit 28 - 8th Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d7(&mut self) -> ADC_SSCTL0_D7_W {
        ADC_SSCTL0_D7_W { w: self }
    }
    #[doc = "Bit 29 - 8th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end7(&mut self) -> ADC_SSCTL0_END7_W {
        ADC_SSCTL0_END7_W { w: self }
    }
    #[doc = "Bit 30 - 8th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie7(&mut self) -> ADC_SSCTL0_IE7_W {
        ADC_SSCTL0_IE7_W { w: self }
    }
    #[doc = "Bit 31 - 8th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts7(&mut self) -> ADC_SSCTL0_TS7_W {
        ADC_SSCTL0_TS7_W { w: self }
    }
}
