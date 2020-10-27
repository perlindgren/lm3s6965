#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SSI_ICR_RORIC`"]
pub struct SSI_ICR_RORIC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_ICR_RORIC_W<'a> {
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
#[doc = "Write proxy for field `SSI_ICR_RTIC`"]
pub struct SSI_ICR_RTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_ICR_RTIC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - SSI Receive Overrun Interrupt Clear"]
    #[inline(always)]
    pub fn ssi_icr_roric(&mut self) -> SSI_ICR_RORIC_W {
        SSI_ICR_RORIC_W { w: self }
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Interrupt Clear"]
    #[inline(always)]
    pub fn ssi_icr_rtic(&mut self) -> SSI_ICR_RTIC_W {
        SSI_ICR_RTIC_W { w: self }
    }
}
